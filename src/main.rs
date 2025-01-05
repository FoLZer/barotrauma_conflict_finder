#![feature(hash_set_entry)]

pub mod content_file;
pub mod content_package;
pub mod player_config;
pub mod shared;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use clap::Parser;
use content_package::{AnyContentPackage, ContentPackage, Core, Regular};
use directories::BaseDirs;
use log::{info, trace, warn};
use paste::paste;
use player_config::PlayerConfigFile;
use roxmltree::Document;
use simple_logger::SimpleLogger;

#[derive(Parser)]
struct Args {
    #[arg(default_value = r#"C:\Program Files (x86)\Steam\steamapps\common\Barotrauma"#)]
    game_path: PathBuf,
    config_player_path: Option<PathBuf>,
}

macro_rules! detect_conflict {
    ($item_name: literal, $id_map: ident, $content_file: expr, $overridable_field: ident) => {
        for item_file in &$content_file {
            for item in &item_file.$overridable_field {
                let identifier = &item.value.get_identifier();
                match $id_map.entry(identifier.to_string()) {
                    std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                        if occupied_entry.get().was_overriden {
                            log::error!("Item id {} is already loaded!", identifier);
                            continue;
                        } else {
                            if !item.is_override {
                                log::error!(
                                    "[{}] id {} was already defined and this mod declares it but doesn't override!",
                                    $item_name, identifier
                                );
                                continue;
                            } else {
                                occupied_entry.get_mut().was_overriden = true;
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
                        });
                    }
                }
            }
        }
    };
}

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
                info!(
                    "Loading package {}",
                    package
                        .name()
                        .clone()
                        .unwrap_or_else(|| package.steam_workshop_id().unwrap().to_string())
                );
                $(
                    paste! {
                        detect_conflict!($item_name, [<loaded_ $content_file _ $overridable_field _ids>], content_files.$content_file, $overridable_field);
                    }
                )*
            }
        };
    }

    //TODO: Text conflicts

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
        "Structure Prefabs", structures, prefabs
        //TODO: everything else

    );
}

struct IdCheck {
    pub was_overriden: bool,
}
