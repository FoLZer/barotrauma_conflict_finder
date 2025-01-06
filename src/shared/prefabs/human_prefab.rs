use std::str::FromStr;

use bitfield_struct::bitfield;
use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::item_prefab::{Color, DoesNotExistError};

#[derive(Debug)]
pub struct HumanPrefab {
    pub identifier: String,
    pub human_properties: HumanProperties,
    pub itemsets: Vec<(ItemSet, f32)>,
    pub custom_character_infos: Vec<(CharacterInfo, f32)>,
    pub preferred_outpost_module_types: Vec<String>,
    pub npc_set_identifier: Option<String>,
}

impl HumanPrefab {
    pub fn new(element: Node, npc_set_identifier: Option<String>) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let human_properties = HumanProperties::new(element);
        let mut itemsets = Vec::new();
        let mut custom_character_infos = Vec::new();
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "itemset" => itemsets.push((
                    ItemSet::new(child),
                    child
                        .attribute_ignore_ascii_case("commonness")
                        .map_or(1.0, |v| v.parse::<f32>().unwrap()),
                )),
                "character" => {
                    custom_character_infos.push((
                        CharacterInfo::new(child),
                        child
                            .attribute_ignore_ascii_case("commonness")
                            .map_or(1.0, |v| v.parse::<f32>().unwrap()),
                    ));
                }
                _ => (),
            }
        }
        let preferred_outpost_module_types = element
            .attribute_ignore_ascii_case("preferredoutpostmoduletypes")
            .map(|v| {
                v.split(',')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        itemsets.shrink_to_fit();
        custom_character_infos.shrink_to_fit();

        Self {
            identifier,
            human_properties,
            itemsets,
            custom_character_infos,
            preferred_outpost_module_types,
            npc_set_identifier,
        }
    }
}

#[derive(Debug)]
pub struct ItemSet {
    pub items: Vec<Item>,
}

impl ItemSet {
    pub fn new(element: Node) -> Self {
        let items = element
            .children()
            .filter(Node::is_element)
            .filter(|v| v.tag_name().name().eq_ignore_ascii_case("item"))
            .map(Item::new)
            .collect();
        Self { items }
    }
}

#[derive(Debug)]
pub struct CharacterInfo {
    pub name: Option<String>,
    pub original_name: Option<String>,
    pub salary: Option<u32>,
    pub experience_points: Option<u32>,
    pub additional_talent_points: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub species_name: Option<String>,
    pub hair_index: Option<u32>,
    pub beard_index: Option<u32>,
    pub moustache_index: Option<u32>,
    pub face_attachment_index: Option<u32>,
    pub skin_color: Option<Color>,
    pub hair_color: Option<Color>,
    pub facial_hair_color: Option<Color>,
    pub start_items_given: bool,
    pub personality_name: Option<String>,
    pub ragdoll_file_name: Option<String>,
    pub npc_set_id: Option<String>,
    pub npc_id: Option<String>,
    pub missions_completed_since_death: Option<u32>,
    pub min_reputation_to_hire: Option<(String, f32)>,
    pub job: Option<Job>,
    pub saved_stat_values: Vec<StatValue>,
    pub talents: Vec<String>,
}

