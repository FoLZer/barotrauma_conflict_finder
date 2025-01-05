use std::{marker::PhantomData, path::PathBuf, sync::Arc};

use regex::RegexBuilder;
use roxmltree::{Document, Node};

use crate::{
    content_file::{ContentFile, SubmarineAsset},
    shared::{
        content_files::prelude::*, date_time::SerializableDateTime, util::NodeExp, version::Version,
    },
};

pub struct Core;

pub struct Regular;

pub trait ContentPackageType: Sync + Send {
    const IS_CORE: bool;
}

impl ContentPackageType for Core {
    const IS_CORE: bool = true;
}
impl ContentPackageType for Regular {
    const IS_CORE: bool = false;
}

#[derive(Debug, Default)]
pub struct ContentFilePaths {
    pub items: Vec<String>,
    pub texts: Vec<String>,
    pub submarines: Vec<String>,
    pub outposts: Vec<String>,
    pub outpost_modules: Vec<String>,
    pub wrecks: Vec<String>,
    pub beacon_stations: Vec<String>,
    pub enemy_submarines: Vec<String>,
    pub npc_conversations: Vec<String>,
    pub item_assemblies: Vec<String>,
    pub talents: Vec<String>,
    pub npc_sets: Vec<String>,
    pub characters: Vec<String>,
    pub slideshows: Vec<String>,
    pub talent_trees: Vec<String>,
    pub level_generation_parameters: Vec<String>,
    pub ballast_flora: Vec<String>,
    pub start_items: Vec<String>,
    pub level_object_prefabs: Vec<String>,
    pub afflictions: Vec<String>,
    pub random_events: Vec<String>,
    pub structures: Vec<String>,
    pub ui_styles: Vec<String>,
    pub upgrade_modules: Vec<String>,
    pub ruin_configs: Vec<String>,
    pub outpost_configs: Vec<String>,
    pub wreck_ai_configs: Vec<String>,
    pub map_generation_params: Vec<String>,
    pub cave_generation_params: Vec<String>,
    pub background_creature_prefabs: Vec<String>,
    pub particle_prefabs: Vec<String>,
    pub event_manager_settings: Vec<String>,
    pub npc_personality_traits: Vec<String>,
    pub jobs: Vec<String>,
    pub corpse_prefabs: Vec<String>,
    pub sound_prefabs: Vec<String>,
    pub decal_prefabs: Vec<String>,
    pub location_types: Vec<String>,
    pub mission_prefabs: Vec<String>,
    pub order_prefabs: Vec<String>,
    pub skill_settings: Vec<String>,
    pub faction_prefabs: Vec<String>,
    pub tutorial_prefabs: Vec<String>,
}

