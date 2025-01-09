use directories::BaseDirs;
use iced::futures::{SinkExt, Stream};
use iced::stream::try_channel;
use log::{info, trace, warn};
use paste::paste;
use roxmltree::Document;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    content_package::{
        AnyContentPackage, ContentFilePaths, ContentFiles, ContentPackage, Core, Regular,
    },
    player_config::PlayerConfigFile,
};

macro_rules! detect_conflict {
    ($item_name: literal, $id_map: expr, $content_file: expr, $overridable_field: ident, $package_id: ident, $package: ident) => {
        for item_file in &$content_file {
            for item in &item_file.$overridable_field {
                let identifier = &item.value.get_identifier();
                match $id_map.entry(identifier.to_string()) {
                    std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                        if occupied_entry.get().was_overriden {
                            log::error!("[{}] id {} is already loaded!", $item_name, identifier);
                            occupied_entry.get_mut().added_by.push($package.clone());
                            continue;
                        } else {
                            if !item.is_override {
                                log::error!(
                                    "[{}] id {} was already defined and this mod declares it but doesn't override!",
                                    $item_name, identifier
                                );
                                occupied_entry.get_mut().added_by.push($package.clone());

                                continue;
                            } else {
                                let e = occupied_entry.get_mut();
                                e.was_overriden = true;
                                e.added_by.push($package.clone());

                                trace!(
                                    "[{}] id {} is overriden by this mod",
                                    $item_name, identifier
                                );
                            }
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(IdCheck {
                            was_overriden: if (item.is_override) {
                                warn!("[{}] id {} is overriding but nothing under this id was loaded before, is this a mistake in load order?", $item_name, identifier);
                                true
                            } else {
                                false
                            },
                            added_by: vec![$package.clone()]
                        });
                    }
                }
            }
        }
    };
}

