use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ResourceBlock {
    V010401(crate::resource_block::v1_4_1::ResourceBlock),
    V010305(crate::resource_block::v1_3_5::ResourceBlock),
    V010206(crate::resource_block::v1_2_6::ResourceBlock),
    V010107(crate::resource_block::v1_1_7::ResourceBlock),
    V010007(crate::resource_block::v1_0_7::ResourceBlock),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ResourceBlockLimits {
    V010401(crate::resource_block::v1_4_1::ResourceBlockLimits),
    V010305(crate::resource_block::v1_3_5::ResourceBlockLimits),
}
pub mod v1_0_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource_block::v1_0_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CompositionState {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "Composing")]
        Composing,
        #[serde(rename = "Failed")]
        Failed,
        #[serde(rename = "Unused")]
        Unused,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CompositionStatus {
        #[serde(rename = "CompositionState")]
        pub composition_state: Option<crate::resource_block::v1_0_7::CompositionState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reserved")]
        pub reserved: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones")]
        pub zones: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones@odata.count")]
        pub zones_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::resource_block::v1_0_7::Actions>,
        #[serde(rename = "CompositionStatus")]
        pub composition_status: crate::resource_block::v1_0_7::CompositionStatus,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EthernetInterfaces@odata.count"
        )]
        pub ethernet_interfaces_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::resource_block::v1_0_7::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory@odata.count")]
        pub memory_odata_count: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkInterfaces@odata.count"
        )]
        pub network_interfaces_odata_count: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
        #[serde(rename = "ResourceBlockType")]
        pub resource_block_type: Vec<crate::resource_block::v1_0_7::ResourceBlockType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SimpleStorage@odata.count"
        )]
        pub simple_storage_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Storage@odata.count"
        )]
        pub storage_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResourceBlockType {
        #[default]
        #[serde(rename = "Compute")]
        Compute,
        #[serde(rename = "ComputerSystem")]
        ComputerSystem,
        #[serde(rename = "Memory")]
        Memory,
        #[serde(rename = "Network")]
        Network,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "Storage")]
        Storage,
    }
}
pub mod v1_1_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource_block::v1_1_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CompositionState {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "ComposedAndAvailable")]
        ComposedAndAvailable,
        #[serde(rename = "Composing")]
        Composing,
        #[serde(rename = "Failed")]
        Failed,
        #[serde(rename = "Unused")]
        Unused,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CompositionStatus {
        #[serde(rename = "CompositionState")]
        pub composition_state: Option<crate::resource_block::v1_1_7::CompositionState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxCompositions")]
        pub max_compositions: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NumberOfCompositions"
        )]
        pub number_of_compositions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reserved")]
        pub reserved: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SharingCapable")]
        pub sharing_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SharingEnabled")]
        pub sharing_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones")]
        pub zones: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones@odata.count")]
        pub zones_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::resource_block::v1_1_7::Actions>,
        #[serde(rename = "CompositionStatus")]
        pub composition_status: crate::resource_block::v1_1_7::CompositionStatus,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EthernetInterfaces@odata.count"
        )]
        pub ethernet_interfaces_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::resource_block::v1_1_7::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory@odata.count")]
        pub memory_odata_count: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkInterfaces@odata.count"
        )]
        pub network_interfaces_odata_count: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
        #[serde(rename = "ResourceBlockType")]
        pub resource_block_type: Vec<crate::resource_block::v1_1_7::ResourceBlockType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SimpleStorage@odata.count"
        )]
        pub simple_storage_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Storage@odata.count"
        )]
        pub storage_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResourceBlockType {
        #[default]
        #[serde(rename = "Compute")]
        Compute,
        #[serde(rename = "ComputerSystem")]
        ComputerSystem,
        #[serde(rename = "Memory")]
        Memory,
        #[serde(rename = "Network")]
        Network,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "Storage")]
        Storage,
    }
}
pub mod v1_2_6 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource_block::v1_2_6::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CompositionState {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "ComposedAndAvailable")]
        ComposedAndAvailable,
        #[serde(rename = "Composing")]
        Composing,
        #[serde(rename = "Failed")]
        Failed,
        #[serde(rename = "Unavailable")]
        Unavailable,
        #[serde(rename = "Unused")]
        Unused,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CompositionStatus {
        #[serde(rename = "CompositionState")]
        pub composition_state: Option<crate::resource_block::v1_2_6::CompositionState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxCompositions")]
        pub max_compositions: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NumberOfCompositions"
        )]
        pub number_of_compositions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reserved")]
        pub reserved: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SharingCapable")]
        pub sharing_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SharingEnabled")]
        pub sharing_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones")]
        pub zones: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones@odata.count")]
        pub zones_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::resource_block::v1_2_6::Actions>,
        #[serde(rename = "CompositionStatus")]
        pub composition_status: crate::resource_block::v1_2_6::CompositionStatus,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EthernetInterfaces@odata.count"
        )]
        pub ethernet_interfaces_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::resource_block::v1_2_6::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory@odata.count")]
        pub memory_odata_count: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkInterfaces@odata.count"
        )]
        pub network_interfaces_odata_count: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
        #[serde(rename = "ResourceBlockType")]
        pub resource_block_type: Vec<crate::resource_block::v1_2_6::ResourceBlockType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SimpleStorage@odata.count"
        )]
        pub simple_storage_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Storage@odata.count"
        )]
        pub storage_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResourceBlockType {
        #[default]
        #[serde(rename = "Compute")]
        Compute,
        #[serde(rename = "ComputerSystem")]
        ComputerSystem,
        #[serde(rename = "Expansion")]
        Expansion,
        #[serde(rename = "Memory")]
        Memory,
        #[serde(rename = "Network")]
        Network,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "Storage")]
        Storage,
    }
}
pub mod v1_3_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource_block::v1_3_5::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CompositionState {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "ComposedAndAvailable")]
        ComposedAndAvailable,
        #[serde(rename = "Composing")]
        Composing,
        #[serde(rename = "Failed")]
        Failed,
        #[serde(rename = "Unavailable")]
        Unavailable,
        #[serde(rename = "Unused")]
        Unused,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CompositionStatus {
        #[serde(rename = "CompositionState")]
        pub composition_state: Option<crate::resource_block::v1_3_5::CompositionState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxCompositions")]
        pub max_compositions: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NumberOfCompositions"
        )]
        pub number_of_compositions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reserved")]
        pub reserved: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SharingCapable")]
        pub sharing_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SharingEnabled")]
        pub sharing_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones")]
        pub zones: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones@odata.count")]
        pub zones_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::resource_block::v1_3_5::Actions>,
        #[serde(rename = "CompositionStatus")]
        pub composition_status: crate::resource_block::v1_3_5::CompositionStatus,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives")]
        pub drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives@odata.count")]
        pub drives_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EthernetInterfaces@odata.count"
        )]
        pub ethernet_interfaces_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::resource_block::v1_3_5::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory@odata.count")]
        pub memory_odata_count: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkInterfaces@odata.count"
        )]
        pub network_interfaces_odata_count: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
        #[serde(rename = "ResourceBlockType")]
        pub resource_block_type: Vec<crate::resource_block::v1_3_5::ResourceBlockType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SimpleStorage@odata.count"
        )]
        pub simple_storage_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Storage@odata.count"
        )]
        pub storage_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceBlockLimits {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxCompute")]
        pub max_compute: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxComputerSystem")]
        pub max_computer_system: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxExpansion")]
        pub max_expansion: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxMemory")]
        pub max_memory: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxNetwork")]
        pub max_network: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxProcessor")]
        pub max_processor: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxStorage")]
        pub max_storage: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinCompute")]
        pub min_compute: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinComputerSystem")]
        pub min_computer_system: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinExpansion")]
        pub min_expansion: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinMemory")]
        pub min_memory: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinNetwork")]
        pub min_network: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinProcessor")]
        pub min_processor: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinStorage")]
        pub min_storage: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResourceBlockType {
        #[default]
        #[serde(rename = "Compute")]
        Compute,
        #[serde(rename = "ComputerSystem")]
        ComputerSystem,
        #[serde(rename = "Expansion")]
        Expansion,
        #[serde(rename = "Memory")]
        Memory,
        #[serde(rename = "Network")]
        Network,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "Storage")]
        Storage,
    }
}
pub mod v1_4_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource_block::v1_4_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CompositionState {
        #[default]
        #[serde(rename = "Composed")]
        Composed,
        #[serde(rename = "ComposedAndAvailable")]
        ComposedAndAvailable,
        #[serde(rename = "Composing")]
        Composing,
        #[serde(rename = "Failed")]
        Failed,
        #[serde(rename = "Unavailable")]
        Unavailable,
        #[serde(rename = "Unused")]
        Unused,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CompositionStatus {
        #[serde(rename = "CompositionState")]
        pub composition_state: Option<crate::resource_block::v1_4_1::CompositionState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxCompositions")]
        pub max_compositions: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NumberOfCompositions"
        )]
        pub number_of_compositions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reserved")]
        pub reserved: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SharingCapable")]
        pub sharing_capable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SharingEnabled")]
        pub sharing_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingResourceBlocks"
        )]
        pub consuming_resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingResourceBlocks@odata.count"
        )]
        pub consuming_resource_blocks_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingResourceBlocks"
        )]
        pub supplying_resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyingResourceBlocks@odata.count"
        )]
        pub supplying_resource_blocks_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones")]
        pub zones: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones@odata.count")]
        pub zones_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PoolType {
        #[default]
        #[serde(rename = "Active")]
        Active,
        #[serde(rename = "Free")]
        Free,
        #[serde(rename = "Unassigned")]
        Unassigned,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceBlock {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::resource_block::v1_4_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Client")]
        pub client: Option<String>,
        #[serde(rename = "CompositionStatus")]
        pub composition_status: crate::resource_block::v1_4_1::CompositionStatus,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ComputerSystems")]
        pub computer_systems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComputerSystems@odata.count"
        )]
        pub computer_systems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives")]
        pub drives: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Drives@odata.count")]
        pub drives_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EthernetInterfaces@odata.count"
        )]
        pub ethernet_interfaces_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::resource_block::v1_4_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory")]
        pub memory: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Memory@odata.count")]
        pub memory_odata_count: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkInterfaces")]
        pub network_interfaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkInterfaces@odata.count"
        )]
        pub network_interfaces_odata_count: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Pool")]
        pub pool: Option<crate::resource_block::v1_4_1::PoolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
        #[serde(rename = "ResourceBlockType")]
        pub resource_block_type: Vec<crate::resource_block::v1_4_1::ResourceBlockType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SimpleStorage")]
        pub simple_storage: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SimpleStorage@odata.count"
        )]
        pub simple_storage_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Storage")]
        pub storage: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Storage@odata.count"
        )]
        pub storage_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceBlockLimits {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxCompute")]
        pub max_compute: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxComputerSystem")]
        pub max_computer_system: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxExpansion")]
        pub max_expansion: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxMemory")]
        pub max_memory: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxNetwork")]
        pub max_network: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxProcessor")]
        pub max_processor: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxStorage")]
        pub max_storage: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinCompute")]
        pub min_compute: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinComputerSystem")]
        pub min_computer_system: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinExpansion")]
        pub min_expansion: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinMemory")]
        pub min_memory: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinNetwork")]
        pub min_network: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinProcessor")]
        pub min_processor: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinStorage")]
        pub min_storage: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResourceBlockType {
        #[default]
        #[serde(rename = "Compute")]
        Compute,
        #[serde(rename = "ComputerSystem")]
        ComputerSystem,
        #[serde(rename = "Expansion")]
        Expansion,
        #[serde(rename = "IndependentResource")]
        IndependentResource,
        #[serde(rename = "Memory")]
        Memory,
        #[serde(rename = "Network")]
        Network,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "Storage")]
        Storage,
    }
}
