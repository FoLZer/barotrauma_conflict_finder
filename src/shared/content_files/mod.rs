pub mod afflictions_file;
pub mod background_creature_prefabs_file;
pub mod ballast_flora_file;
pub mod cave_generation_parameters_file;
pub mod character_file;
pub mod corpses_file;
pub mod decals_file;
pub mod event_manager_settings_file;
pub mod factions_file;
pub mod item_assembly_file;
pub mod item_file;
pub mod jobs_file;
pub mod level_generation_parameters_file;
pub mod level_object_prefabs_file;
pub mod location_types_files;
pub mod map_generation_parameters_file;
pub mod missions_file;
pub mod npc_conversations_file;
pub mod npc_personality_traits_file;
pub mod npc_sets_file;
pub mod orders_file;
pub mod outpost_config_file;
pub mod particles_file;
pub mod random_events_file;
pub mod ruin_config_file;
pub mod skill_settings_file;
pub mod slideshows_file;
pub mod sounds_file;
pub mod start_items_file;
pub mod structure_file;
pub mod submarine_file;
pub mod talent_trees_file;
pub mod talents_file;
pub mod text_file;
pub mod tutorials_file;
pub mod ui_style;
pub mod upgrade_moduled_file;
pub mod wreck_ai_config_file;

pub mod prelude {
    pub use super::{
        afflictions_file::AfflictionsFile,
        background_creature_prefabs_file::BackgroundCreaturePrefabsFile,
        ballast_flora_file::BallastFloraFile,
        cave_generation_parameters_file::CaveGenerationParamsFile, character_file::CharacterFile,
        corpses_file::CorpsesFile, decals_file::DecalsFile,
        event_manager_settings_file::EventManagerSettingsFile, factions_file::FactionsFile,
        item_assembly_file::ItemAssemblyFile, item_file::ItemFile, jobs_file::JobsFile,
        level_generation_parameters_file::LevelGenerationParametersFile,
        level_object_prefabs_file::LevelObjectPrefabsFile, location_types_files::LocationTypesFile,
        map_generation_parameters_file::MapGenerationParametersFile, missions_file::MissionsFile,
        npc_conversations_file::NPCConversationFile,
        npc_personality_traits_file::NPCPersonalityTraitsFile, npc_sets_file::NPCSetsFile,
        orders_file::OrdersFile, outpost_config_file::OutpostConfigFile,
        particles_file::ParticlesFile, random_events_file::RandomEventsFile,
        ruin_config_file::RuinConfigFile, skill_settings_file::SkillSettingsFile,
        slideshows_file::SlideshowsFile, sounds_file::SoundsFile, start_items_file::StartItemsFile,
        structure_file::StructureFile, talent_trees_file::TalentTreesFile,
        talents_file::TalentsFile, text_file::TextFile, tutorials_file::TutorialsFile,
        ui_style::UIStyleFile, upgrade_moduled_file::UpgradeModulesFile,
        wreck_ai_config_file::WreckAIConfigFile,
    };
}