#[derive(Debug, Default)]
pub struct ContentFiles {
    pub items: Vec<ContentFile<ItemFile>>,
    pub texts: Vec<ContentFile<TextFile>>,
    pub submarines: Vec<Arc<SubmarineAsset>>,
    pub outposts: Vec<Arc<SubmarineAsset>>,
    pub outpost_modules: Vec<Arc<SubmarineAsset>>,
    pub wrecks: Vec<Arc<SubmarineAsset>>,
    pub beacon_stations: Vec<Arc<SubmarineAsset>>,
    pub enemy_submarines: Vec<Arc<SubmarineAsset>>,
    pub npc_conversations: Vec<ContentFile<NPCConversationFile>>,
    pub item_assemblies: Vec<ContentFile<ItemAssemblyFile>>,
    pub talents: Vec<ContentFile<TalentsFile>>,
    pub npc_sets: Vec<ContentFile<NPCSetsFile>>,
    pub characters: Vec<ContentFile<CharacterFile>>,
    pub slideshows: Vec<ContentFile<SlideshowsFile>>,
    pub talent_trees: Vec<ContentFile<TalentTreesFile>>,
    pub level_generation_parameters: Vec<ContentFile<LevelGenerationParametersFile>>,
    pub ballast_flora: Vec<ContentFile<BallastFloraFile>>,
    pub start_items: Vec<ContentFile<StartItemsFile>>,
    pub level_object_prefabs: Vec<ContentFile<LevelObjectPrefabsFile>>,
    pub afflictions: Vec<ContentFile<AfflictionsFile>>,
    pub random_events: Vec<ContentFile<RandomEventsFile>>,
    pub structures: Vec<ContentFile<StructureFile>>,
    pub ui_styles: Vec<ContentFile<UIStyleFile>>,
    pub upgrade_modules: Vec<ContentFile<UpgradeModulesFile>>,
    pub ruin_configs: Vec<ContentFile<RuinConfigFile>>,
    pub outpost_configs: Vec<ContentFile<OutpostConfigFile>>,
    pub wreck_ai_configs: Vec<ContentFile<WreckAIConfigFile>>,
    pub map_generation_params: Vec<ContentFile<MapGenerationParametersFile>>,
    pub cave_generation_params: Vec<ContentFile<CaveGenerationParamsFile>>,
    pub background_creature_prefabs: Vec<ContentFile<BackgroundCreaturePrefabsFile>>,
    pub particle_prefabs: Vec<ContentFile<ParticlesFile>>,
    pub event_manager_settings: Vec<ContentFile<EventManagerSettingsFile>>,
    pub npc_personality_traits: Vec<ContentFile<NPCPersonalityTraitsFile>>,
    pub jobs: Vec<ContentFile<JobsFile>>,
    pub corpse_prefabs: Vec<ContentFile<CorpsesFile>>,
    pub sound_prefabs: Vec<ContentFile<SoundsFile>>,
    pub decal_prefabs: Vec<ContentFile<DecalsFile>>,
    pub location_types: Vec<ContentFile<LocationTypesFile>>,
    pub mission_prefabs: Vec<ContentFile<MissionsFile>>,
    pub order_prefabs: Vec<ContentFile<OrdersFile>>,
    pub skill_settings: Vec<ContentFile<SkillSettingsFile>>,
    pub faction_prefabs: Vec<ContentFile<FactionsFile>>,
    pub tutorial_prefabs: Vec<ContentFile<TutorialsFile>>,
}

pub enum AnyContentPackage {
    Core(ContentPackage<Core>),
    Regular(ContentPackage<Regular>),
}

impl AnyContentPackage {
    pub fn name(&self) -> &Option<String> {
        match self {
            AnyContentPackage::Core(content_package) => &content_package.name,
            AnyContentPackage::Regular(content_package) => &content_package.name,
        }
    }

    pub fn steam_workshop_id(&self) -> &Option<u64> {
        match self {
            AnyContentPackage::Core(content_package) => &content_package.steam_workshop_id,
            AnyContentPackage::Regular(content_package) => &content_package.steam_workshop_id,
        }
    }
}

pub struct ContentPackage<T: ContentPackageType> {
    //attributes
    pub name: Option<String>,
    pub alt_names: Option<Vec<String>>,
    pub steam_workshop_id: Option<u64>,
    pub game_version: Option<Version>,
    pub mod_version: Option<String>,

    pub install_time: Option<SerializableDateTime>,
    pub expected_hash: Option<String>,

    pub file_paths: ContentFilePaths,

    _phantom: PhantomData<T>,
}