macro_rules! detect_conflict_loop {
    (
        $loaded_content_files: ident, $conflicts_struct_name: ident,
        $( $item_name: literal, $content_file: ident, $overridable_field: ident, $field_name: ident );*
    ) => {
        let mut $conflicts_struct_name = Conflicts::default();
        $(
            paste! {
                let mut [<loaded_ $content_file $overridable_field _id>]: HashMap<String, IdCheck> = HashMap::new();
            }
        )*
        for (package, content_files) in &$loaded_content_files {
            let package_id = package.package_id();
            info!(
                "Loading package {}",
                package_id
            );
            $(
                paste! {
                    detect_conflict!($item_name, [<loaded_ $content_file $overridable_field _id>], content_files.$content_file, $overridable_field, package_id, package);
                }
            )*
        }
        log::info!("------Conflicts------");
        $(
            paste! {
                for (id, entry) in [<loaded_ $content_file $overridable_field _id>] {
                    if entry.added_by.len() > 2 {
                        log::error!("{}: {} is defined by: {:?}", $item_name, id, entry.added_by.iter().map(|v| v.package_id()).collect::<Vec<_>>());
                        $conflicts_struct_name.$field_name.insert(id, entry);
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

        loaded_content_files.push((
            Arc::new(AnyContentPackage::Core(core_package)),
            core_package_files,
        ));

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
            loaded_content_files.push((Arc::new(AnyContentPackage::Regular(package)), files));
        }

        let _ = output.send(Progress::LoadingConflicts).await;

        info!("Done parsing, starting to detect conflicts...");

        //TODO: Text conflicts
        //TODO: skill_settings conflicts

        #[rustfmt::skip]
        detect_conflict_loop!(
            loaded_content_files, conflicts,

            "Item",items,items,items_items;
            "Item assembly",item_assemblies,item_assemblies,item_assemblies_item_assemblies;
            "Talents",talents,items,talents_items;
            "NPC Sets",npc_sets,sets,npc_sets_sets;
            "Slideshows",slideshows,slideshows,slideshows_slideshows;
            "Talent Trees",talent_trees,trees,talent_trees_trees;
            "Biomes",level_generation_parameters,biomes,level_generation_parameters_biomes;
            "Level Generation Parameters",level_generation_parameters,level_generation_params,level_generation_parameters_level_generation_params;
            "Ballast Flora",ballast_flora,prefabs,ballast_flora_prefabs;
            "Start Items",start_items,sets,start_items_sets;
            "Level Object Prefabs",level_object_prefabs,prefabs,level_object_prefabs_prefabs;
            //"CPR Settings", afflictions, cpr_settings; TODO: these identifiers are based on file location, not implemented yet
            //"Damage Overlays", afflictions, damage_overlays; TODO: these identifiers are based on file location, not implemented yet
            "Affliction Prefabs",afflictions,affliction_prefabs,afflictions_affliction_prefabs;
            "Random Traitor Event Prefabs",random_events,traitor_event_prefabs,random_events_traitor_event_prefabs;
            "Random Event Prefabs",random_events,event_prefabs,random_events_event_prefabs;
            "Random Event Sprites",random_events,event_sprites,random_events_event_sprites;
            "Random Event Sets",random_events,event_sets,random_events_event_sets;
            "Structure Prefabs",structures,prefabs,structures_prefabs;
            //TODO: ui_styles
            "Upgrade Modules Categories",upgrade_modules,categories,upgrade_modules_categories;
            "Upgrade Modules Prefabs",upgrade_modules,prefabs,upgrade_modules_prefabs;
            "Ruin Generation Parameters",ruin_configs,ruin_generation_params,ruin_configs_ruin_generation_params;
            "Outpost Generation Parameters",outpost_configs,outpost_generation_params,outpost_configs_outpost_generation_params;
            "Wreck AI Configs",wreck_ai_configs,wreck_ai_configs,wreck_ai_configs_wreck_ai_configs;
            //"Map Generation Parameters", map_generation_params, map_generation_params; TODO: these identifiers are based on file location, not implemented yet
            "Cave Generation Parameters",cave_generation_params,cave_generation_params,cave_generation_params_cave_generation_params;
            "Particle Prefabs",particle_prefabs,particle_prefabs,particle_prefabs_particle_prefabs;
            "Event Manager Settings",event_manager_settings,event_manager_settings,event_manager_settings_event_manager_settings;
            "NPC Personality Traits",npc_personality_traits,npc_personality_traits,npc_personality_traits_npc_personality_traits;
            "Item Repair Priorities",jobs,item_repair_priorities,jobs_item_repair_priorities;
            "Jobs",jobs,jobs,jobs_jobs;
            "Corpse Prefabs",corpse_prefabs,corpse_prefabs,corpse_prefabs_corpse_prefabs;
            //"Sound Prefabs", sound_prefabs, sound_prefabs; TODO: not entirely correct identifier yet
            "Damage Sound Prefabs",sound_prefabs,damage_sound_prefabs,sound_prefabs_damage_sound_prefabs;
            "Background Music Prefabs",sound_prefabs,background_music_prefabs,sound_prefabs_background_music_prefabs;
            "GUI Sound Prefabs",sound_prefabs,gui_sound_prefabs,sound_prefabs_gui_sound_prefabs;
            //"Grime Decals Sprites", decal_prefabs, grime_sprites; TODO: based on index in file, not implemented yet
            "Decal Prefabs",decal_prefabs,decal_prefabs,decal_prefabs_decal_prefabs;
            "Location Types",location_types,location_types,location_types_location_types;
            "Mission Prefabs",mission_prefabs,mission_prefabs,mission_prefabs_mission_prefabs;
            "Order Prefabs",order_prefabs,order_prefabs,order_prefabs_order_prefabs;
            "Order Category Icons",order_prefabs,order_category_icons,order_prefabs_order_category_icons;
            "Faction Prefabs",faction_prefabs,faction_prefabs,faction_prefabs_faction_prefabs;
            "Tutorial Prefabs",tutorial_prefabs,tutorial_prefabs,tutorial_prefabs_tutorial_prefabs
        );
        let _ = output
            .send(Progress::Finished(
                Arc::new(loaded_content_files),
                Arc::new(conflicts),
            ))
            .await;
        Ok(())
    })
}

#[derive(Default, Debug)]
pub struct Conflicts {
    items_items: HashMap<String, IdCheck>,
    item_assemblies_item_assemblies: HashMap<String, IdCheck>,
    talents_items: HashMap<String, IdCheck>,
    npc_sets_sets: HashMap<String, IdCheck>,
    slideshows_slideshows: HashMap<String, IdCheck>,
    talent_trees_trees: HashMap<String, IdCheck>,
    level_generation_parameters_biomes: HashMap<String, IdCheck>,
    level_generation_parameters_level_generation_params: HashMap<String, IdCheck>,
    ballast_flora_prefabs: HashMap<String, IdCheck>,
    start_items_sets: HashMap<String, IdCheck>,
    level_object_prefabs_prefabs: HashMap<String, IdCheck>,
    afflictions_affliction_prefabs: HashMap<String, IdCheck>,
    random_events_traitor_event_prefabs: HashMap<String, IdCheck>,
    random_events_event_prefabs: HashMap<String, IdCheck>,
    random_events_event_sprites: HashMap<String, IdCheck>,
    random_events_event_sets: HashMap<String, IdCheck>,
    structures_prefabs: HashMap<String, IdCheck>,
    upgrade_modules_categories: HashMap<String, IdCheck>,
    upgrade_modules_prefabs: HashMap<String, IdCheck>,
    ruin_configs_ruin_generation_params: HashMap<String, IdCheck>,
    outpost_configs_outpost_generation_params: HashMap<String, IdCheck>,
    wreck_ai_configs_wreck_ai_configs: HashMap<String, IdCheck>,
    cave_generation_params_cave_generation_params: HashMap<String, IdCheck>,
    particle_prefabs_particle_prefabs: HashMap<String, IdCheck>,
    event_manager_settings_event_manager_settings: HashMap<String, IdCheck>,
    npc_personality_traits_npc_personality_traits: HashMap<String, IdCheck>,
    jobs_item_repair_priorities: HashMap<String, IdCheck>,
    jobs_jobs: HashMap<String, IdCheck>,
    corpse_prefabs_corpse_prefabs: HashMap<String, IdCheck>,
    sound_prefabs_damage_sound_prefabs: HashMap<String, IdCheck>,
    sound_prefabs_background_music_prefabs: HashMap<String, IdCheck>,
    sound_prefabs_gui_sound_prefabs: HashMap<String, IdCheck>,
    decal_prefabs_decal_prefabs: HashMap<String, IdCheck>,
    location_types_location_types: HashMap<String, IdCheck>,
    mission_prefabs_mission_prefabs: HashMap<String, IdCheck>,
    order_prefabs_order_prefabs: HashMap<String, IdCheck>,
    order_prefabs_order_category_icons: HashMap<String, IdCheck>,
    faction_prefabs_faction_prefabs: HashMap<String, IdCheck>,
    tutorial_prefabs_tutorial_prefabs: HashMap<String, IdCheck>,
}

macro_rules! build_conflict_type_enum {
    (
        $( $item_name: ident, $display_name: literal, $content_file: ident, $overridable_field: ident, $field_name: ident, $prefab_name: literal );*
    ) => {
        #[derive(EnumIter, Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
        pub enum ConflictType {
            $(
                $item_name,
            )*
        }

        impl ConflictType {
            pub fn get_conflict_by_type<'a>(&self, conflicts: &'a Conflicts) -> &'a HashMap<String, IdCheck> {
                match self {
                    $(
                        Self::$item_name => &conflicts.$field_name,
                    )*
                }
            }

            /// Currently implemented as a slow lookup
            pub fn get_conflict_file_by_type<'a>(&self, files: &'a ContentFiles, item_identifier: &str) -> Option<&'a String> {
                match self {
                    $(
                        Self::$item_name => files.$content_file.iter().find(|v| v.$overridable_field.iter().any(|v| v.value.get_identifier().to_string() == item_identifier)).map(|v| &v.file_path),
                    )*
                }
            }

            pub fn get_mut_conflict_file_paths_by_type<'a>(&self, file_paths: &'a mut ContentFilePaths,) -> &'a mut Vec<String> {
                match self {
                    $(
                        Self::$item_name => &mut file_paths.$content_file,
                    )*
                }
            }

            //probably shouldn't be here but it's convenient
            pub fn get_prefab_name(&self) -> &'static str {
                match self {
                    $(
                        Self::$item_name => $prefab_name,
                    )*
                }
            }
        }

        impl Display for ConflictType {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
                write!(f, "{}", match self {
                    $(
                        Self::$item_name => $display_name,
                    )*
                })
            }
        }
    };
}

