pub type Chassis = crate::chassis::v1_27_0::Chassis;
pub mod v1_27_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Chassis.Reset")]
        pub chassis_reset: Option<crate::chassis::v1_27_0::Reset>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::chassis::v1_27_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Chassis {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::chassis::v1_27_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "ChassisType")]
        pub chassis_type: crate::chassis::v1_27_0::ChassisType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Controls")]
        pub controls: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DepthMm")]
        pub depth_mm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::chassis::v1_27_0::ChassisDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Doors")]
        pub doors: Option<crate::chassis::v1_27_0::Doors>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives")]
        pub drives: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ElectricalSourceManagerURIs"
        )]
        pub electrical_source_manager_ur_is: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ElectricalSourceNames"
        )]
        pub electrical_source_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentalClass")]
        pub environmental_class: Option<crate::chassis::v1_27_0::ChassisEnvironmentalClass>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricAdapters")]
        pub fabric_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HeatingCoolingEquipmentNames"
        )]
        pub heating_cooling_equipment_names: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HeatingCoolingManagerURIs"
        )]
        pub heating_cooling_manager_ur_is: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HeightMm")]
        pub height_mm: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotPluggable")]
        pub hot_pluggable: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::chassis::v1_27_0::ChassisIndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LeakDetectors")]
        pub leak_detectors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::chassis::v1_27_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogServices")]
        pub log_services: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPowerWatts")]
        pub max_power_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaControllers")]
        pub media_controllers: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPowerWatts")]
        pub min_power_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkAdapters")]
        pub network_adapters: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.context")]
        pub odata_context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.etag")]
        pub odata_etag: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(rename = "@odata.type")]
        pub odata_type: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeSlots")]
        pub pcie_slots: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSecurity")]
        pub physical_security: Option<crate::chassis::v1_27_0::PhysicalSecurity>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Power")]
        pub power: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::chassis::v1_27_0::ChassisPowerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSubsystem")]
        pub power_subsystem: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredByParent")]
        pub powered_by_parent: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Replaceable")]
        pub replaceable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sensors")]
        pub sensors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Thermal")]
        pub thermal: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ThermalDirection")]
        pub thermal_direction: Option<crate::chassis::v1_27_0::ChassisThermalDirection>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ThermalManagedByParent"
        )]
        pub thermal_managed_by_parent: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ThermalSubsystem")]
        pub thermal_subsystem: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrustedComponents")]
        pub trusted_components: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<crate::chassis::v1_27_0::ChassisUUID>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WeightKg")]
        pub weight_kg: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WidthMm")]
        pub width_mm: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ChassisDescription {
        V000001(crate::chassis::v1_27_0::ChassisDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ChassisDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ChassisDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ChassisEnvironmentalClass {
        V012700(crate::chassis::v1_27_0::EnvironmentalClass),
        V000001(crate::chassis::v1_27_0::ChassisEnvironmentalClassN1),
    }
    impl Default for ChassisEnvironmentalClass {
        fn default() -> Self {
            Self::V012700(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ChassisEnvironmentalClassN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ChassisIndicatorLED {
        V012700(crate::chassis::v1_27_0::IndicatorLED),
        V000001(crate::chassis::v1_27_0::ChassisIndicatorLEDN1),
    }
    impl Default for ChassisIndicatorLED {
        fn default() -> Self {
            Self::V012700(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ChassisIndicatorLEDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ChassisPowerState {
        V000001(crate::chassis::v1_27_0::ChassisPowerStateN1),
        ResourcePowerState(crate::resource::PowerState),
    }
    impl Default for ChassisPowerState {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ChassisPowerStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ChassisThermalDirection {
        V012700(crate::chassis::v1_27_0::ThermalDirection),
        V000001(crate::chassis::v1_27_0::ChassisThermalDirectionN1),
    }
    impl Default for ChassisThermalDirection {
        fn default() -> Self {
            Self::V012700(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ChassisThermalDirectionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ChassisType {
        #[default]
        #[serde(rename = "Blade")]
        Blade,
        #[serde(rename = "Card")]
        Card,
        #[serde(rename = "Cartridge")]
        Cartridge,
        #[serde(rename = "Component")]
        Component,
        #[serde(rename = "Drawer")]
        Drawer,
        #[serde(rename = "Enclosure")]
        Enclosure,
        #[serde(rename = "Expansion")]
        Expansion,
        #[serde(rename = "HeatExchanger")]
        HeatExchanger,
        #[serde(rename = "IPBasedDrive")]
        IPBasedDrive,
        #[serde(rename = "ImmersionTank")]
        ImmersionTank,
        #[serde(rename = "Module")]
        Module,
        #[serde(rename = "Other")]
        Other,
        #[serde(rename = "Pod")]
        Pod,
        #[serde(rename = "PowerStrip")]
        PowerStrip,
        #[serde(rename = "Rack")]
        Rack,
        #[serde(rename = "RackGroup")]
        RackGroup,
        #[serde(rename = "RackMount")]
        RackMount,
        #[serde(rename = "Row")]
        Row,
        #[serde(rename = "Shelf")]
        Shelf,
        #[serde(rename = "Sidecar")]
        Sidecar,
        #[serde(rename = "Sled")]
        Sled,
        #[serde(rename = "StandAlone")]
        StandAlone,
        #[serde(rename = "StorageEnclosure")]
        StorageEnclosure,
        #[serde(rename = "Zone")]
        Zone,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ChassisUUID {
        V000001(crate::chassis::v1_27_0::ChassisUUIDN1),
        ResourceUUID(String),
    }
    impl Default for ChassisUUID {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ChassisUUIDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Door {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DoorState")]
        pub door_state: Option<crate::chassis::v1_27_0::DoorDoorState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Locked")]
        pub locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DoorDoorState {
        V012700(crate::chassis::v1_27_0::DoorState),
        V000001(crate::chassis::v1_27_0::DoorDoorStateN1),
    }
    impl Default for DoorDoorState {
        fn default() -> Self {
            Self::V012700(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DoorDoorStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DoorState {
        #[default]
        #[serde(rename = "Closed")]
        Closed,
        #[serde(rename = "Locked")]
        Locked,
        #[serde(rename = "LockedAndOpen")]
        LockedAndOpen,
        #[serde(rename = "Open")]
        Open,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Doors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Front")]
        pub front: Option<crate::chassis::v1_27_0::DoorsFront>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rear")]
        pub rear: Option<crate::chassis::v1_27_0::DoorsRear>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DoorsFront {
        V012700(crate::chassis::v1_27_0::Door),
        V000001(crate::chassis::v1_27_0::DoorsFrontN1),
    }
    impl Default for DoorsFront {
        fn default() -> Self {
            Self::V012700(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DoorsFrontN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DoorsRear {
        V012700(crate::chassis::v1_27_0::Door),
        V000001(crate::chassis::v1_27_0::DoorsRearN1),
    }
    impl Default for DoorsRear {
        fn default() -> Self {
            Self::V012700(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DoorsRearN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnvironmentalClass {
        #[default]
        #[serde(rename = "A1")]
        A1,
        #[serde(rename = "A2")]
        A2,
        #[serde(rename = "A3")]
        A3,
        #[serde(rename = "A4")]
        A4,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IntrusionSensor {
        #[default]
        #[serde(rename = "HardwareIntrusion")]
        HardwareIntrusion,
        #[serde(rename = "Normal")]
        Normal,
        #[serde(rename = "TamperingDetected")]
        TamperingDetected,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IntrusionSensorReArm {
        #[default]
        #[serde(rename = "Automatic")]
        Automatic,
        #[serde(rename = "Manual")]
        Manual,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AutomationNodes")]
        pub automation_nodes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutomationNodes@odata.count"
        )]
        pub automation_nodes_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Cables")]
        pub cables: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Cables@odata.count")]
        pub cables_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedCoolingLoops"
        )]
        pub connected_cooling_loops: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedCoolingLoops@odata.count"
        )]
        pub connected_cooling_loops_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContainedBy")]
        pub contained_by: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Contains")]
        pub contains: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Contains@odata.count"
        )]
        pub contains_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CooledBy")]
        pub cooled_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CooledBy@odata.count"
        )]
        pub cooled_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolingUnits")]
        pub cooling_units: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CoolingUnits@odata.count"
        )]
        pub cooling_units_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives")]
        pub drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives@odata.count")]
        pub drives_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Facility")]
        pub facility: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans")]
        pub fans: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans@odata.count")]
        pub fans_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagersInChassis")]
        pub managers_in_chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagersInChassis@odata.count"
        )]
        pub managers_in_chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerDistribution")]
        pub power_distribution: Option<crate::chassis::v1_27_0::LinksPowerDistribution>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerOutlets")]
        pub power_outlets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOutlets@odata.count"
        )]
        pub power_outlets_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplies")]
        pub power_supplies: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerSupplies@odata.count"
        )]
        pub power_supplies_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweredBy")]
        pub powered_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweredBy@odata.count"
        )]
        pub powered_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Storage@odata.count"
        )]
        pub storage_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Switches")]
        pub switches: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Switches@odata.count"
        )]
        pub switches_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksPowerDistribution {
        V000001(crate::chassis::v1_27_0::LinksPowerDistributionN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksPowerDistribution {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksPowerDistributionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PhysicalSecurity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IntrusionSensor")]
        pub intrusion_sensor: Option<crate::chassis::v1_27_0::PhysicalSecurityIntrusionSensor>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IntrusionSensorNumber"
        )]
        pub intrusion_sensor_number: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IntrusionSensorReArm"
        )]
        pub intrusion_sensor_re_arm:
            Option<crate::chassis::v1_27_0::PhysicalSecurityIntrusionSensorReArm>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PhysicalSecurityIntrusionSensor {
        V012700(crate::chassis::v1_27_0::IntrusionSensor),
        V000001(crate::chassis::v1_27_0::PhysicalSecurityIntrusionSensorN1),
    }
    impl Default for PhysicalSecurityIntrusionSensor {
        fn default() -> Self {
            Self::V012700(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PhysicalSecurityIntrusionSensorN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PhysicalSecurityIntrusionSensorReArm {
        V012700(crate::chassis::v1_27_0::IntrusionSensorReArm),
        V000001(crate::chassis::v1_27_0::PhysicalSecurityIntrusionSensorReArmN1),
    }
    impl Default for PhysicalSecurityIntrusionSensorReArm {
        fn default() -> Self {
            Self::V012700(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PhysicalSecurityIntrusionSensorReArmN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Reset {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResetType")]
        pub reset_type: Option<crate::resource::ResetType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThermalDirection {
        #[default]
        #[serde(rename = "BackToFront")]
        BackToFront,
        #[serde(rename = "FrontToBack")]
        FrontToBack,
        #[serde(rename = "Sealed")]
        Sealed,
        #[serde(rename = "TopExhaust")]
        TopExhaust,
    }
}