impl<T: ContentPackageType> ContentPackage<T> {
    pub fn load(s: &str) -> Result<Self, roxmltree::Error> {
        let document = Document::parse(&s).unwrap();

        let root = document.root_element();
        let mut name = root
            .attribute_ignore_ascii_case("name")
            .map(|v| v.trim().to_owned());
        let alt_names = root.attribute_ignore_ascii_case("altnames").map(|v| {
            v.split(',')
                .map(|v| v.trim().to_owned())
                .collect::<Vec<_>>()
        });
        if name.is_none() {
            if let Some(altnames) = &alt_names {
                name = altnames.first().cloned();
            }
        }
        let steam_workshop_id = root
            .attribute_ignore_ascii_case("steamworkshopid")
            .map(|v| v.parse::<u64>().unwrap()); //TODO: error handling

        let game_version = root
            .attribute_ignore_ascii_case("gameversion")
            .map(|v| v.parse::<Version>().unwrap()); //TODO: error handling

        let mod_version = root
            .attribute_ignore_ascii_case("modversion")
            .map(std::borrow::ToOwned::to_owned);
        let install_time = root
            .attribute_ignore_ascii_case("installtime")
            .map(|v| v.parse::<SerializableDateTime>().unwrap()); //TODO: error handling;

        let expected_hash = root
            .attribute_ignore_ascii_case("expectedhash")
            .map(|v| v.to_owned());

        let is_core_package = root
            .attribute_ignore_ascii_case("corepackage")
            .map(|v| v.parse::<bool>().unwrap_or(false))
            .unwrap(); //TODO: error handling;
        assert_eq!(is_core_package, T::IS_CORE);

        let mut file_paths = ContentFilePaths::default();

        for element in root.children().filter(Node::is_element) {
            let elem_name = element.tag_name().name();
            let file_path = element
                .attribute_ignore_ascii_case("file")
                .unwrap()
                .to_owned();
            match elem_name {
                "Item" => {
                    file_paths.items.push(file_path);
                }
                "Text" => {
                    file_paths.texts.push(file_path);
                }
                "Submarine" => {
                    file_paths.submarines.push(file_path);
                }
                "Outpost" => {
                    file_paths.outposts.push(file_path);
                }
                "OutpostModule" => {
                    file_paths.outpost_modules.push(file_path);
                }
                "Wreck" => {
                    file_paths.wrecks.push(file_path);
                }
                "BeaconStation" => {
                    file_paths.beacon_stations.push(file_path);
                }
                "EnemySubmarine" => {
                    file_paths.enemy_submarines.push(file_path);
                }
                "NPCConversations" => {
                    file_paths.npc_conversations.push(file_path);
                }
                "ItemAssembly" => {
                    file_paths.item_assemblies.push(file_path);
                }
                "Talents" => {
                    file_paths.talents.push(file_path);
                }
                "NPCSets" => {
                    file_paths.npc_sets.push(file_path);
                }
                "Character" => {
                    file_paths.characters.push(file_path);
                }
                "Slideshows" => {
                    file_paths.slideshows.push(file_path);
                }
                "TalentTrees" => {
                    file_paths.talent_trees.push(file_path);
                }
                "LevelGenerationParameters" => {
                    file_paths.level_generation_parameters.push(file_path);
                }
                "BallastFlora" | "MapCreature" => {
                    file_paths.ballast_flora.push(file_path);
                }
                "StartItems" => {
                    file_paths.start_items.push(file_path);
                }
                "LevelObjectPrefabs" => {
                    file_paths.level_object_prefabs.push(file_path);
                }
                "Afflictions" => {
                    file_paths.afflictions.push(file_path);
                }
                "RandomEvents" => {
                    file_paths.random_events.push(file_path);
                }
                "Structure" => {
                    file_paths.structures.push(file_path);
                }
                "UIStyle" => {
                    file_paths.ui_styles.push(file_path);
                }
                "UpgradeModules" => {
                    file_paths.upgrade_modules.push(file_path);
                }
                "RuinConfig" => {
                    file_paths.ruin_configs.push(file_path);
                }
                "OutpostConfig" => {
                    file_paths.outpost_configs.push(file_path);
                }
                "WreckAIConfig" => {
                    file_paths.wreck_ai_configs.push(file_path);
                }
                "MapGenerationParameters" => {
                    file_paths.map_generation_params.push(file_path);
                }
                "CaveGenerationParameters" => {
                    file_paths.cave_generation_params.push(file_path);
                }
                "BackgroundCreaturePrefabs" => {
                    file_paths.background_creature_prefabs.push(file_path);
                }
                "Particles" => {
                    file_paths.particle_prefabs.push(file_path);
                }
                "EventManagerSettings" => {
                    file_paths.event_manager_settings.push(file_path);
                }
                "NPCPersonalityTraits" => {
                    file_paths.npc_personality_traits.push(file_path);
                }
                "Jobs" => {
                    file_paths.jobs.push(file_path);
                }
                "Corpses" => {
                    file_paths.corpse_prefabs.push(file_path);
                }
                "Sounds" => {
                    file_paths.sound_prefabs.push(file_path);
                }
                "Decals" => {
                    file_paths.decal_prefabs.push(file_path);
                }
                "LocationTypes" => {
                    file_paths.location_types.push(file_path);
                }
                "Missions" => {
                    file_paths.mission_prefabs.push(file_path);
                }
                "Orders" => {
                    file_paths.order_prefabs.push(file_path);
                }
                "SkillSettings" => {
                    file_paths.skill_settings.push(file_path);
                }
                "Factions" => {
                    file_paths.faction_prefabs.push(file_path);
                }
                "Tutorials" => {
                    file_paths.tutorial_prefabs.push(file_path);
                }
                "Other" => {
                    //Ignored by the game
                }
                _ => {
                    log::error!("TODO: {}", file_path)
                }
            }
        }

        Ok(ContentPackage {
            name,
            alt_names,
            steam_workshop_id,
            game_version,
            mod_version,
            install_time,
            expected_hash,
            _phantom: PhantomData,
            file_paths,
        })
    }