build_conflict_type_enum!(
    Item,"Item",items,items,items_items, "Items";
    ItemAssembly,"Item assembly",item_assemblies,item_assemblies,item_assemblies_item_assemblies, "ItemAssemblies";
    Talents,"Talents",talents,items,talents_items, "Talents";
    NPCSets,"NPC Sets",npc_sets,sets,npc_sets_sets, "NpcSets";
    Slideshows,"Slideshows",slideshows,slideshows,slideshows_slideshows, "Slideshows";
    TalentTrees,"Talent Trees",talent_trees,trees,talent_trees_trees, "TalentTrees";
    Biomes,"Biomes",level_generation_parameters,biomes,level_generation_parameters_biomes, "Biomes";
    LevelGenerationParameters,"Level Generation Parameters",level_generation_parameters,level_generation_params,level_generation_parameters_level_generation_params, "LevelGenerationParameters";
    BallastFlora,"Ballast Flora",ballast_flora,prefabs,ballast_flora_prefabs, "BallastFloraBehaviors";
    StartItems,"Start Items",start_items,sets,start_items_sets, "StartItems";
    LevelObjectPrefabs,"Level Object Prefabs",level_object_prefabs,prefabs,level_object_prefabs_prefabs, "LevelObjects";
    AfflictionPrefabs,"Affliction Prefabs",afflictions,affliction_prefabs,afflictions_affliction_prefabs, "Afflictions";
    RandomTraitorEventPrefabs,"Random Traitor Event Prefabs",random_events,traitor_event_prefabs,random_events_traitor_event_prefabs, "EventPrefabs";
    RandomEventPrefabs,"Random Event Prefabs",random_events,event_prefabs,random_events_event_prefabs, "EventPrefabs";
    RandomEventSprites,"Random Event Sprites",random_events,event_sprites,random_events_event_sprites, "EventSprites";
    RandomEventSets,"Random Event Sets",random_events,event_sets,random_events_event_sets, "EventSet";
    StructurePrefabs,"Structure Prefabs",structures,prefabs,structures_prefabs, "Structures";
    UpgradeModulesCategories,"Upgrade Modules Categories",upgrade_modules,categories,upgrade_modules_categories, "UpgradeCategory";
    UpgradeModulesPrefabs,"Upgrade Modules Prefabs",upgrade_modules,prefabs,upgrade_modules_prefabs, "UpgradeModules";
    RuinGenerationParameters,"Ruin Generation Parameters",ruin_configs,ruin_generation_params,ruin_configs_ruin_generation_params, "RuinGenerationParameters";
    OutpostGenerationParameters,"Outpost Generation Parameters",outpost_configs,outpost_generation_params,outpost_configs_outpost_generation_params, "OutpostGenerationParameters";
    WreckAIConfigs,"Wreck AI Configs",wreck_ai_configs,wreck_ai_configs,wreck_ai_configs_wreck_ai_configs, "WreckAIConfigs";
    CaveGenerationParameters,"Cave Generation Parameters",cave_generation_params,cave_generation_params,cave_generation_params_cave_generation_params, "CaveGenerationParameters";
    ParticlePrefabs,"Particle Prefabs",particle_prefabs,particle_prefabs,particle_prefabs_particle_prefabs, "Particles";
    EventManagerSettings,"Event Manager Settings",event_manager_settings,event_manager_settings,event_manager_settings_event_manager_settings, "EventManagerSettings";
    NPCPersonalityTraits,"NPC Personality Traits",npc_personality_traits,npc_personality_traits,npc_personality_traits_npc_personality_traits, "PersonalityTraits";
    ItemRepairPriorities,"Item Repair Priorities",jobs,item_repair_priorities,jobs_item_repair_priorities, "ItemRepairPriorities";
    Jobs,"Jobs",jobs,jobs,jobs_jobs, "Job";
    CorpsePrefabs,"Corpse Prefabs",corpse_prefabs,corpse_prefabs,corpse_prefabs_corpse_prefabs, "Corpses";
    DamageSoundPrefabs,"Damage Sound Prefabs",sound_prefabs,damage_sound_prefabs,sound_prefabs_damage_sound_prefabs, "DamageSound";
    BackgroundMusicPrefabs,"Background Music Prefabs",sound_prefabs,background_music_prefabs,sound_prefabs_background_music_prefabs, "Music";
    GUISoundPrefabs,"GUI Sound Prefabs",sound_prefabs,gui_sound_prefabs,sound_prefabs_gui_sound_prefabs, "GUISound";
    DecalPrefabs,"Decal Prefabs",decal_prefabs,decal_prefabs,decal_prefabs_decal_prefabs, "Decal";
    LocationTypes,"Location Types",location_types,location_types,location_types_location_types, "LocationTypes";
    MissionPrefabs,"Mission Prefabs",mission_prefabs,mission_prefabs,mission_prefabs_mission_prefabs, "Missions";
    OrderPrefabs,"Order Prefabs",order_prefabs,order_prefabs,order_prefabs_order_prefabs, "Orders";
    OrderCategoryIcons,"Order Category Icons",order_prefabs,order_category_icons,order_prefabs_order_category_icons, "OrderCategoryIcon";
    FactionPrefabs,"Faction Prefabs",faction_prefabs,faction_prefabs,faction_prefabs_faction_prefabs, "Factions";
    TutorialPrefabs,"Tutorial Prefabs",tutorial_prefabs,tutorial_prefabs,tutorial_prefabs_tutorial_prefabs, "Tutorials"
);

