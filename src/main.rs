pub mod content_file;
pub mod content_package;
pub mod logger;
pub mod player_config;
pub mod shared;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use clap::Parser;
use content_package::{AnyContentPackage, ContentPackage, Core, Regular};
use directories::BaseDirs;
use iced::{
    Element, Subscription, Task,
    futures::{SinkExt, Stream, StreamExt, channel::mpsc::UnboundedReceiver, lock::Mutex},
    stream,
    widget::{button, column, container, radio, row, text, text_editor, text_input},
};
use log::{info, trace, warn};
use logger::SimpleLogger;
use paste::paste;
use player_config::PlayerConfigFile;
use roxmltree::Document;

#[derive(Parser)]
struct Args {
    #[arg(default_value = r#"C:\Program Files (x86)\Steam\steamapps\common\Barotrauma"#)]
    game_path: String,
    config_player_path: Option<String>,
}

impl Args {
    pub fn config_player_path(&self) -> PathBuf {
        self.config_player_path
            .as_ref()
            .map(|v| PathBuf::from(v.clone()))
            .unwrap_or_else(|| PathBuf::from(self.game_path.clone()).join("config_player.xml"))
    }
}

impl Default for Args {
    fn default() -> Self {
        Args::parse()
    }
}

macro_rules! detect_conflict {
    ($item_name: literal, $id_map: ident, $content_file: expr, $overridable_field: ident, $package_id: ident) => {
        for item_file in &$content_file {
            for item in &item_file.$overridable_field {
                let identifier = &item.value.get_identifier();
                match $id_map.entry(identifier.to_string()) {
                    std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                        if occupied_entry.get().was_overriden {
                            log::error!("Item id {} is already loaded!", identifier);
                            occupied_entry.get_mut().added_by.push($package_id.clone());
                            continue;
                        } else {
                            if !item.is_override {
                                log::error!(
                                    "[{}] id {} was already defined and this mod declares it but doesn't override!",
                                    $item_name, identifier
                                );
                                occupied_entry.get_mut().added_by.push($package_id.clone());

                                continue;
                            } else {
                                let e = occupied_entry.get_mut();
                                e.was_overriden = true;
                                e.added_by.push($package_id.clone());

                                trace!(
                                    "[{}] id {} is overriden by this mod",
                                    $item_name, identifier
                                );
                            }
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(IdCheck {
                            was_overriden: false,
                            added_by: vec![$package_id.clone()]
                        });
                    }
                }
            }
        }
    };
}

#[derive(Debug, Clone)]
pub enum Message {
    ScreenChanged(Screen),
    GamePathChanged(String),
    ConfigPathChanged(String),
    StartParsing,
    LogMessage(String),
    StartParsing2,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    #[default]
    Main,
    Settings,
    Logs,
}

struct App {
    pub args: Args,
    pub screen: Screen,

    pub logs: text_editor::Content,