impl CharacterInfo {
    pub fn new(element: Node) -> Self {
        let name = element
            .attribute_ignore_ascii_case("name")
            .map(std::borrow::ToOwned::to_owned);
        let original_name = element
            .attribute_ignore_ascii_case("originalname")
            .map(std::borrow::ToOwned::to_owned);
        let salary = element
            .attribute_ignore_ascii_case("salary")
            .map(|v| v.parse::<u32>().unwrap());
        let experience_points = element
            .attribute_ignore_ascii_case("experiencepoints")
            .map(|v| v.parse::<u32>().unwrap());
        let additional_talent_points = element
            .attribute_ignore_ascii_case("additionaltalentpoints")
            .map(|v| v.parse::<u32>().unwrap());
        let tags = element.attribute_ignore_ascii_case("tags").map(|v| {
            v.split(',')
                .map(std::borrow::ToOwned::to_owned)
                .collect::<Vec<_>>()
        });
        let species_name = element
            .attribute_ignore_ascii_case("speciesname")
            .map(std::borrow::ToOwned::to_owned);
        let hair_index = element
            .attribute_ignore_ascii_case("hairindex")
            .map(|v| v.parse::<u32>().unwrap());
        let beard_index = element
            .attribute_ignore_ascii_case("hairindex")
            .map(|v| v.parse::<u32>().unwrap());
        let moustache_index = element
            .attribute_ignore_ascii_case("moustacheindex")
            .map(|v| v.parse::<u32>().unwrap());
        let face_attachment_index = element
            .attribute_ignore_ascii_case("faceattachmentindex")
            .map(|v| v.parse::<u32>().unwrap());
        let skin_color = element
            .attribute_ignore_ascii_case("skincolor")
            .map(|v| v.parse::<Color>().unwrap());
        let hair_color = element
            .attribute_ignore_ascii_case("haircolor")
            .map(|v| v.parse::<Color>().unwrap());
        let facial_hair_color = element
            .attribute_ignore_ascii_case("facialhaircolor")
            .map(|v| v.parse::<Color>().unwrap());
        let start_items_given = element
            .attribute_ignore_ascii_case("startitemsgiven")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let personality_name = element
            .attribute_ignore_ascii_case("personality")
            .map(std::borrow::ToOwned::to_owned);
        let ragdoll_file_name = element
            .attribute_ignore_ascii_case("ragdoll")
            .map(std::borrow::ToOwned::to_owned);
        let npc_set_id = element
            .attribute_ignore_ascii_case("npcsetid")
            .map(std::borrow::ToOwned::to_owned);
        let npc_id = element
            .attribute_ignore_ascii_case("npcid")
            .map(std::borrow::ToOwned::to_owned);
        let missions_completed_since_death = element
            .attribute_ignore_ascii_case("missionscompletedsincedeath")
            .map(|v| v.parse::<u32>().unwrap());
        let min_reputation_to_hire = element
            .attribute_ignore_ascii_case("factionId")
            .map(std::borrow::ToOwned::to_owned)
            .zip(
                element
                    .attribute_ignore_ascii_case("minreputation")
                    .map(|v| v.parse::<f32>().unwrap()),
            );
        let mut job = None;
        let mut saved_stat_values = Vec::new();
        let mut talents = Vec::new();
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "job" => job = Some(Job::new(child)),
                "savedstatvalues" => {
                    for child in child.children().filter(Node::is_element) {
                        let stat_type = child
                            .attribute_ignore_ascii_case("stattype")
                            .map(|v| v.parse::<StatType>().unwrap())
                            .unwrap();
                        let value = child
                            .attribute_ignore_ascii_case("statvalue")
                            .map(|v| v.parse::<f32>().unwrap())
                            .unwrap();
                        let stat_identifier = child
                            .attribute_ignore_ascii_case("statidentifier")
                            .map(std::borrow::ToOwned::to_owned)
                            .unwrap();
                        let remove_on_death = child
                            .attribute_ignore_ascii_case("removeondeath")
                            .map_or(true, |v| v.parse::<bool>().unwrap());
                        saved_stat_values.push(StatValue {
                            stat_type,
                            value,
                            identifier: stat_identifier,
                            remove_on_death,
                        });
                    }
                }
                "talents" => {
                    //let version = child
                    //    .attribute_ignore_ascii_case("removeondeath")
                    //    .map(|v| v.parse::<Version>().unwrap());
                    talents = child
                        .children()
                        .filter(Node::is_element)
                        .filter(|v| v.tag_name().name().eq_ignore_ascii_case("talent"))
                        .map(|child| {
                            child
                                .attribute_ignore_ascii_case("identifier")
                                .map(std::borrow::ToOwned::to_owned)
                                .unwrap()
                        })
                        .collect::<Vec<_>>();
                }
                _ => (),
            }
        }

        saved_stat_values.shrink_to_fit();
        talents.shrink_to_fit();

        Self {
            name,
            original_name,
            salary,
            experience_points,
            additional_talent_points,
            tags,
            species_name,
            hair_index,
            beard_index,
            moustache_index,
            face_attachment_index,
            skin_color,
            hair_color,
            facial_hair_color,
            start_items_given,
            personality_name,
            ragdoll_file_name,
            npc_set_id,
            npc_id,
            missions_completed_since_death,
            min_reputation_to_hire,
            job,
            saved_stat_values,
            talents,
        }
    }
}

