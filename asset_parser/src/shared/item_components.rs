pub mod adder_component;
pub mod and_component;
pub mod arithmetic_component;
pub mod boolean_operator;
pub mod button_terminal;
pub mod circuit_box;
pub mod color_component;
pub mod concat_component;
pub mod connection_panel;
pub mod connection_selector_component;
pub mod controller;
pub mod custom_interface;
pub mod deconstructor;
pub mod delay;
pub mod demultiplexer_component;
pub mod divide_component;
pub mod docking_port;
pub mod door;
pub mod electrical_discharger;
pub mod engine;
pub mod entity_spawner;
pub mod equals_component;
pub mod exponentiation_component;
pub mod fabricator;
pub mod function_component;
pub mod genetic_material;
pub mod greater_component;
pub mod growable;
pub mod holdable;
pub mod id_card;
pub mod item_container;
pub mod item_label;
pub mod ladder;
pub mod level_resource;
pub mod light;
pub mod melee_weapon;
pub mod memory_component;
pub mod minimap;
pub mod modulo_component;
pub mod motion_sensor;
pub mod multiply_component;
pub mod name_tag;
pub mod not_component;
pub mod or_component;
pub mod oscillator;
pub mod outpost_terminal;
pub mod oxygen_detector;
pub mod oxygen_generator;
pub mod pickable;
pub mod planter;
pub mod power_container;
pub mod power_transfer;
pub mod powered;
pub mod projectile;
pub mod propulsion;
pub mod pump;
pub mod quality;
pub mod ranged_weapon;
pub mod reactor;
pub mod regex_find_component;
pub mod relay;
pub mod repair_tool;
pub mod repairable;
pub mod rope;
pub mod scanner;
pub mod signal_check;
pub mod smoke_detector;
pub mod sonar;
pub mod sonar_transducer;
pub mod status_hud;
pub mod steering;
pub mod string_component;
pub mod subtract_component;
pub mod terminal;
pub mod throwable;
pub mod trigger_component;
pub mod trigonometric_function_component;
pub mod turret;
pub mod vent;
pub mod water_detector;
pub mod wearable;
pub mod wifi;
pub mod wire;
pub mod xor_component;

use adder_component::AdderComponent;
use and_component::AndComponent;
use button_terminal::ButtonTerminalComponent;
use circuit_box::CircuitBoxComponent;
use color_component::ColorComponent;
use concat_component::ConcatComponent;
use connection_panel::ConnectionPannelComponent;
use controller::ControllerComponent;
use custom_interface::CustomInterfaceComponent;
use deconstructor::DeconstructorComponent;
use delay::DelayComponent;
use demultiplexer_component::DemultiplexerComponent;
use divide_component::DivideComponent;
use docking_port::DockingPortComponent;
use door::DoorComponent;
use electrical_discharger::ElectricalDischargerComponent;
use engine::EngineComponent;
use entity_spawner::EntitySpawnerComponent;
use equals_component::EqualsComponent;
use exponentiation_component::ExponentiationComponent;
use fabricator::FabricatorComponent;
use function_component::FunctionComponent;
use genetic_material::GeneticMaterialComponent;
use greater_component::GreaterComponent;
use growable::GrowableComponent;
use holdable::HoldableComponent;
use id_card::IdCardComponent;
use item_container::ItemContainerComponent;
use item_label::ItemLabelComponent;
use ladder::LadderComponent;
use level_resource::LevelResourceComponent;
use light::LightComponent;
use melee_weapon::MeleeWeaponComponent;
use memory_component::MemoryComponent;
use minimap::MiniMapComponent;
use modulo_component::ModuloComponent;
use motion_sensor::MotionSensorComponent;
use multiply_component::MultiplyComponent;
use name_tag::NameTagComponent;
use not_component::NotComponent;
use or_component::OrComponent;
use oscillator::OscillatorComponent;
use outpost_terminal::OutpostTerminalComponent;
use oxygen_detector::OxygenDetectorComponent;
use oxygen_generator::OxygenGeneratorComponent;
use pickable::PickableComponent;
use planter::PlanterComponent;
use power_container::PowerContainerComponent;
use power_transfer::PowerTransfer;
use powered::PoweredComponent;
use projectile::ProjectileComponent;
use propulsion::PropulsionComponent;
use pump::PumpComponent;
use quality::QualityComponent;
use ranged_weapon::RangedWeaponComponent;
use reactor::ReactorComponent;
use regex_find_component::RegExFindComponent;
use relay::RelayComponent;
use repair_tool::RepairToolComponent;
use repairable::RepairableComponent;
use rope::RopeComponent;
use roxmltree::Node;
use scanner::ScannerComponent;
use signal_check::SignalCheckComponent;
use smoke_detector::SmokeDetectorComponent;
use sonar::SonarComponent;
use sonar_transducer::SonarTransducerComponent;
use status_hud::StatusHUDComponent;
use steering::SteeringComponent;
use subtract_component::SubtractComponent;
use terminal::TerminalComponent;
use throwable::ThrowableComponent;
use trigger_component::TriggerComponent;
use trigonometric_function_component::TrigonometricFunctionComponent;
use turret::TurretComponent;
use vent::VentComponent;
use water_detector::WaterDetectorComponent;
use wearable::WearableComponent;
use wifi::WifiComponent;
use wire::WireComponent;
use xor_component::XorComponent;