    pub logger_rx: Arc<Mutex<UnboundedReceiver<String>>>,
}

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        column![
            //Screen selection
            row![
                radio(
                    "Main",
                    Screen::Main,
                    Some(self.screen),
                    Message::ScreenChanged
                ),
                radio(
                    "Settings",
                    Screen::Settings,
                    Some(self.screen),
                    Message::ScreenChanged
                ),
                radio(
                    "Logs",
                    Screen::Logs,
                    Some(self.screen),
                    Message::ScreenChanged
                )
            ],
            //Main view
            container(match self.screen {
                Screen::Main => {
                    Into::<Element<'_, Message>>::into(column![
                        text!("Game Path: {}", &self.args.game_path),
                        button("Parse").on_press(Message::StartParsing)
                    ])
                }
                Screen::Settings => {
                    column![
                        row![
                            text!("Game Path:"),
                            text_input("", &self.args.game_path).on_input(Message::GamePathChanged)
                        ],
                        row![
                            text!("Config Path (Optional):"),
                            text_input("", self.args.config_player_path.as_ref().map_or("", |v| v))
                                .on_input(Message::ConfigPathChanged)
                        ]
                    ]
                    .into()
                }
                Screen::Logs => {
                    text_editor(&self.logs).into()
                }
            })
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ScreenChanged(screen) => {
                self.screen = screen;
            }
            Message::GamePathChanged(s) => {
                self.args.game_path = s;
            }
            Message::ConfigPathChanged(s) => {
                self.args.config_player_path = if s.len() > 0 { Some(s) } else { None }
            }
            Message::LogMessage(s) => {
                self.logs
                    .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                        Arc::new(format!("{}\n", s)),
                    )));
            }
            Message::StartParsing => {
                return Task::done(Message::ScreenChanged(Screen::Logs))
                    .chain(Task::done(Message::StartParsing2));
            }
            Message::StartParsing2 => {
                let game_path = PathBuf::from(&self.args.game_path);
                let config_player_path = self.args.config_player_path();

                if !game_path.exists() {
                    log::error!("Game Path folder doesn't exist!");
                    return Task::done(Message::ScreenChanged(Screen::Logs));
                }

                if let Err(e) = std::env::set_current_dir(&self.args.game_path) {
                    log::error!(
                        "Failed to set current working directory to game path folder: {}",
                        e
                    );
                    return Task::done(Message::ScreenChanged(Screen::Logs));
                }

                if !config_player_path.exists() {
                    log::error!(
                        "config_player.xml was not found, try checking your game_path argument or provide a custom config_player_path argument"
                    );
                    return Task::done(Message::ScreenChanged(Screen::Logs));
                }
                let player_config = {
                    let s = match std::fs::read_to_string(config_player_path) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("Failed to read config_player.xml: {}", e);
                            return Task::done(Message::ScreenChanged(Screen::Logs));
                        }
                    };

                    let doc = match Document::parse(&s) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("Failed to parse config_player.xml: {}", e);
                            return Task::done(Message::ScreenChanged(Screen::Logs));
                        }
                    };
                    PlayerConfigFile::from_xml(doc.root_element())
                };

                info!("Reading all installed workshop mods...");
                let installed_packages = {
                    let mut v = Vec::new();

                    let workshop_folder_path = match BaseDirs::new() {
                        Some(v) => v
                            .data_local_dir()
                            .join("Daedalic Entertainment GmbH/Barotrauma/WorkshopMods/Installed"),
                        None => {
                            log::error!("Failed to retrieve home folder (unsupported platform)");
                            return Task::done(Message::ScreenChanged(Screen::Logs));
                        }
                    };
                    info!("Workshop folder path: {}", workshop_folder_path.display());
                    for entry in std::fs::read_dir(workshop_folder_path)
                        .expect("Failed to read mods folder (probably doesn't exist)")
                    {
                        let entry = entry.unwrap();
                        let path = entry.path();
                        if !path.is_dir() {
                            continue;
                        }
                        let file_list_path = path.join("filelist.xml");
                        let package = ContentPackage::<Regular>::load(
                            &std::fs::read_to_string(&file_list_path)
                                .expect("Failed to read filelist.xml"),
                        )
                        .expect("Failed to parse filelist.xml");
                        v.push((package, path));
                    }
                    v
                };

                info!("Parsing core content package...");

                let mut loaded_content_files = Vec::new();

                dbg!(&player_config.content_packages.core_package.path);

                let core_package = ContentPackage::<Core>::load(
                    &std::fs::read_to_string(&player_config.content_packages.core_package.path)
                        .expect("Failed to read filelist.xml"),
                )
                .expect("Failed to parse filelist.xml");
                let core_package_files = core_package.load_file_list(
                    &std::path::absolute(&player_config.content_packages.core_package.path)
                        .unwrap()
                        .parent()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                    &installed_packages,
                );

                loaded_content_files
                    .push((AnyContentPackage::Core(core_package), core_package_files));

                for v in player_config.content_packages.regular_packages {
                    let package =
                        ContentPackage::<Regular>::load(&std::fs::read_to_string(&v.path).unwrap())
                            .unwrap();
                    if Path::new(&v.path).parent().unwrap().join("CSharp").exists() {
                        warn!(
                            "C# mod detected: {}, C# mods are not checked by the conflict detector!",
                            package.name.unwrap_or(v.path)
                        );
                        continue;
                    }
                    info!(
                        "Parsing {}...",
                        package
                            .name
                            .as_ref()
                            .unwrap_or(&package.steam_workshop_id.unwrap().to_string())
                    );
                    let files = package.load_file_list(
                        &std::path::absolute(v.path)
                            .unwrap()
                            .parent()
                            .unwrap()
                            .to_str()
                            .unwrap(),
                        &installed_packages,
                    );
                    loaded_content_files.push((AnyContentPackage::Regular(package), files));
                }

                info!("Done parsing, starting to detect conflicts...");

                #[rustfmt::skip]
    macro_rules! detect_conflict_loop {
        (
            $( $item_name: literal, $content_file: ident, $overridable_field: ident );*
        ) => {
            $(
                paste! {
                    let mut [<loaded_ $content_file _ $overridable_field _ids>]: HashMap::<String, IdCheck> = HashMap::new();
                }
            )*
            for (package, content_files) in &loaded_content_files {
                let package_id = package
                    .name()
                    .clone()
                    .unwrap_or_else(|| package.steam_workshop_id().unwrap().to_string());
                info!(
                    "Loading package {}",
                    package_id
                );
                $(
                    paste! {
                        detect_conflict!($item_name, [<loaded_ $content_file _ $overridable_field _ids>], content_files.$content_file, $overridable_field, package_id);
                    }
                )*
            }
            log::info!("------Conflicts------");
            $(
                paste! {
                    for (id, entry) in &[<loaded_ $content_file _ $overridable_field _ids>] {
                        if entry.added_by.len() > 2 {
                            log::error!("{}: {} is defined by: {:?}", $item_name, id, entry.added_by);
                        }
                    }
                }
            )*
        };
    }

                //TODO: Text conflicts
                //TODO: skill_settings conflicts

                detect_conflict_loop!(
                    "Item", items, items;
                    "Item assembly", item_assemblies, item_assemblies;
                    "Talents", talents, items;
                    "NPC Sets", npc_sets, sets;
                    "Slideshows", slideshows, slideshows;
                    "Talent Trees", talent_trees, trees;
                    "Biomes", level_generation_parameters, biomes;
                    "Level Generation Parameters", level_generation_parameters, level_generation_params;
                    "Ballast Flora", ballast_flora, prefabs;
                    "Start Items", start_items, sets;
                    "Level Object Prefabs", level_object_prefabs, prefabs;
                    //"CPR Settings", afflictions, cpr_settings; TODO: these identifiers are based on file location, not implemented yet
                    //"Damage Overlays", afflictions, damage_overlays; TODO: these identifiers are based on file location, not implemented yet
                    "Affliction Prefabs", afflictions, affliction_prefabs;
                    "Random Traitor Event Prefabs", random_events, traitor_event_prefabs;
                    "Random Event Prefabs", random_events, event_prefabs;
                    "Random Event Sprites", random_events, event_sprites;
                    "Random Event Sets", random_events, event_sets;
                    "Structure Prefabs", structures, prefabs;
                    //TODO: ui_styles
                    "Upgrade Modules Categories", upgrade_modules, categories;
                    "Upgrade Modules Prefabs", upgrade_modules, prefabs;
                    "Ruin Generation Parameters", ruin_configs, ruin_generation_params;
                    "Outpost Generation Parameters", outpost_configs, outpost_generation_params;
                    "Wreck AI Configs", wreck_ai_configs, wreck_ai_configs;
                    //"Map Generation Parameters", map_generation_params, map_generation_params; TODO: these identifiers are based on file location, not implemented yet
                    "Cave Generation Parameters", cave_generation_params, cave_generation_params;
                    "Particle Prefabs", particle_prefabs, particle_prefabs;
                    "Event Manager Settings", event_manager_settings, event_manager_settings;
                    "NPC Personality Traits", npc_personality_traits, npc_personality_traits;
                    "Item Repair Priorities", jobs, item_repair_priorities;
                    "Jobs", jobs, jobs;
                    "Corpse Prefabs", corpse_prefabs, corpse_prefabs;
                    //"Sound Prefabs", sound_prefabs, sound_prefabs; TODO: not entirely correct identifier yet
                    "Damage Sound Prefabs", sound_prefabs, damage_sound_prefabs;
                    "Background Music Prefabs", sound_prefabs, background_music_prefabs;
                    "GUI Sound Prefabs", sound_prefabs, gui_sound_prefabs;
                    //"Grime Decals Sprites", decal_prefabs, grime_sprites; TODO: based on index in file, not implemented yet
                    "Decal Prefabs", decal_prefabs, decal_prefabs;
                    "Location Types", location_types, location_types;
                    "Mission Prefabs", mission_prefabs, mission_prefabs;
                    "Order Prefabs", order_prefabs, order_prefabs;
                    "Order Category Icons", order_prefabs, order_category_icons;
                    "Faction Prefabs", faction_prefabs, faction_prefabs;
                    "Tutorial Prefabs", tutorial_prefabs, tutorial_prefabs
                );
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        #[derive(Hash)]
        struct Sub;

        Subscription::run_with_id(Sub, log_worker(self.logger_rx.clone()))
    }
}