#[derive(Debug)]
pub struct Job {
    pub identifier: String,
    pub skills: Vec<Skill>,
}

impl Job {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(std::borrow::ToOwned::to_owned)
            .unwrap();
        let skills = element
            .children()
            .filter(Node::is_element)
            .filter(|v| v.tag_name().name().eq_ignore_ascii_case("skill"))
            .map(|child| {
                let skill_identifier = child
                    .attribute_ignore_ascii_case("identifier")
                    .map(std::borrow::ToOwned::to_owned)
                    .unwrap();
                let level = child
                    .attribute_ignore_ascii_case("level")
                    .map(|v| v.parse::<f32>().unwrap())
                    .unwrap();
                Skill {
                    identifier: skill_identifier,
                    level,
                }
            })
            .collect::<Vec<_>>();

        Self { identifier, skills }
    }
}

#[derive(Debug)]
pub struct Skill {
    pub identifier: String,
    pub level: f32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum StatType {
    // Used to indicate an invalid stat type. Should not be used.
    None,

    // Boosts electrical skill by a flat amount.
    ElectricalSkillBonus,

    // Boosts helm skill by a flat amount.
    HelmSkillBonus,

    // Boosts mechanical skill by a flat amount.
    MechanicalSkillBonus,

    // Boosts medical skill by a flat amount.
    MedicalSkillBonus,

    // Boosts weapons skill by a flat amount.
    WeaponsSkillBonus,

    // Boosts the character's helm skill to the given value if it's lower than the given value.
    HelmSkillOverride,

    // Boosts the character's medical skill to the given value if it's lower than the given value.
    MedicalSkillOverride,

    // Boosts the character's weapons skill to the given value if it's lower than the given value.
    WeaponsSkillOverride,

    // Boosts the character's electrical skill to the given value if it's lower than the given value.
    ElectricalSkillOverride,

    // Boosts the character's mechanical skill to the given value if it's lower than the given value.
    MechanicalSkillOverride,

    // Increases character's maximum vitality by a percentage.
    MaximumHealthMultiplier,

    // Increases both walking and swimming speed of the character by a percentage.
    MovementSpeed,

    // Increases the character's walking speed by a percentage.
    WalkingSpeed,

    // Increases the character's swimming speed by a percentage.
    SwimmingSpeed,

    // Increases the character's speed by a percentage when using an item that propels the character forwards (such as a diving scooter).
    PropulsionSpeed,

    // Decreases how long it takes for buffs applied to the character decay over time by a percentage.
    // Buffs are afflictions that have isBuff set to true.
    BuffDurationMultiplier,

    // Decreases how long it takes for debuff applied to the character decay over time by a percentage.
    // Debuffs are afflictions that have isBuff set to false.
    DebuffDurationMultiplier,

    // Increases the strength of afflictions that are applied to the character by a percentage.
    // Medicines are items that have the "medical" tag.
    MedicalItemEffectivenessMultiplier,

    // Increases the resistance to pushing force caused by flowing water by a percentage. The resistance cannot be below 0% or higher than 100%.
    FlowResistance,

    // Increases how much damage the character deals via all attacks by a percentage.
    AttackMultiplier,

    // Increases how much damage the character deals to other characters on the same team by a percentage.
    TeamAttackMultiplier,

    // Decreases the reload time of ranged weapons held by the character by a percentage.
    RangedAttackSpeed,

    /// Increases the damage dealt by ranged weapons held by the character by a percentage.
    RangedAttackMultiplier,

    // Decreases the reload time of submarine turrets operated by the character by a percentage.
    TurretAttackSpeed,

    // Decreases the power consumption of submarine turrets operated by the character by a percentage.
    TurretPowerCostReduction,

    // Increases how fast submarine turrets operated by the character charge up by a percentage. Affects turrets like pulse laser.
    TurretChargeSpeed,

    // Increases how fast the character can swing melee weapons by a percentage.
    MeleeAttackSpeed,

    // Increases the damage dealt by melee weapons held by the character by a percentage.
    MeleeAttackMultiplier,

    // Decreases the spread of ranged weapons held by the character by a percentage.
    RangedSpreadReduction,

    // Increases the repair speed of the character by a percentage.
    RepairSpeed,

    // Increases the repair speed of the character when repairing mechanical items by a percentage.
    MechanicalRepairSpeed,

    // Increases the repair speed of the character when repairing electrical items by a percentage.
    ElectricalRepairSpeed,

    // Increase deconstruction speed of deconstructor operated by the character by a percentage.
    DeconstructorSpeedMultiplier,

    // Increases the repair speed of repair tools that fix submarine walls by a percentage.
    RepairToolStructureRepairMultiplier,

    // Increases the wall damage of tools that destroy submarine walls like plasma cutter by a percentage.
    RepairToolStructureDamageMultiplier,

    // Increase the detach speed of items like minerals that require a tool to detach from the wall by a percentage.
    RepairToolDeattachTimeMultiplier,

    // Allows the character to repair mechanical items past the maximum condition by a flat percentage amount. For example setting this to 0.1 allows the character to repair mechanical items to 110% condition.
    MaxRepairConditionMultiplierMechanical,

    // Allows the character to repair electrical items past the maximum condition by a flat percentage amount. For example setting this to 0.1 allows the character to repair electrical items to 110% condition.
    MaxRepairConditionMultiplierElectrical,

    // Increase the the quality of items crafted by the character by a flat amount.
    // Can be made to only affect certain item with a given tag types by specifying a tag via CharacterAbilityGivePermanentStat, when no tag is specified the ability affects all items.
    IncreaseFabricationQuality,

    // Boosts the condition of genes combined by the character by a flat amount.
    GeneticMaterialRefineBonus,

    // Reduces the chance to taint a gene when combining genes by a percentage. Tainting probability can not go below 0% or above 100%.
    GeneticMaterialTaintedProbabilityReductionOnCombine,

    // Increases the speed at which the character gains skills by a percentage.
    SkillGainSpeed,

    // Whenever the character's skill level up add a flat amount of more skill levels to the character.
    ExtraLevelGain,

    // Increases the speed at which the character gains helm skill by a percentage.
    HelmSkillGainSpeed,

    // Increases the speed at which the character gains weapons skill by a percentage.
    WeaponsSkillGainSpeed,

    // Increases the speed at which the character gains medical skill by a percentage.
    MedicalSkillGainSpeed,

    // Increases the speed at which the character gains electrical skill by a percentage.
    ElectricalSkillGainSpeed,

    // Increases the speed at which the character gains mechanical skill by a percentage.
    MechanicalSkillGainSpeed,

    // Increases the strength of afflictions the character applies to other characters via medicine by a percentage.
    // Medicines are items that have the "medical" tag.
    MedicalItemApplyingMultiplier,

    // Increases the strength of afflictions the character applies to other characters via medicine by a percentage.
    // Works only for afflictions that have isBuff set to true.
    BuffItemApplyingMultiplier,

    // Increases the strength of afflictions the character applies to other characters via medicine by a percentage.
    // Works only for afflictions that have "poison" type.
    PoisonMultiplier,

    // Increases how long the character can tinker with items by a flat amount where 1 = 1 second.
    TinkeringDuration,

    // Increases the effectiveness of the character's tinkerings by a percentage.
    // Tinkering strength affects the speed and effectiveness of the item that is being tinkered with.
    TinkeringStrength,

    // Increases how much condition tinkered items lose when the character tinkers with them by a percentage.
    TinkeringDamage,

    // Increases how much reputation the character gains by a percentage.
    // Can be made to only affect certain factions with a given tag types by specifying a tag via CharacterAbilityGivePermanentStat, when no tag is specified the ability affects all factions.
    ReputationGainMultiplier,

    // Increases how much reputation the character loses by a percentage.
    // Can be made to only affect certain factions with a given tag types by specifying a tag via CharacterAbilityGivePermanentStat, when no tag is specified the ability affects all factions.
    ReputationLossMultiplier,

    // Increases how much money the character gains from missions by a percentage.
    MissionMoneyGainMultiplier,

    // Increases how much talent experience the character gains from all sources by a percentage.
    ExperienceGainMultiplier,

    // Increases how much talent experience the character gains from missions by a percentage.
    MissionExperienceGainMultiplier,

    // Increases how many missions the characters crew can have at the same time by a flat amount.
    ExtraMissionCount,

    // Increases how many items are in stock in special sales in the store by a flat amount.
    ExtraSpecialSalesCount,

    // Increases how much money is gained from selling items to the store by a percentage.
    StoreSellMultiplier,

    // Decreases the prices of items in affiliated store by a percentage.
    StoreBuyMultiplierAffiliated,

    // Decreases the prices of items in all stores by a percentage.
    StoreBuyMultiplier,

    // Decreases the price of upgrades and submarines in affiliated outposts by a percentage.
    ShipyardBuyMultiplierAffiliated,

    // Decreases the price of upgrades and submarines in all outposts by a percentage.
    ShipyardBuyMultiplier,

    // Limits how many of a certain item can be attached to the wall in the submarine at the same time.
    // Has to be used with CharacterAbilityGivePermanentStat to specify the tag of the item that is affected. Does nothing if no tag is specified.
    MaxAttachableCount,

    // Increase the radius of explosions caused by the character by a percentage.
    ExplosionRadiusMultiplier,

    // Increases the damage of explosions caused by the character by a percentage.
    ExplosionDamageMultiplier,

    // Decreases the time it takes to fabricate items on fabricators operated by the character by a percentage.
    FabricationSpeed,

    // Increases how much damage the character deals to ballast flora by a percentage.
    BallastFloraDamageMultiplier,

    // Increases the time it takes for the character to pass out when out of oxygen.
    HoldBreathMultiplier,

    // Used to set the character's apprencticeship to a certain job.
    // Used by the "apprenticeship" talent and requires a job to be specified via CharacterAbilityGivePermanentStat.
    Apprenticeship,

    // Increases the revival chance of the character when performing CPR by a percentage.
    CPRBoost,

    // Can be used to prevent certain talents from being unlocked by specifying the talent's identifier via CharacterAbilityGivePermanentStat.
    LockedTalents,

    // Used to reduce or increase the cost of hiring certain jobs by a percentage.
    HireCostMultiplier,

    // Used to increase how much items can stack in the characters inventory.
    InventoryExtraStackSize,

    // Modifies the range of the sounds emitted by the character (can be used to make the character easier or more difficult for monsters to hear)
    SoundRangeMultiplier,

    // Modifies how far the character can be seen from (can be used to make the character easier or more difficult for monsters to see)
    SightRangeMultiplier,

    // Reduces the dual wielding penalty by a percentage.
    DualWieldingPenaltyReduction,

    // Multiplier bonus to melee attacks coming from a natural weapon (limb).
    NaturalMeleeAttackMultiplier,

    // Multiplier bonus to ranged attacks coming from a natural weapon (limb).
    NaturalRangedAttackMultiplier,
}

impl FromStr for StatType {
    type Err = DoesNotExistError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(match s {
            "None" => StatType::None,
            "ElectricalSkillBonus" => StatType::ElectricalSkillBonus,
            "HelmSkillBonus" => StatType::HelmSkillBonus,
            "MechanicalSkillBonus" => StatType::MechanicalSkillBonus,
            "MedicalSkillBonus" => StatType::MedicalSkillBonus,
            "WeaponsSkillBonus" => StatType::WeaponsSkillBonus,
            "HelmSkillOverride" => StatType::HelmSkillOverride,
            "MedicalSkillOverride" => StatType::MedicalSkillOverride,
            "WeaponsSkillOverride" => StatType::WeaponsSkillOverride,
            "ElectricalSkillOverride" => StatType::ElectricalSkillOverride,
            "MechanicalSkillOverride" => StatType::MechanicalSkillOverride,
            "MaximumHealthMultiplier" => StatType::MaximumHealthMultiplier,
            "MovementSpeed" => StatType::MovementSpeed,
            "WalkingSpeed" => StatType::WalkingSpeed,
            "SwimmingSpeed" => StatType::SwimmingSpeed,
            "PropulsionSpeed" => StatType::PropulsionSpeed,
            "BuffDurationMultiplier" => StatType::BuffDurationMultiplier,
            "DebuffDurationMultiplier" => StatType::DebuffDurationMultiplier,
            "MedicalItemEffectivenessMultiplier" => StatType::MedicalItemEffectivenessMultiplier,
            "FlowResistance" => StatType::FlowResistance,
            "AttackMultiplier" => StatType::AttackMultiplier,
            "TeamAttackMultiplier" => StatType::TeamAttackMultiplier,
            "RangedAttackSpeed" => StatType::RangedAttackSpeed,
            "RangedAttackMultiplier" => StatType::RangedAttackMultiplier,
            "TurretAttackSpeed" => StatType::TurretAttackSpeed,
            "TurretPowerCostReduction" => StatType::TurretPowerCostReduction,
            "TurretChargeSpeed" => StatType::TurretChargeSpeed,
            "MeleeAttackSpeed" => StatType::MeleeAttackSpeed,
            "MeleeAttackMultiplier" => StatType::MeleeAttackMultiplier,
            "RangedSpreadReduction" => StatType::RangedSpreadReduction,
            "RepairSpeed" => StatType::RepairSpeed,
            "MechanicalRepairSpeed" => StatType::MechanicalRepairSpeed,
            "ElectricalRepairSpeed" => StatType::ElectricalRepairSpeed,
            "DeconstructorSpeedMultiplier" => StatType::DeconstructorSpeedMultiplier,
            "RepairToolStructureRepairMultiplier" => StatType::RepairToolStructureRepairMultiplier,
            "RepairToolStructureDamageMultiplier" => StatType::RepairToolStructureDamageMultiplier,
            "RepairToolDeattachTimeMultiplier" => StatType::RepairToolDeattachTimeMultiplier,
            "MaxRepairConditionMultiplierMechanical" => {
                StatType::MaxRepairConditionMultiplierMechanical
            }
            "MaxRepairConditionMultiplierElectrical" => {
                StatType::MaxRepairConditionMultiplierElectrical
            }
            "IncreaseFabricationQuality" => StatType::IncreaseFabricationQuality,
            "GeneticMaterialRefineBonus" => StatType::GeneticMaterialRefineBonus,
            "GeneticMaterialTaintedProbabilityReductionOnCombine" => {
                StatType::GeneticMaterialTaintedProbabilityReductionOnCombine
            }
            "SkillGainSpeed" => StatType::SkillGainSpeed,
            "ExtraLevelGain" => StatType::ExtraLevelGain,
            "HelmSkillGainSpeed" => StatType::HelmSkillGainSpeed,
            "WeaponsSkillGainSpeed" => StatType::WeaponsSkillGainSpeed,
            "MedicalSkillGainSpeed" => StatType::MedicalSkillGainSpeed,
            "ElectricalSkillGainSpeed" => StatType::ElectricalSkillGainSpeed,
            "MechanicalSkillGainSpeed" => StatType::MechanicalSkillGainSpeed,
            "MedicalItemApplyingMultiplier" => StatType::MedicalItemApplyingMultiplier,
            "BuffItemApplyingMultiplier" => StatType::BuffItemApplyingMultiplier,
            "PoisonMultiplier" => StatType::PoisonMultiplier,
            "TinkeringDuration" => StatType::TinkeringDuration,
            "TinkeringStrength" => StatType::TinkeringStrength,
            "TinkeringDamage" => StatType::TinkeringDamage,
            "ReputationGainMultiplier" => StatType::ReputationGainMultiplier,
            "ReputationLossMultiplier" => StatType::ReputationLossMultiplier,
            "MissionMoneyGainMultiplier" => StatType::MissionMoneyGainMultiplier,
            "ExperienceGainMultiplier" => StatType::ExperienceGainMultiplier,
            "MissionExperienceGainMultiplier" => StatType::MissionExperienceGainMultiplier,
            "ExtraMissionCount" => StatType::ExtraMissionCount,
            "ExtraSpecialSalesCount" => StatType::ExtraSpecialSalesCount,
            "StoreSellMultiplier" => StatType::StoreSellMultiplier,
            "StoreBuyMultiplierAffiliated" => StatType::StoreBuyMultiplierAffiliated,
            "StoreBuyMultiplier" => StatType::StoreBuyMultiplier,
            "ShipyardBuyMultiplierAffiliated" => StatType::ShipyardBuyMultiplierAffiliated,
            "ShipyardBuyMultiplier" => StatType::ShipyardBuyMultiplier,
            "MaxAttachableCount" => StatType::MaxAttachableCount,
            "ExplosionRadiusMultiplier" => StatType::ExplosionRadiusMultiplier,
            "ExplosionDamageMultiplier" => StatType::ExplosionDamageMultiplier,
            "FabricationSpeed" => StatType::FabricationSpeed,
            "BallastFloraDamageMultiplier" => StatType::BallastFloraDamageMultiplier,
            "HoldBreathMultiplier" => StatType::HoldBreathMultiplier,
            "Apprenticeship" => StatType::Apprenticeship,
            "CPRBoost" => StatType::CPRBoost,
            "LockedTalents" => StatType::LockedTalents,
            "HireCostMultiplier" => StatType::HireCostMultiplier,
            "InventoryExtraStackSize" => StatType::InventoryExtraStackSize,
            "SoundRangeMultiplier" => StatType::SoundRangeMultiplier,
            "SightRangeMultiplier" => StatType::SightRangeMultiplier,
            "DualWieldingPenaltyReduction" => StatType::DualWieldingPenaltyReduction,
            "NaturalMeleeAttackMultiplier" => StatType::NaturalMeleeAttackMultiplier,
            "NaturalRangedAttackMultiplier" => StatType::NaturalRangedAttackMultiplier,
            _ => return Err(DoesNotExistError(s.to_owned())),
        })
    }
}