use crate::shared::{
    prefabs::{level_object_prefab::LogicalOperatorType, map_generation_params::Point},
    util::NodeExp,
};

#[derive(Default, Debug)]
pub struct ItemComponents {
    pub relay: Option<RelayComponent>,
    pub oscillator: Option<OscillatorComponent>,
    pub light: Vec<LightComponent>,
    pub wifi: Option<WifiComponent>,
    pub signal_check: Option<SignalCheckComponent>,
    pub motion_sensor: Option<MotionSensorComponent>,
    pub water_detector: Option<WaterDetectorComponent>,
    pub smoke_detector: Option<SmokeDetectorComponent>,
    pub oxygen_detector: Option<OxygenDetectorComponent>,
    pub delay: Option<DelayComponent>,
    pub not_component: Option<NotComponent>,
    pub and_component: Option<AndComponent>,
    pub or_component: Option<OrComponent>,
    pub greater_component: Option<GreaterComponent>,
    pub memory_component: Option<MemoryComponent>,
    pub adder_component: Option<AdderComponent>,
    pub divide_component: Option<DivideComponent>,
    pub subtract_component: Option<SubtractComponent>,
    pub color_component: Option<ColorComponent>,
    pub function_component: Option<FunctionComponent>,
    pub xor_component: Option<XorComponent>,
    pub equals_component: Option<EqualsComponent>,
    pub multiply_component: Option<MultiplyComponent>,
    pub demultiplexer_component: Option<DemultiplexerComponent>,
    pub concat_component: Option<ConcatComponent>,
    pub regex_find_component: Option<RegExFindComponent>,
    pub exponentiation_component: Option<ExponentiationComponent>,
    pub modulo_component: Option<ModuloComponent>,
    pub trigonometric_function_component: Option<TrigonometricFunctionComponent>,
    pub holdable: Option<HoldableComponent>,
    pub connection_panel: Option<ConnectionPannelComponent>,
    pub power_transfer: Option<PowerTransfer>,
    pub repairable: Option<RepairableComponent>,
    pub wire: Option<WireComponent>,
    pub terminal: Option<TerminalComponent>,
    pub power_container: Option<PowerContainerComponent>,
    pub item_container: Vec<ItemContainerComponent>,
    pub id_card: Option<IdCardComponent>,
    pub turret: Option<TurretComponent>,
    pub pump: Option<PumpComponent>,
    pub door: Option<DoorComponent>,
    pub controller: Option<ControllerComponent>,
    pub steering: Option<SteeringComponent>,
    pub sonar: Option<SonarComponent>,
    pub custom_interface: Option<CustomInterfaceComponent>,
    pub minimap: Option<MiniMapComponent>,
    pub reactor: Option<ReactorComponent>,
    pub ladder: Option<LadderComponent>,
    pub engine: Option<EngineComponent>,
    pub vent: Option<VentComponent>,
    pub oxygen_generator: Option<OxygenGeneratorComponent>,
    pub item_component: Option<ItemComponent>,
    pub fabricator: Option<FabricatorComponent>,
    pub deconstructor: Option<DeconstructorComponent>,
    pub electrical_discharger: Option<ElectricalDischargerComponent>,
    pub quality: Option<QualityComponent>,
    pub pickable: Option<PickableComponent>,
    pub projectile: Option<ProjectileComponent>,
    pub ranged_weapon: Option<RangedWeaponComponent>,
    pub wearable: Option<WearableComponent>,
    pub item_label: Option<ItemLabelComponent>,
    pub powered: Option<PoweredComponent>,
    pub melee_weapon: Option<MeleeWeaponComponent>,
    pub repair_tool: Vec<RepairToolComponent>,
    pub propulsion: Option<PropulsionComponent>,
    pub docking_port: Option<DockingPortComponent>,
    pub circuit_box: Option<CircuitBoxComponent>,
    pub sonar_transducer: Option<SonarTransducerComponent>,
    pub growable: Option<GrowableComponent>,
    pub entity_spawner: Option<EntitySpawnerComponent>,
    pub button_terminal: Option<ButtonTerminalComponent>,
    pub trigger: Vec<TriggerComponent>,
    pub outpost_terminal: Option<OutpostTerminalComponent>,
    pub genetic_material: Option<GeneticMaterialComponent>,
    pub level_resource: Option<LevelResourceComponent>,
    pub planter: Option<PlanterComponent>,
    pub throwable: Option<ThrowableComponent>,
    pub rope: Option<RopeComponent>,
    pub name_tag: Option<NameTagComponent>,
    pub status_hud: Option<StatusHUDComponent>,
    pub scanner: Option<ScannerComponent>,
}