fn log_worker(rx: Arc<Mutex<UnboundedReceiver<String>>>) -> impl Stream<Item = Message> {
    stream::channel(5, |mut output| async move {
        let mut l = rx.lock().await;
        loop {
            let m = l.select_next_some().await;
            output.send(Message::LogMessage(m)).await.unwrap();
        }
    })
}

fn main() -> iced::Result {
    let logger = SimpleLogger::new();
    let logger_rx = logger.rx.clone();
    logger.init().unwrap();
    iced::application("Barotrauma Conflict Finder", App::update, App::view)
        .subscription(App::subscription)
        .run_with(|| {
            let state = App {
                args: Default::default(),
                screen: Default::default(),
                logs: Default::default(),
                logger_rx,
            };
            (state, Task::none())
        })
}

struct IdCheck {
    pub was_overriden: bool,
    pub added_by: Vec<String>,
}

/*
fn main() {
    SimpleLogger::new().init().unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let args = Args::parse();
    let config_player_path = args
        .config_player_path
        .unwrap_or_else(|| args.game_path.join("config_player.xml"));

    std::env::set_current_dir(&args.game_path).unwrap();

    if !config_player_path.exists() {
        log::error!(
            "config_player.xml was not found, try checking your game_path argument or provide a custom config_player_path argument"
        );
        return;
    }

    let player_config = PlayerConfigFile::from_xml(
        Document::parse(
            &std::fs::read_to_string(config_player_path).expect("Failed to read config_player.xml"),
        )
        .expect("Failed to parse config_player.xml")
        .root_element(),
    );

    info!("Reading all installed workshop mods...");
    let installed_packages = {
        let mut v = Vec::new();
        let workshop_folder_path = BaseDirs::new()
            .expect("Failed to retrieve mods folder")
            .data_local_dir()
            .join("Daedalic Entertainment GmbH/Barotrauma/WorkshopMods/Installed");
        info!("Workshop folder path: {}", workshop_folder_path.display());
        for entry in std::fs::read_dir(workshop_folder_path)
            .expect("Failed to read mods folder (probably doesn't exist)")
        {
            let entry = entry.unwrap();
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let file_list_path = path.join("filelist.xml");
            let package = ContentPackage::<Regular>::load(
                &std::fs::read_to_string(&file_list_path).expect("Failed to read filelist.xml"),
            )
            .expect("Failed to parse filelist.xml");
            v.push((package, path));
        }
        v
    };

    info!("Parsing core content package...");

    let mut loaded_content_files = Vec::new();

    let core_package = ContentPackage::<Core>::load(
        &std::fs::read_to_string(&player_config.content_packages.core_package.path)
            .expect("Failed to read filelist.xml"),
    )
    .expect("Failed to parse filelist.xml");
    let core_package_files = core_package.load_file_list(
        &std::path::absolute(&player_config.content_packages.core_package.path)
            .unwrap()
            .parent()
            .unwrap()
            .to_str()
            .unwrap(),
        &installed_packages,
    );

    loaded_content_files.push((AnyContentPackage::Core(core_package), core_package_files));

    for v in player_config.content_packages.regular_packages {
        let package =
            ContentPackage::<Regular>::load(&std::fs::read_to_string(&v.path).unwrap()).unwrap();
        if Path::new(&v.path).parent().unwrap().join("CSharp").exists() {
            warn!(
                "C# mod detected: {}, C# mods are not checked by the conflict detector!",
                package.name.unwrap_or(v.path)
            );
            continue;
        }
        info!(
            "Parsing {}...",
            package
                .name
                .as_ref()
                .unwrap_or(&package.steam_workshop_id.unwrap().to_string())
        );
        let files = package.load_file_list(
            &std::path::absolute(v.path)
                .unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap(),
            &installed_packages,
        );
        loaded_content_files.push((AnyContentPackage::Regular(package), files));
    }

    info!("Done parsing, starting to detect conflicts...");

    #[rustfmt::skip]
    macro_rules! detect_conflict_loop {
        (
            $( $item_name: literal, $content_file: ident, $overridable_field: ident );*
        ) => {
            $(
                paste! {
                    let mut [<loaded_ $content_file _ $overridable_field _ids>]: HashMap::<String, IdCheck> = HashMap::new();
                }
            )*
            for (package, content_files) in &loaded_content_files {
                let package_id = package
                    .name()
                    .clone()
                    .unwrap_or_else(|| package.steam_workshop_id().unwrap().to_string());
                info!(
                    "Loading package {}",
                    package_id
                );
                $(
                    paste! {
                        detect_conflict!($item_name, [<loaded_ $content_file _ $overridable_field _ids>], content_files.$content_file, $overridable_field, package_id);
                    }
                )*
            }
            log::info!("------Conflicts------");
            $(
                paste! {
                    for (id, entry) in &[<loaded_ $content_file _ $overridable_field _ids>] {
                        if entry.added_by.len() > 2 {
                            log::error!("{}: {} is defined by: {:?}", $item_name, id, entry.added_by);
                        }
                    }
                }
            )*
        };
    }

    //TODO: Text conflicts
    //TODO: skill_settings conflicts

    detect_conflict_loop!(
        "Item", items, items;
        "Item assembly", item_assemblies, item_assemblies;
        "Talents", talents, items;
        "NPC Sets", npc_sets, sets;
        "Slideshows", slideshows, slideshows;
        "Talent Trees", talent_trees, trees;
        "Biomes", level_generation_parameters, biomes;
        "Level Generation Parameters", level_generation_parameters, level_generation_params;
        "Ballast Flora", ballast_flora, prefabs;
        "Start Items", start_items, sets;
        "Level Object Prefabs", level_object_prefabs, prefabs;
        //"CPR Settings", afflictions, cpr_settings; TODO: these identifiers are based on file location, not implemented yet
        //"Damage Overlays", afflictions, damage_overlays; TODO: these identifiers are based on file location, not implemented yet
        "Affliction Prefabs", afflictions, affliction_prefabs;
        "Random Traitor Event Prefabs", random_events, traitor_event_prefabs;
        "Random Event Prefabs", random_events, event_prefabs;
        "Random Event Sprites", random_events, event_sprites;
        "Random Event Sets", random_events, event_sets;
        "Structure Prefabs", structures, prefabs;
        //TODO: ui_styles
        "Upgrade Modules Categories", upgrade_modules, categories;
        "Upgrade Modules Prefabs", upgrade_modules, prefabs;
        "Ruin Generation Parameters", ruin_configs, ruin_generation_params;
        "Outpost Generation Parameters", outpost_configs, outpost_generation_params;
        "Wreck AI Configs", wreck_ai_configs, wreck_ai_configs;
        //"Map Generation Parameters", map_generation_params, map_generation_params; TODO: these identifiers are based on file location, not implemented yet
        "Cave Generation Parameters", cave_generation_params, cave_generation_params;
        "Particle Prefabs", particle_prefabs, particle_prefabs;
        "Event Manager Settings", event_manager_settings, event_manager_settings;
        "NPC Personality Traits", npc_personality_traits, npc_personality_traits;
        "Item Repair Priorities", jobs, item_repair_priorities;
        "Jobs", jobs, jobs;
        "Corpse Prefabs", corpse_prefabs, corpse_prefabs;
        //"Sound Prefabs", sound_prefabs, sound_prefabs; TODO: not entirely correct identifier yet
        "Damage Sound Prefabs", sound_prefabs, damage_sound_prefabs;
        "Background Music Prefabs", sound_prefabs, background_music_prefabs;
        "GUI Sound Prefabs", sound_prefabs, gui_sound_prefabs;
        //"Grime Decals Sprites", decal_prefabs, grime_sprites; TODO: based on index in file, not implemented yet
        "Decal Prefabs", decal_prefabs, decal_prefabs;
        "Location Types", location_types, location_types;
        "Mission Prefabs", mission_prefabs, mission_prefabs;
        "Order Prefabs", order_prefabs, order_prefabs;
        "Order Category Icons", order_prefabs, order_category_icons;
        "Faction Prefabs", faction_prefabs, faction_prefabs;
        "Tutorial Prefabs", tutorial_prefabs, tutorial_prefabs
    );
}

struct IdCheck {
    pub was_overriden: bool,
    pub added_by: Vec<String>,
}
*/