#[derive(Debug, Clone)]
pub enum Progress {
    ReadingModList,
    LoadingCoreContent,
    LoadingMods {
        i: usize,
        max: usize,
    },
    LoadingConflicts,
    Finished(
        Arc<Vec<(Arc<AnyContentPackage>, ContentFiles)>>,
        Arc<Conflicts>,
    ),
}

#[derive(Debug)]
pub struct IdCheck {
    pub was_overriden: bool,
    pub added_by: Vec<Arc<AnyContentPackage>>,
}

pub enum LoadingState {
    Started,
    ReadingModList,
    LoadingCoreContent,
    LoadingMods {
        i: usize,
        max: usize,
    },
    LoadingConflicts,
    Finished(
        Arc<Vec<(Arc<AnyContentPackage>, ContentFiles)>>,
        Arc<Conflicts>,
    ),
}

impl From<Progress> for LoadingState {
    fn from(value: Progress) -> Self {
        match value {
            Progress::ReadingModList => Self::ReadingModList,
            Progress::LoadingCoreContent => Self::LoadingCoreContent,
            Progress::LoadingMods { i, max } => Self::LoadingMods { i, max },
            Progress::LoadingConflicts => Self::LoadingConflicts,
            Progress::Finished(content_files, conflicts) => {
                Self::Finished(content_files, conflicts)
            }
        }
    }
}