#[derive(Debug)]
pub struct StatValue {
    pub stat_type: StatType,
    pub value: f32,
    pub identifier: String,
    pub remove_on_death: bool,
}

#[derive(Debug)]

pub struct Item {
    pub amount: u32,
    pub identifier: String,
    pub equip: bool,
    pub tags: Vec<String>,
    pub inner_items: Vec<Item>,
}

impl Item {
    pub fn new(element: Node) -> Self {
        let amount = element
            .attribute_ignore_ascii_case("amount")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .unwrap()
            .to_owned();
        let equip = element
            .attribute_ignore_ascii_case("equip")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let tags = element
            .attribute_ignore_ascii_case("tags")
            .map(|v| {
                v.split(',')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let inner_items = element
            .children()
            .filter(Node::is_element)
            .map(|child| Item::new(child))
            .collect();
        Self {
            amount,
            identifier,
            equip,
            tags,
            inner_items,
        }
    }
}

#[derive(Debug)]
pub struct HumanProperties {
    pub job: String,
    pub commonness: f32,
    pub health_multiplier: f32,
    pub health_multiplier_in_multiplayer: f32,
    pub aim_speed: f32,
    pub aim_accuracy: f32,
    pub experience_points: u32,
    pub tags: Vec<String>,
    pub module_flags: Vec<String>,
    pub spawn_point_tags: Vec<String>,
    pub campaign_interaction_type: InteractionType,
    pub behavior: BehaviorType,
    pub report_range: f32,
    pub faction: String,
    pub group: String,
    pub allow_dragging_indefinitely: bool,
}

impl HumanProperties {
    pub fn new(element: Node) -> Self {
        Self {
            job: element
                .attribute_ignore_ascii_case("job")
                .map(|v| v.parse().unwrap())
                .unwrap_or_default(),
            commonness: element
                .attribute_ignore_ascii_case("commonness")
                .map_or(1.0, |v| v.parse().unwrap()),
            health_multiplier: element
                .attribute_ignore_ascii_case("healthmultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            health_multiplier_in_multiplayer: element
                .attribute_ignore_ascii_case("healthmultiplierinmultiplayer")
                .map_or(1.0, |v| v.parse().unwrap()),
            aim_speed: element
                .attribute_ignore_ascii_case("aimspeed")
                .map_or(1.0, |v| v.parse().unwrap()),
            aim_accuracy: element
                .attribute_ignore_ascii_case("aimaccuracy")
                .map_or(1.0, |v| v.parse().unwrap()),
            experience_points: element
                .attribute_ignore_ascii_case("experiencepoints")
                .map_or(0, |v| v.parse().unwrap()),
            tags: element
                .attribute_ignore_ascii_case("tags")
                .map(|v| v.split(',').map(std::borrow::ToOwned::to_owned).collect())
                .unwrap_or_default(),
            module_flags: element
                .attribute_ignore_ascii_case("moduleflags")
                .map(|v| v.split(',').map(std::borrow::ToOwned::to_owned).collect())
                .unwrap_or_default(),
            spawn_point_tags: element
                .attribute_ignore_ascii_case("spawnpointtags")
                .map(|v| v.split(',').map(std::borrow::ToOwned::to_owned).collect())
                .unwrap_or_default(),
            campaign_interaction_type: element
                .attribute_ignore_ascii_case("campaigninteractiontype")
                .map_or(InteractionType::None, |v| v.parse().unwrap()),
            behavior: element
                .attribute_ignore_ascii_case("behavior")
                .map_or(BehaviorType::Passive, |v| v.parse().unwrap()),
            report_range: element
                .attribute_ignore_ascii_case("reportrange")
                .map_or(f32::MAX, |v| v.parse().unwrap()),
            faction: element
                .attribute_ignore_ascii_case("faction")
                .map(|v| v.parse().unwrap())
                .unwrap_or_default(),
            group: element
                .attribute_ignore_ascii_case("group")
                .map(|v| v.parse().unwrap())
                .unwrap_or_default(),
            allow_dragging_indefinitely: element
                .attribute_ignore_ascii_case("allow_dragging_indefinitely")
                .is_some_and(|v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
pub enum InteractionType {
    None,
    Talk,
    Examine,
    Map,
    Crew,
    Store,
    Upgrade,
    PurchaseSub,
    MedicalClinic,
    Cargo,
}

impl FromStr for InteractionType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "Talk" => Ok(Self::Talk),
            "Examine" => Ok(Self::Examine),
            "Map" => Ok(Self::Map),
            "Crew" => Ok(Self::Crew),
            "Store" => Ok(Self::Store),
            "Upgrade" => Ok(Self::Upgrade),
            "PurchaseSub" => Ok(Self::PurchaseSub),
            "MedicalClinic" => Ok(Self::MedicalClinic),
            "Cargo" => Ok(Self::Cargo),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum BehaviorType {
    Patrol,
    Passive,
    StayInHull,
    Active,
}

impl FromStr for BehaviorType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "patrol" => Ok(Self::Patrol),
            "passive" => Ok(Self::Passive),
            "stayinhull" => Ok(Self::StayInHull),
            "active" => Ok(Self::Active),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct CorpsePrefab {
    pub spawn_position: PositionType,
    pub min_money: u32,
    pub max_money: u32,
    pub human_prefab: HumanPrefab,
}

impl CorpsePrefab {
    pub fn new(element: Node) -> Self {
        Self {
            spawn_position: element
                .attribute_ignore_ascii_case("spawnposition")
                .map_or(PositionType::new().with_wreck(true), |v| v.parse().unwrap()),
            min_money: element
                .attribute_ignore_ascii_case("minmoney")
                .map_or(0, |v| v.parse().unwrap()),
            max_money: element
                .attribute_ignore_ascii_case("maxmoney")
                .map_or(0, |v| v.parse().unwrap()),
            human_prefab: HumanPrefab::new(element, None),
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.human_prefab.identifier
    }
}

#[bitfield(u16)]

pub struct PositionType {
    pub main_path: bool,
    pub side_path: bool,
    pub cave: bool,
    pub ruin: bool,
    pub wreck: bool,
    pub beacon_station: bool,
    pub abyss: bool,
    pub abyss_cave: bool,
    pub outpost: bool,
    #[bits(7)]
    _unused: u8,
}

impl FromStr for PositionType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::new()),
            "mainpath" => Ok(Self::new().with_main_path(true)),
            "sidepath" => Ok(Self::new().with_side_path(true)),
            "cave" => Ok(Self::new().with_cave(true)),
            "ruin" => Ok(Self::new().with_ruin(true)),
            "wreck" => Ok(Self::new().with_wreck(true)),
            "beaconstation" => Ok(Self::new().with_beacon_station(true)),
            "abyss" => Ok(Self::new().with_abyss(true)),
            "abysscave" => Ok(Self::new().with_abyss_cave(true)),
            "outpost" => Ok(Self::new().with_outpost(true)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