impl ItemComponents {
    pub fn add_from_xml(&mut self, element: &Node) {
        let tag_name = element.tag_name().name();
        match tag_name {
            "RelayComponent" => {
                assert!(self.relay.is_none());
                self.relay = Some(RelayComponent::from_xml(element));
            }
            "OscillatorComponent" => {
                assert!(self.oscillator.is_none());
                self.oscillator = Some(OscillatorComponent::from_xml(element));
            }
            "LightComponent" => {
                self.light.push(LightComponent::from_xml(element));
            }
            "WifiComponent" => {
                assert!(self.wifi.is_none());
                self.wifi = Some(WifiComponent::from_xml(element));
            }
            "SignalCheckComponent" => {
                assert!(self.signal_check.is_none());
                self.signal_check = Some(SignalCheckComponent::from_xml(element));
            }
            "MotionSensor" => {
                assert!(self.motion_sensor.is_none());
                self.motion_sensor = Some(MotionSensorComponent::from_xml(element));
            }
            "WaterDetector" => {
                assert!(self.water_detector.is_none());
                self.water_detector = Some(WaterDetectorComponent::from_xml(element));
            }
            "SmokeDetector" => {
                assert!(self.smoke_detector.is_none());
                self.smoke_detector = Some(SmokeDetectorComponent::from_xml(element));
            }
            "OxygenDetector" => {
                assert!(self.oxygen_detector.is_none());
                self.oxygen_detector = Some(OxygenDetectorComponent::from_xml(element));
            }
            "DelayComponent" => {
                assert!(self.delay.is_none());
                self.delay = Some(DelayComponent::from_xml(element));
            }
            "NotComponent" => {
                assert!(self.not_component.is_none());
                self.not_component = Some(NotComponent::from_xml(element));
            }
            "AndComponent" => {
                assert!(self.and_component.is_none());
                self.and_component = Some(AndComponent::from_xml(element));
            }
            "OrComponent" => {
                assert!(self.or_component.is_none());
                self.or_component = Some(OrComponent::from_xml(element));
            }
            "GreaterComponent" => {
                assert!(self.greater_component.is_none());
                self.greater_component = Some(GreaterComponent::from_xml(element));
            }
            "MemoryComponent" => {
                assert!(self.memory_component.is_none());
                self.memory_component = Some(MemoryComponent::from_xml(element));
            }
            "AdderComponent" => {
                assert!(self.adder_component.is_none());
                self.adder_component = Some(AdderComponent::from_xml(element));
            }
            "DivideComponent" => {
                assert!(self.divide_component.is_none());
                self.divide_component = Some(DivideComponent::from_xml(element));
            }
            "SubtractComponent" => {
                assert!(self.subtract_component.is_none());
                self.subtract_component = Some(SubtractComponent::from_xml(element));
            }
            "ColorComponent" => {
                assert!(self.color_component.is_none());
                self.color_component = Some(ColorComponent::from_xml(element));
            }
            "FunctionComponent" => {
                assert!(self.function_component.is_none());
                self.function_component = Some(FunctionComponent::from_xml(element));
            }
            "XorComponent" => {
                assert!(self.xor_component.is_none());
                self.xor_component = Some(XorComponent::from_xml(element));
            }
            "EqualsComponent" => {
                assert!(self.equals_component.is_none());
                self.equals_component = Some(EqualsComponent::from_xml(element));
            }
            "MultiplyComponent" => {
                assert!(self.multiply_component.is_none());
                self.multiply_component = Some(MultiplyComponent::from_xml(element));
            }
            "DemultiplexerComponent" => {
                assert!(self.demultiplexer_component.is_none());
                self.demultiplexer_component = Some(DemultiplexerComponent::from_xml(element));
            }
            "ConcatComponent" => {
                assert!(self.concat_component.is_none());
                self.concat_component = Some(ConcatComponent::from_xml(element));
            }
            "RegExFindComponent" => {
                assert!(self.regex_find_component.is_none());
                self.regex_find_component = Some(RegExFindComponent::from_xml(element));
            }
            "ExponentiationComponent" => {
                assert!(self.exponentiation_component.is_none());
                self.exponentiation_component = Some(ExponentiationComponent::from_xml(element));
            }
            "ModuloComponent" => {
                assert!(self.modulo_component.is_none());
                self.modulo_component = Some(ModuloComponent::from_xml(element));
            }
            "TrigonometricFunctionComponent" => {
                assert!(self.trigonometric_function_component.is_none());
                self.trigonometric_function_component =
                    Some(TrigonometricFunctionComponent::from_xml(element));
            }
            "Holdable" => {
                assert!(self.holdable.is_none());
                self.holdable = Some(HoldableComponent::from_xml(element));
            }
            "ConnectionPanel" => {
                assert!(self.connection_panel.is_none());
                self.connection_panel = Some(ConnectionPannelComponent::from_xml(element));
            }
            "PowerTransfer" => {
                assert!(self.power_transfer.is_none());
                self.power_transfer = Some(PowerTransfer::from_xml(element));
            }
            "Repairable" => {
                assert!(self.repairable.is_none());
                self.repairable = Some(RepairableComponent::from_xml(element));
            }
            "Wire" => {
                assert!(self.wire.is_none());
                self.wire = Some(WireComponent::from_xml(element));
            }
            "Terminal" => {
                assert!(self.terminal.is_none());
                self.terminal = Some(TerminalComponent::from_xml(element));
            }
            "PowerContainer" => {
                assert!(self.power_container.is_none());
                self.power_container = Some(PowerContainerComponent::from_xml(element));
            }
            "ItemContainer" => {
                self.item_container
                    .push(ItemContainerComponent::from_xml(element));
            }
            "IdCard" => {
                assert!(self.id_card.is_none());
                self.id_card = Some(IdCardComponent::from_xml(element));
            }
            "Turret" => {
                assert!(self.turret.is_none());
                self.turret = Some(TurretComponent::from_xml(element));
            }
            "Pump" => {
                assert!(self.pump.is_none());
                self.pump = Some(PumpComponent::from_xml(element));
            }
            "Door" => {
                assert!(self.door.is_none());
                self.door = Some(DoorComponent::from_xml(element));
            }
            "Controller" => {
                assert!(self.controller.is_none());
                self.controller = Some(ControllerComponent::from_xml(element));
            }
            "Steering" => {
                assert!(self.steering.is_none());
                self.steering = Some(SteeringComponent::from_xml(element));
            }
            "Sonar" => {
                assert!(self.sonar.is_none());
                self.sonar = Some(SonarComponent::from_xml(element));
            }
            "CustomInterface" => {
                assert!(self.custom_interface.is_none());
                self.custom_interface = Some(CustomInterfaceComponent::from_xml(element));
            }
            "MiniMap" => {
                assert!(self.minimap.is_none());
                self.minimap = Some(MiniMapComponent::from_xml(element));
            }
            "Reactor" => {
                assert!(self.reactor.is_none());
                self.reactor = Some(ReactorComponent::from_xml(element));
            }
            "Ladder" => {
                assert!(self.ladder.is_none());
                self.ladder = Some(LadderComponent::from_xml(element));
            }
            "Engine" => {
                assert!(self.engine.is_none());
                self.engine = Some(EngineComponent::from_xml(element));
            }
            "Vent" => {
                assert!(self.vent.is_none());
                self.vent = Some(VentComponent::from_xml(element));
            }
            "OxygenGenerator" => {
                assert!(self.oxygen_generator.is_none());
                self.oxygen_generator = Some(OxygenGeneratorComponent::from_xml(element));
            }
            "ItemComponent" => {
                assert!(self.item_component.is_none());
                self.item_component = Some(ItemComponent::from_xml(element));
            }
            "Fabricator" => {
                assert!(self.fabricator.is_none());
                self.fabricator = Some(FabricatorComponent::from_xml(element));
            }
            "Deconstructor" => {
                assert!(self.deconstructor.is_none());
                self.deconstructor = Some(DeconstructorComponent::from_xml(element));
            }
            "ElectricalDischarger" => {
                assert!(self.electrical_discharger.is_none());
                self.electrical_discharger = Some(ElectricalDischargerComponent::from_xml(element));
            }
            "Quality" => {
                assert!(self.quality.is_none());
                self.quality = Some(QualityComponent::from_xml(element));
            }
            "Pickable" => {
                assert!(self.pickable.is_none());
                self.pickable = Some(PickableComponent::from_xml(element));
            }
            "Projectile" => {
                assert!(self.projectile.is_none());
                self.projectile = Some(ProjectileComponent::from_xml(element));
            }
            "RangedWeapon" => {
                assert!(self.ranged_weapon.is_none());
                self.ranged_weapon = Some(RangedWeaponComponent::from_xml(element));
            }
            "Wearable" => {
                assert!(self.wearable.is_none());
                self.wearable = Some(WearableComponent::from_xml(element));
            }
            "ItemLabel" => {
                assert!(self.item_label.is_none());
                self.item_label = Some(ItemLabelComponent::from_xml(element));
            }
            "Powered" => {
                assert!(self.powered.is_none());
                self.powered = Some(PoweredComponent::from_xml(element));
            }
            "MeleeWeapon" => {
                assert!(self.melee_weapon.is_none());
                self.melee_weapon = Some(MeleeWeaponComponent::from_xml(element));
            }
            "RepairTool" => {
                self.repair_tool
                    .push(RepairToolComponent::from_xml(element));
            }
            "Propulsion" => {
                assert!(self.propulsion.is_none());
                self.propulsion = Some(PropulsionComponent::from_xml(element));
            }
            "DockingPort" => {
                assert!(self.docking_port.is_none());
                self.docking_port = Some(DockingPortComponent::from_xml(element));
            }
            "CircuitBox" => {
                assert!(self.circuit_box.is_none());
                self.circuit_box = Some(CircuitBoxComponent::from_xml(element));
            }
            "SonarTransducer" => {
                assert!(self.sonar_transducer.is_none());
                self.sonar_transducer = Some(SonarTransducerComponent::from_xml(element));
            }
            "Growable" => {
                assert!(self.growable.is_none());
                self.growable = Some(GrowableComponent::from_xml(element));
            }
            "EntitySpawnerComponent" => {
                assert!(self.entity_spawner.is_none());
                self.entity_spawner = Some(EntitySpawnerComponent::from_xml(element));
            }
            "ButtonTerminal" => {
                assert!(self.button_terminal.is_none());
                self.button_terminal = Some(ButtonTerminalComponent::from_xml(element));
            }
            "TriggerComponent" => {
                self.trigger.push(TriggerComponent::from_xml(element));
            }
            "OutpostTerminal" => {
                assert!(self.outpost_terminal.is_none());
                self.outpost_terminal = Some(OutpostTerminalComponent::from_xml(element));
            }
            "GeneticMaterial" => {
                assert!(self.genetic_material.is_none());
                self.genetic_material = Some(GeneticMaterialComponent::from_xml(element));
            }
            "LevelResource" => {
                assert!(self.level_resource.is_none());
                self.level_resource = Some(LevelResourceComponent::from_xml(element));
            }
            "Planter" => {
                assert!(self.planter.is_none());
                self.planter = Some(PlanterComponent::from_xml(element));
            }
            "Throwable" => {
                assert!(self.throwable.is_none());
                self.throwable = Some(ThrowableComponent::from_xml(element));
            }
            "Rope" => {
                assert!(self.rope.is_none());
                self.rope = Some(RopeComponent::from_xml(element));
            }
            "NameTag" => {
                assert!(self.name_tag.is_none());
                self.name_tag = Some(NameTagComponent::from_xml(element));
            }
            "StatusHUD" => {
                assert!(self.status_hud.is_none());
                self.status_hud = Some(StatusHUDComponent::from_xml(element));
            }
            "Scanner" => {
                assert!(self.scanner.is_none());
                self.scanner = Some(ScannerComponent::from_xml(element));
            }
            _ => {
                todo!("Unimplemented ItemComponent encountered: {}", tag_name);
            }
        }
    }
}