    pub fn load_file_list(
        &self,
        mod_path: &str,
        installed_packages: &[(ContentPackage<Regular>, PathBuf)],
    ) -> ContentFiles {
        #[rustfmt::skip]
        let files = ContentFiles {
            items: self.file_paths.items.iter().map(|file_path| ContentFile::<ItemFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            texts: self.file_paths.texts.iter().map(|file_path| ContentFile::<TextFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            submarines: self.file_paths.submarines.iter().map(|file_path| Arc::new(SubmarineAsset::load(&std::fs::read(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap())).collect(),
            outposts: self.file_paths.outposts.iter().map(|file_path| Arc::new(SubmarineAsset::load(&std::fs::read(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap())).collect(),
            outpost_modules: self.file_paths.outpost_modules.iter().map(|file_path| Arc::new(SubmarineAsset::load(&std::fs::read(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap())).collect(),
            wrecks: self.file_paths.wrecks.iter().map(|file_path| Arc::new(SubmarineAsset::load(&std::fs::read(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap())).collect(),
            beacon_stations: self.file_paths.beacon_stations.iter().map(|file_path| Arc::new(SubmarineAsset::load(&std::fs::read(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap())).collect(),
            enemy_submarines: self.file_paths.enemy_submarines.iter().map(|file_path| Arc::new(SubmarineAsset::load(&std::fs::read(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap())).collect(),
            npc_conversations: self.file_paths.npc_conversations.iter().map(|file_path| ContentFile::<NPCConversationFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            item_assemblies: self.file_paths.item_assemblies.iter().map(|file_path| ContentFile::<ItemAssemblyFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            talents: self.file_paths.talents.iter().map(|file_path| ContentFile::<TalentsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            npc_sets: self.file_paths.npc_sets.iter().map(|file_path| ContentFile::<NPCSetsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            characters: self.file_paths.characters.iter().map(|file_path| ContentFile::<CharacterFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            slideshows: self.file_paths.slideshows.iter().map(|file_path| ContentFile::<SlideshowsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            talent_trees: self.file_paths.talent_trees.iter().map(|file_path| ContentFile::<TalentTreesFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            level_generation_parameters: self.file_paths.level_generation_parameters.iter().map(|file_path| ContentFile::<LevelGenerationParametersFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            ballast_flora: self.file_paths.ballast_flora.iter().map(|file_path| ContentFile::<BallastFloraFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            start_items: self.file_paths.start_items.iter().map(|file_path| ContentFile::<StartItemsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            level_object_prefabs: self.file_paths.level_object_prefabs.iter().map(|file_path| ContentFile::<LevelObjectPrefabsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            afflictions: self.file_paths.afflictions.iter().map(|file_path| ContentFile::<AfflictionsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            random_events: self.file_paths.random_events.iter().map(|file_path| ContentFile::<RandomEventsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            structures: self.file_paths.structures.iter().map(|file_path| ContentFile::<StructureFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            ui_styles: self.file_paths.ui_styles.iter().map(|file_path| ContentFile::<UIStyleFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            upgrade_modules: self.file_paths.upgrade_modules.iter().map(|file_path| ContentFile::<UpgradeModulesFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            ruin_configs: self.file_paths.ruin_configs.iter().map(|file_path| ContentFile::<RuinConfigFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            outpost_configs: self.file_paths.outpost_configs.iter().map(|file_path| ContentFile::<OutpostConfigFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            wreck_ai_configs: self.file_paths.wreck_ai_configs.iter().map(|file_path| ContentFile::<WreckAIConfigFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            map_generation_params: self.file_paths.map_generation_params.iter().map(|file_path| ContentFile::<MapGenerationParametersFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            cave_generation_params: self.file_paths.cave_generation_params.iter().map(|file_path| ContentFile::<CaveGenerationParamsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            background_creature_prefabs: self.file_paths.background_creature_prefabs.iter().map(|file_path| ContentFile::<BackgroundCreaturePrefabsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            particle_prefabs: self.file_paths.particle_prefabs.iter().map(|file_path| ContentFile::<ParticlesFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            event_manager_settings: self.file_paths.event_manager_settings.iter().map(|file_path| ContentFile::<EventManagerSettingsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            npc_personality_traits: self.file_paths.npc_personality_traits.iter().map(|file_path| ContentFile::<NPCPersonalityTraitsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            jobs: self.file_paths.jobs.iter().map(|file_path| ContentFile::<JobsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            corpse_prefabs: self.file_paths.corpse_prefabs.iter().map(|file_path| ContentFile::<CorpsesFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            sound_prefabs: self.file_paths.sound_prefabs.iter().map(|file_path| ContentFile::<SoundsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            decal_prefabs: self.file_paths.decal_prefabs.iter().map(|file_path| ContentFile::<DecalsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            location_types: self.file_paths.location_types.iter().map(|file_path| ContentFile::<LocationTypesFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            mission_prefabs: self.file_paths.mission_prefabs.iter().map(|file_path| ContentFile::<MissionsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            order_prefabs: self.file_paths.order_prefabs.iter().map(|file_path| ContentFile::<OrdersFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            skill_settings: self.file_paths.skill_settings.iter().map(|file_path| ContentFile::<SkillSettingsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            faction_prefabs: self.file_paths.faction_prefabs.iter().map(|file_path| ContentFile::<FactionsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
            tutorial_prefabs: self.file_paths.tutorial_prefabs.iter().map(|file_path| ContentFile::<TutorialsFile>::load(&std::fs::read_to_string(replace_file_path(file_path, mod_path, installed_packages)).unwrap()).unwrap()).collect(),
        };

        files
    }
}

fn replace_file_path(
    path: &str,
    mod_path: &str,
    installed_packages: &[(ContentPackage<Regular>, PathBuf)],
) -> String {
    let other_mod_regex = RegexBuilder::new("%ModDir:(.+?)%")
        .case_insensitive(true)
        .build()
        .unwrap();
    let regex = RegexBuilder::new("%ModDir%")
        .case_insensitive(true)
        .build()
        .unwrap();

    let mut p = regex.replace(path, mod_path).to_string();

    for (_, [mod_ref]) in other_mod_regex.captures_iter(path).map(|c| c.extract()) {
        let package_match = installed_packages
            .iter()
            .find(|(p, _)| {
                p.steam_workshop_id
                    .is_some_and(|v| v.to_string() == mod_ref)
            })
            .or_else(|| {
                installed_packages
                    .iter()
                    .find(|(p, _)| p.name.as_ref().is_some_and(|v| v == mod_ref))
            })
            .or_else(|| {
                installed_packages.iter().find(|(p, _)| {
                    p.alt_names
                        .as_ref()
                        .is_some_and(|v| v.iter().any(|n| n == mod_ref))
                })
            });
        let Some((_, path)) = package_match else {
            panic!(
                "Package with id \"{}\" was not found in installed packages!",
                mod_ref
            );
        };
        p = p.replace(&format!("%ModDir:{}%", mod_ref), path.to_str().unwrap());
    }
    p
}
