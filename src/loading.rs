use directories::BaseDirs;
use iced::futures::{SinkExt, Stream};
use iced::stream::try_channel;
use log::{info, trace, warn};
use paste::paste;
use roxmltree::Document;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::content_package::{AnyContentPackage, ContentPackage, Core, Regular};
use crate::player_config::PlayerConfigFile;

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

#[rustfmt::skip]
macro_rules! detect_conflict_loop {
    (
        $loaded_content_files: ident,
        $( $item_name: literal, $content_file: ident, $overridable_field: ident );*
    ) => {
        $(
            paste! {
                let mut [<loaded_ $content_file _ $overridable_field _ids>]: HashMap::<String, IdCheck> = HashMap::new();
            }
        )*
        for (package, content_files) in &$loaded_content_files {
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

pub fn load(
    game_path: PathBuf,
    config_player_path: PathBuf,
) -> impl Stream<Item = Result<Progress, ()>> {
    try_channel(1, move |mut output| async move {
        if !game_path.exists() {
            log::error!("Game Path folder doesn't exist!");
            return Err(());
        }

        if let Err(e) = std::env::set_current_dir(&game_path) {
            log::error!(
                "Failed to set current working directory to game path folder: {}",
                e
            );
            return Err(());
        }

        if !config_player_path.exists() {
            log::error!(
                "config_player.xml was not found, try checking your game_path argument or provide a custom config_player_path argument"
            );
            return Err(());
        }
        let player_config = {
            let s = match std::fs::read_to_string(config_player_path) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("Failed to read config_player.xml: {}", e);
                    return Err(());
                }
            };

            let doc = match Document::parse(&s) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("Failed to parse config_player.xml: {}", e);
                    return Err(());
                }
            };
            PlayerConfigFile::from_xml(doc.root_element())
        };

        let _ = output.send(Progress::ReadingModList).await;

        info!("Reading all installed workshop mods...");
        let installed_packages = {
            let mut v = Vec::new();

            let workshop_folder_path = match BaseDirs::new() {
                Some(v) => v
                    .data_local_dir()
                    .join("Daedalic Entertainment GmbH/Barotrauma/WorkshopMods/Installed"),
                None => {
                    log::error!("Failed to retrieve home folder (unsupported platform)");
                    return Err(());
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
                    &std::fs::read_to_string(&file_list_path).expect("Failed to read filelist.xml"),
                )
                .expect("Failed to parse filelist.xml");
                v.push((package, path));
            }
            v
        };

        let _ = output.send(Progress::LoadingCoreContent).await;

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

        loaded_content_files.push((AnyContentPackage::Core(core_package), core_package_files));

        let num_mods = player_config.content_packages.regular_packages.len();

        for (i, v) in player_config
            .content_packages
            .regular_packages
            .iter()
            .enumerate()
        {
            let _ = output
                .send(Progress::LoadingMods {
                    i: i + 1,
                    max: num_mods,
                })
                .await;
            let package =
                ContentPackage::<Regular>::load(&std::fs::read_to_string(&v.path).unwrap())
                    .unwrap();
            if Path::new(&v.path).parent().unwrap().join("CSharp").exists() {
                warn!(
                    "C# mod detected: {}, C# mods are not checked by the conflict detector!",
                    package.name.as_ref().unwrap_or(&v.path)
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
                &std::path::absolute(&v.path)
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                &installed_packages,
            );
            loaded_content_files.push((AnyContentPackage::Regular(package), files));
        }

        let _ = output.send(Progress::LoadingConflicts).await;

        info!("Done parsing, starting to detect conflicts...");

        //TODO: Text conflicts
        //TODO: skill_settings conflicts

        detect_conflict_loop!(
            loaded_content_files,
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
        Ok(())
    })
}

#[derive(Debug, Clone)]
pub enum Progress {
    ReadingModList,
    LoadingCoreContent,
    LoadingMods { i: usize, max: usize },
    LoadingConflicts,
    Finished,
}

struct IdCheck {
    pub was_overriden: bool,
    pub added_by: Vec<String>,
}

pub enum LoadingState {
    Started,
    ReadingModList,
    LoadingCoreContent,
    LoadingMods { i: usize, max: usize },
    LoadingConflicts,
    Finished,
}

impl From<Progress> for LoadingState {
    fn from(value: Progress) -> Self {
        match value {
            Progress::ReadingModList => Self::ReadingModList,
            Progress::LoadingCoreContent => Self::LoadingCoreContent,
            Progress::LoadingMods { i, max } => Self::LoadingMods { i, max },
            Progress::LoadingConflicts => Self::LoadingConflicts,
            Progress::Finished => Self::Finished,
        }
    }
}