#[derive(Debug)]
pub struct ItemComponent {
    pub inherit_parent_is_active: bool,
    pub picking_time: f32,
    pub picking_msg: Option<String>,
    pub is_active_conditional_comparison: LogicalOperatorType,
    pub can_be_picked: bool,
    pub draw_hud_when_equipped: bool,
    pub lock_gui_frame_position: bool,
    pub gui_frame_offset: Option<Point>,
    pub can_be_selected: bool,
    pub can_be_combined: bool,
    pub remove_on_combined: bool,
    pub character_usable: bool,
    pub allow_in_game_editing: bool,
    pub delete_on_use: bool,
    pub msg: String,
    pub combat_priority: f32,
    pub manually_selected_sound: Option<u32>,
}

impl ItemComponent {
    pub fn from_xml(element: &Node) -> Self {
        //TODO: children

        Self {
            inherit_parent_is_active: element
                .attribute_ignore_ascii_case("inheritparentisactive")
                .map_or(true, |v| v.parse().unwrap()),
            picking_time: element
                .attribute_ignore_ascii_case("pickingtime")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            picking_msg: element
                .attribute_ignore_ascii_case("pickingmsg")
                .map(|v| v.to_owned()),
            is_active_conditional_comparison: element
                .attribute_ignore_ascii_case("isactiveconditionalcomparison")
                .map_or(LogicalOperatorType::And, |v| v.parse().unwrap()),
            can_be_picked: element
                .attribute_ignore_ascii_case("canbepicked")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            draw_hud_when_equipped: element
                .attribute_ignore_ascii_case("drawhudwhenequipped")
                .map_or(false, |v| v.parse().unwrap()),
            lock_gui_frame_position: element
                .attribute_ignore_ascii_case("lockguiframeposition")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            gui_frame_offset: element
                .attribute_ignore_ascii_case("guiframeoffset")
                .map(|v| v.parse().unwrap()),
            can_be_selected: element
                .attribute_ignore_ascii_case("canbeselected")
                .map_or(false, |v| v.parse().unwrap()),
            can_be_combined: element
                .attribute_ignore_ascii_case("canbecombined")
                .map_or(false, |v| v.parse().unwrap()),
            remove_on_combined: element
                .attribute_ignore_ascii_case("removeoncombined")
                .map_or(false, |v| v.parse().unwrap()),
            character_usable: element
                .attribute_ignore_ascii_case("characterusable")
                .map_or(false, |v| v.parse().unwrap()),
            allow_in_game_editing: element
                .attribute_ignore_ascii_case("allowingameediting")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            delete_on_use: element
                .attribute_ignore_ascii_case("deleteonuse")
                .map_or(false, |v| v.parse().unwrap()),
            msg: element
                .attribute_ignore_ascii_case("msg")
                .unwrap()
                .to_owned(),
            combat_priority: element
                .attribute_ignore_ascii_case("combatpriority")
                .map_or(0.0, |v| v.parse().unwrap()),
            manually_selected_sound: element
                .attribute_ignore_ascii_case("manuallyselectedsound")
                .map(|v| v.parse().unwrap()),
        }
    }
}
