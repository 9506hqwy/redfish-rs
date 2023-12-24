use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Endpoint {
    V010800(crate::endpoint::v1_8_0::Endpoint),
    V010700(crate::endpoint::v1_7_0::Endpoint),
    V010602(crate::endpoint::v1_6_2::Endpoint),
    V010504(crate::endpoint::v1_5_4::Endpoint),
    V010406(crate::endpoint::v1_4_6::Endpoint),
    V010307(crate::endpoint::v1_3_7::Endpoint),
    V010208(crate::endpoint::v1_2_8::Endpoint),
    V010109(crate::endpoint::v1_1_9::Endpoint),
    V010012(crate::endpoint::v1_0_12::Endpoint),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_12 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::endpoint::v1_0_12::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectedEntity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityLink")]
        pub entity_link: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityPciId")]
        pub entity_pci_id: Option<crate::endpoint::v1_0_12::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityRole")]
        pub entity_role: Option<crate::endpoint::v1_0_12::EntityRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityType")]
        pub entity_type: Option<crate::endpoint::v1_0_12::EntityType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciClassCode")]
        pub pci_class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciFunctionNumber")]
        pub pci_function_number: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Endpoint {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::endpoint::v1_0_12::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedEntities")]
        pub connected_entities: Option<Vec<crate::endpoint::v1_0_12::ConnectedEntity>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointProtocol")]
        pub endpoint_protocol: Option<crate::protocol::Protocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostReservationMemoryBytes"
        )]
        pub host_reservation_memory_bytes: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::endpoint::v1_0_12::Links>,
        #[serde(rename = "Name")]
        pub name: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciId")]
        pub pci_id: Option<crate::endpoint::v1_0_12::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityRole {
        #[default]
        #[serde(rename = "Both")]
        Both,
        #[serde(rename = "Initiator")]
        Initiator,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityType {
        #[default]
        #[serde(rename = "Bridge")]
        Bridge,
        #[serde(rename = "DisplayController")]
        DisplayController,
        #[serde(rename = "Drive")]
        Drive,
        #[serde(rename = "NetworkController")]
        NetworkController,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "RootComplex")]
        RootComplex,
        #[serde(rename = "StorageExpander")]
        StorageExpander,
        #[serde(rename = "StorageInitiator")]
        StorageInitiator,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints"
        )]
        pub mutually_exclusive_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints@odata.count"
        )]
        pub mutually_exclusive_endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports@odata.count")]
        pub ports_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PciId {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceId")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemId")]
        pub subsystem_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorId")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorId")]
        pub vendor_id: Option<String>,
    }
}
pub mod v1_1_9 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::endpoint::v1_1_9::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectedEntity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityLink")]
        pub entity_link: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityPciId")]
        pub entity_pci_id: Option<crate::endpoint::v1_1_9::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityRole")]
        pub entity_role: Option<crate::endpoint::v1_1_9::EntityRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityType")]
        pub entity_type: Option<crate::endpoint::v1_1_9::EntityType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciClassCode")]
        pub pci_class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciFunctionNumber")]
        pub pci_function_number: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Endpoint {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::endpoint::v1_1_9::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedEntities")]
        pub connected_entities: Option<Vec<crate::endpoint::v1_1_9::ConnectedEntity>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointProtocol")]
        pub endpoint_protocol: Option<crate::protocol::Protocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostReservationMemoryBytes"
        )]
        pub host_reservation_memory_bytes: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPTransportDetails")]
        pub ip_transport_details: Option<Vec<crate::endpoint::v1_1_9::IPTransportDetails>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::endpoint::v1_1_9::Links>,
        #[serde(rename = "Name")]
        pub name: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciId")]
        pub pci_id: Option<crate::endpoint::v1_1_9::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityRole {
        #[default]
        #[serde(rename = "Both")]
        Both,
        #[serde(rename = "Initiator")]
        Initiator,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityType {
        #[default]
        #[serde(rename = "Bridge")]
        Bridge,
        #[serde(rename = "DisplayController")]
        DisplayController,
        #[serde(rename = "Drive")]
        Drive,
        #[serde(rename = "NetworkController")]
        NetworkController,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "RootComplex")]
        RootComplex,
        #[serde(rename = "StorageExpander")]
        StorageExpander,
        #[serde(rename = "StorageInitiator")]
        StorageInitiator,
        #[serde(rename = "Volume")]
        Volume,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPTransportDetails {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv4Address")]
        pub ipv4_address: Option<crate::ip_addresses::IPv4Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv6Address")]
        pub ipv6_address: Option<crate::ip_addresses::IPv6Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransportProtocol")]
        pub transport_protocol: Option<crate::protocol::Protocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints"
        )]
        pub mutually_exclusive_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints@odata.count"
        )]
        pub mutually_exclusive_endpoints_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction@odata.count"
        )]
        pub network_device_function_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports@odata.count")]
        pub ports_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PciId {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceId")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemId")]
        pub subsystem_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorId")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorId")]
        pub vendor_id: Option<String>,
    }
}
pub mod v1_2_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::endpoint::v1_2_8::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectedEntity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityLink")]
        pub entity_link: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityPciId")]
        pub entity_pci_id: Option<crate::endpoint::v1_2_8::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityRole")]
        pub entity_role: Option<crate::endpoint::v1_2_8::EntityRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityType")]
        pub entity_type: Option<crate::endpoint::v1_2_8::EntityType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciClassCode")]
        pub pci_class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciFunctionNumber")]
        pub pci_function_number: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Endpoint {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::endpoint::v1_2_8::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedEntities")]
        pub connected_entities: Option<Vec<crate::endpoint::v1_2_8::ConnectedEntity>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointProtocol")]
        pub endpoint_protocol: Option<crate::protocol::Protocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostReservationMemoryBytes"
        )]
        pub host_reservation_memory_bytes: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPTransportDetails")]
        pub ip_transport_details: Option<Vec<crate::endpoint::v1_2_8::IPTransportDetails>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::endpoint::v1_2_8::Links>,
        #[serde(rename = "Name")]
        pub name: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciId")]
        pub pci_id: Option<crate::endpoint::v1_2_8::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityRole {
        #[default]
        #[serde(rename = "Both")]
        Both,
        #[serde(rename = "Initiator")]
        Initiator,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityType {
        #[default]
        #[serde(rename = "Bridge")]
        Bridge,
        #[serde(rename = "DisplayController")]
        DisplayController,
        #[serde(rename = "Drive")]
        Drive,
        #[serde(rename = "NetworkController")]
        NetworkController,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "RootComplex")]
        RootComplex,
        #[serde(rename = "StorageExpander")]
        StorageExpander,
        #[serde(rename = "StorageInitiator")]
        StorageInitiator,
        #[serde(rename = "Volume")]
        Volume,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPTransportDetails {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv4Address")]
        pub ipv4_address: Option<crate::ip_addresses::IPv4Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv6Address")]
        pub ipv6_address: Option<crate::ip_addresses::IPv6Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransportProtocol")]
        pub transport_protocol: Option<crate::protocol::Protocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints"
        )]
        pub mutually_exclusive_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints@odata.count"
        )]
        pub mutually_exclusive_endpoints_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction@odata.count"
        )]
        pub network_device_function_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports@odata.count")]
        pub ports_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PciId {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassCode")]
        pub class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceId")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionNumber")]
        pub function_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemId")]
        pub subsystem_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorId")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorId")]
        pub vendor_id: Option<String>,
    }
}
pub mod v1_3_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::endpoint::v1_3_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectedEntity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityLink")]
        pub entity_link: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityPciId")]
        pub entity_pci_id: Option<crate::endpoint::v1_3_7::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityRole")]
        pub entity_role: Option<crate::endpoint::v1_3_7::EntityRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityType")]
        pub entity_type: Option<crate::endpoint::v1_3_7::EntityType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciClassCode")]
        pub pci_class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciFunctionNumber")]
        pub pci_function_number: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Endpoint {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::endpoint::v1_3_7::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedEntities")]
        pub connected_entities: Option<Vec<crate::endpoint::v1_3_7::ConnectedEntity>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointProtocol")]
        pub endpoint_protocol: Option<crate::protocol::Protocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostReservationMemoryBytes"
        )]
        pub host_reservation_memory_bytes: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPTransportDetails")]
        pub ip_transport_details: Option<Vec<crate::endpoint::v1_3_7::IPTransportDetails>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::endpoint::v1_3_7::Links>,
        #[serde(rename = "Name")]
        pub name: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciId")]
        pub pci_id: Option<crate::endpoint::v1_3_7::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityRole {
        #[default]
        #[serde(rename = "Both")]
        Both,
        #[serde(rename = "Initiator")]
        Initiator,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityType {
        #[default]
        #[serde(rename = "AccelerationFunction")]
        AccelerationFunction,
        #[serde(rename = "Bridge")]
        Bridge,
        #[serde(rename = "DisplayController")]
        DisplayController,
        #[serde(rename = "Drive")]
        Drive,
        #[serde(rename = "NetworkController")]
        NetworkController,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "RootComplex")]
        RootComplex,
        #[serde(rename = "StorageExpander")]
        StorageExpander,
        #[serde(rename = "StorageInitiator")]
        StorageInitiator,
        #[serde(rename = "Volume")]
        Volume,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPTransportDetails {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv4Address")]
        pub ipv4_address: Option<crate::ip_addresses::IPv4Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv6Address")]
        pub ipv6_address: Option<crate::ip_addresses::IPv6Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransportProtocol")]
        pub transport_protocol: Option<crate::protocol::Protocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints"
        )]
        pub mutually_exclusive_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints@odata.count"
        )]
        pub mutually_exclusive_endpoints_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction@odata.count"
        )]
        pub network_device_function_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports@odata.count")]
        pub ports_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PciId {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassCode")]
        pub class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceId")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionNumber")]
        pub function_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemId")]
        pub subsystem_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorId")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorId")]
        pub vendor_id: Option<String>,
    }
}
pub mod v1_4_6 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::endpoint::v1_4_6::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectedEntity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityLink")]
        pub entity_link: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityPciId")]
        pub entity_pci_id: Option<crate::endpoint::v1_4_6::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityRole")]
        pub entity_role: Option<crate::endpoint::v1_4_6::EntityRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityType")]
        pub entity_type: Option<crate::endpoint::v1_4_6::EntityType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::endpoint::v1_4_6::GenZ>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciClassCode")]
        pub pci_class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciFunctionNumber")]
        pub pci_function_number: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Endpoint {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::endpoint::v1_4_6::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedEntities")]
        pub connected_entities: Option<Vec<crate::endpoint::v1_4_6::ConnectedEntity>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointProtocol")]
        pub endpoint_protocol: Option<crate::protocol::Protocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostReservationMemoryBytes"
        )]
        pub host_reservation_memory_bytes: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPTransportDetails")]
        pub ip_transport_details: Option<Vec<crate::endpoint::v1_4_6::IPTransportDetails>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::endpoint::v1_4_6::Links>,
        #[serde(rename = "Name")]
        pub name: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciId")]
        pub pci_id: Option<crate::endpoint::v1_4_6::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityRole {
        #[default]
        #[serde(rename = "Both")]
        Both,
        #[serde(rename = "Initiator")]
        Initiator,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityType {
        #[default]
        #[serde(rename = "AccelerationFunction")]
        AccelerationFunction,
        #[serde(rename = "Bridge")]
        Bridge,
        #[serde(rename = "DisplayController")]
        DisplayController,
        #[serde(rename = "Drive")]
        Drive,
        #[serde(rename = "FabricBridge")]
        FabricBridge,
        #[serde(rename = "MediaController")]
        MediaController,
        #[serde(rename = "MemoryChunk")]
        MemoryChunk,
        #[serde(rename = "NetworkController")]
        NetworkController,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "RootComplex")]
        RootComplex,
        #[serde(rename = "StorageExpander")]
        StorageExpander,
        #[serde(rename = "StorageInitiator")]
        StorageInitiator,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "Volume")]
        Volume,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GCID {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CID")]
        pub cid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SID")]
        pub sid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessKey")]
        pub access_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GCID")]
        pub gcid: Option<crate::endpoint::v1_4_6::GCID>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionKey")]
        pub region_key: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPTransportDetails {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv4Address")]
        pub ipv4_address: Option<crate::ip_addresses::IPv4Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv6Address")]
        pub ipv6_address: Option<crate::ip_addresses::IPv6Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransportProtocol")]
        pub transport_protocol: Option<crate::protocol::Protocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressPools")]
        pub address_pools: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AddressPools@odata.count"
        )]
        pub address_pools_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedPorts")]
        pub connected_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedPorts@odata.count"
        )]
        pub connected_ports_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints"
        )]
        pub mutually_exclusive_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints@odata.count"
        )]
        pub mutually_exclusive_endpoints_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction@odata.count"
        )]
        pub network_device_function_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports@odata.count")]
        pub ports_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PciId {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassCode")]
        pub class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceId")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionNumber")]
        pub function_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemId")]
        pub subsystem_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorId")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorId")]
        pub vendor_id: Option<String>,
    }
}
pub mod v1_5_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::endpoint::v1_5_4::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectedEntity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityLink")]
        pub entity_link: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityPciId")]
        pub entity_pci_id: Option<crate::endpoint::v1_5_4::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityRole")]
        pub entity_role: Option<crate::endpoint::v1_5_4::EntityRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityType")]
        pub entity_type: Option<crate::endpoint::v1_5_4::EntityType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::endpoint::v1_5_4::GenZ>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciClassCode")]
        pub pci_class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciFunctionNumber")]
        pub pci_function_number: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Endpoint {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::endpoint::v1_5_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedEntities")]
        pub connected_entities: Option<Vec<crate::endpoint::v1_5_4::ConnectedEntity>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointProtocol")]
        pub endpoint_protocol: Option<crate::protocol::Protocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostReservationMemoryBytes"
        )]
        pub host_reservation_memory_bytes: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPTransportDetails")]
        pub ip_transport_details: Option<Vec<crate::endpoint::v1_5_4::IPTransportDetails>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::endpoint::v1_5_4::Links>,
        #[serde(rename = "Name")]
        pub name: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciId")]
        pub pci_id: Option<crate::endpoint::v1_5_4::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityRole {
        #[default]
        #[serde(rename = "Both")]
        Both,
        #[serde(rename = "Initiator")]
        Initiator,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityType {
        #[default]
        #[serde(rename = "AccelerationFunction")]
        AccelerationFunction,
        #[serde(rename = "Bridge")]
        Bridge,
        #[serde(rename = "DisplayController")]
        DisplayController,
        #[serde(rename = "Drive")]
        Drive,
        #[serde(rename = "FabricBridge")]
        FabricBridge,
        #[serde(rename = "Manager")]
        Manager,
        #[serde(rename = "MediaController")]
        MediaController,
        #[serde(rename = "MemoryChunk")]
        MemoryChunk,
        #[serde(rename = "NetworkController")]
        NetworkController,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "RootComplex")]
        RootComplex,
        #[serde(rename = "StorageExpander")]
        StorageExpander,
        #[serde(rename = "StorageInitiator")]
        StorageInitiator,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "Volume")]
        Volume,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GCID {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CID")]
        pub cid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SID")]
        pub sid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessKey")]
        pub access_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GCID")]
        pub gcid: Option<crate::endpoint::v1_5_4::GCID>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionKey")]
        pub region_key: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPTransportDetails {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv4Address")]
        pub ipv4_address: Option<crate::ip_addresses::IPv4Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv6Address")]
        pub ipv6_address: Option<crate::ip_addresses::IPv6Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransportProtocol")]
        pub transport_protocol: Option<crate::protocol::Protocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressPools")]
        pub address_pools: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AddressPools@odata.count"
        )]
        pub address_pools_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedPorts")]
        pub connected_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedPorts@odata.count"
        )]
        pub connected_ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Connections")]
        pub connections: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Connections@odata.count"
        )]
        pub connections_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints"
        )]
        pub mutually_exclusive_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints@odata.count"
        )]
        pub mutually_exclusive_endpoints_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction@odata.count"
        )]
        pub network_device_function_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports@odata.count")]
        pub ports_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PciId {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassCode")]
        pub class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceId")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionNumber")]
        pub function_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemId")]
        pub subsystem_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorId")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorId")]
        pub vendor_id: Option<String>,
    }
}
pub mod v1_6_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::endpoint::v1_6_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectedEntity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityLink")]
        pub entity_link: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityPciId")]
        pub entity_pci_id: Option<crate::endpoint::v1_6_2::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityRole")]
        pub entity_role: Option<crate::endpoint::v1_6_2::EntityRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityType")]
        pub entity_type: Option<crate::endpoint::v1_6_2::EntityType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::endpoint::v1_6_2::GenZ>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciClassCode")]
        pub pci_class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciFunctionNumber")]
        pub pci_function_number: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Endpoint {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::endpoint::v1_6_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedEntities")]
        pub connected_entities: Option<Vec<crate::endpoint::v1_6_2::ConnectedEntity>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointProtocol")]
        pub endpoint_protocol: Option<crate::protocol::Protocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostReservationMemoryBytes"
        )]
        pub host_reservation_memory_bytes: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPTransportDetails")]
        pub ip_transport_details: Option<Vec<crate::endpoint::v1_6_2::IPTransportDetails>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::endpoint::v1_6_2::Links>,
        #[serde(rename = "Name")]
        pub name: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciId")]
        pub pci_id: Option<crate::endpoint::v1_6_2::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityRole {
        #[default]
        #[serde(rename = "Both")]
        Both,
        #[serde(rename = "Initiator")]
        Initiator,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityType {
        #[default]
        #[serde(rename = "AccelerationFunction")]
        AccelerationFunction,
        #[serde(rename = "Bridge")]
        Bridge,
        #[serde(rename = "DisplayController")]
        DisplayController,
        #[serde(rename = "Drive")]
        Drive,
        #[serde(rename = "FabricBridge")]
        FabricBridge,
        #[serde(rename = "Manager")]
        Manager,
        #[serde(rename = "MediaController")]
        MediaController,
        #[serde(rename = "MemoryChunk")]
        MemoryChunk,
        #[serde(rename = "NetworkController")]
        NetworkController,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "RootComplex")]
        RootComplex,
        #[serde(rename = "StorageExpander")]
        StorageExpander,
        #[serde(rename = "StorageInitiator")]
        StorageInitiator,
        #[serde(rename = "StorageSubsystem")]
        StorageSubsystem,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "Volume")]
        Volume,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GCID {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CID")]
        pub cid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SID")]
        pub sid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessKey")]
        pub access_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GCID")]
        pub gcid: Option<crate::endpoint::v1_6_2::GCID>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionKey")]
        pub region_key: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPTransportDetails {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv4Address")]
        pub ipv4_address: Option<crate::ip_addresses::IPv4Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv6Address")]
        pub ipv6_address: Option<crate::ip_addresses::IPv6Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransportProtocol")]
        pub transport_protocol: Option<crate::protocol::Protocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressPools")]
        pub address_pools: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AddressPools@odata.count"
        )]
        pub address_pools_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedPorts")]
        pub connected_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedPorts@odata.count"
        )]
        pub connected_ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Connections")]
        pub connections: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Connections@odata.count"
        )]
        pub connections_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints"
        )]
        pub mutually_exclusive_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints@odata.count"
        )]
        pub mutually_exclusive_endpoints_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction@odata.count"
        )]
        pub network_device_function_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports@odata.count")]
        pub ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones")]
        pub zones: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones@odata.count")]
        pub zones_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PciId {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassCode")]
        pub class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceId")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionNumber")]
        pub function_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemId")]
        pub subsystem_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorId")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorId")]
        pub vendor_id: Option<String>,
    }
}
pub mod v1_7_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::endpoint::v1_7_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectedEntity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityLink")]
        pub entity_link: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityPciId")]
        pub entity_pci_id: Option<crate::endpoint::v1_7_0::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityRole")]
        pub entity_role: Option<crate::endpoint::v1_7_0::EntityRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityType")]
        pub entity_type: Option<crate::endpoint::v1_7_0::EntityType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::endpoint::v1_7_0::GenZ>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciClassCode")]
        pub pci_class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciFunctionNumber")]
        pub pci_function_number: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Endpoint {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::endpoint::v1_7_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedEntities")]
        pub connected_entities: Option<Vec<crate::endpoint::v1_7_0::ConnectedEntity>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointProtocol")]
        pub endpoint_protocol: Option<crate::protocol::Protocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostReservationMemoryBytes"
        )]
        pub host_reservation_memory_bytes: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPTransportDetails")]
        pub ip_transport_details: Option<Vec<crate::endpoint::v1_7_0::IPTransportDetails>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::endpoint::v1_7_0::Links>,
        #[serde(rename = "Name")]
        pub name: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciId")]
        pub pci_id: Option<crate::endpoint::v1_7_0::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityRole {
        #[default]
        #[serde(rename = "Both")]
        Both,
        #[serde(rename = "Initiator")]
        Initiator,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityType {
        #[default]
        #[serde(rename = "AccelerationFunction")]
        AccelerationFunction,
        #[serde(rename = "Bridge")]
        Bridge,
        #[serde(rename = "DisplayController")]
        DisplayController,
        #[serde(rename = "Drive")]
        Drive,
        #[serde(rename = "FabricBridge")]
        FabricBridge,
        #[serde(rename = "Manager")]
        Manager,
        #[serde(rename = "MediaController")]
        MediaController,
        #[serde(rename = "MemoryChunk")]
        MemoryChunk,
        #[serde(rename = "NetworkController")]
        NetworkController,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "RootComplex")]
        RootComplex,
        #[serde(rename = "StorageExpander")]
        StorageExpander,
        #[serde(rename = "StorageInitiator")]
        StorageInitiator,
        #[serde(rename = "StorageSubsystem")]
        StorageSubsystem,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "Volume")]
        Volume,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GCID {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CID")]
        pub cid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SID")]
        pub sid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessKey")]
        pub access_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GCID")]
        pub gcid: Option<crate::endpoint::v1_7_0::GCID>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionKey")]
        pub region_key: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPTransportDetails {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv4Address")]
        pub ipv4_address: Option<crate::ip_addresses::IPv4Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv6Address")]
        pub ipv6_address: Option<crate::ip_addresses::IPv6Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransportProtocol")]
        pub transport_protocol: Option<crate::protocol::Protocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressPools")]
        pub address_pools: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AddressPools@odata.count"
        )]
        pub address_pools_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedPorts")]
        pub connected_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedPorts@odata.count"
        )]
        pub connected_ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Connections")]
        pub connections: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Connections@odata.count"
        )]
        pub connections_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalPorts")]
        pub local_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocalPorts@odata.count"
        )]
        pub local_ports_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints"
        )]
        pub mutually_exclusive_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints@odata.count"
        )]
        pub mutually_exclusive_endpoints_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction@odata.count"
        )]
        pub network_device_function_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports@odata.count")]
        pub ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones")]
        pub zones: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones@odata.count")]
        pub zones_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PciId {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassCode")]
        pub class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceId")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionNumber")]
        pub function_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemId")]
        pub subsystem_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorId")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorId")]
        pub vendor_id: Option<String>,
    }
}
pub mod v1_8_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::endpoint::v1_8_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectedEntity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityLink")]
        pub entity_link: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityPciId")]
        pub entity_pci_id: Option<crate::endpoint::v1_8_0::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityRole")]
        pub entity_role: Option<crate::endpoint::v1_8_0::EntityRole>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntityType")]
        pub entity_type: Option<crate::endpoint::v1_8_0::EntityType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::endpoint::v1_8_0::GenZ>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciClassCode")]
        pub pci_class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciFunctionNumber")]
        pub pci_function_number: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Endpoint {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::endpoint::v1_8_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedEntities")]
        pub connected_entities: Option<Vec<crate::endpoint::v1_8_0::ConnectedEntity>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointProtocol")]
        pub endpoint_protocol: Option<crate::protocol::Protocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostReservationMemoryBytes"
        )]
        pub host_reservation_memory_bytes: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPTransportDetails")]
        pub ip_transport_details: Option<Vec<crate::endpoint::v1_8_0::IPTransportDetails>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::endpoint::v1_8_0::Links>,
        #[serde(rename = "Name")]
        pub name: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PciId")]
        pub pci_id: Option<crate::endpoint::v1_8_0::PciId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityRole {
        #[default]
        #[serde(rename = "Both")]
        Both,
        #[serde(rename = "Initiator")]
        Initiator,
        #[serde(rename = "Target")]
        Target,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EntityType {
        #[default]
        #[serde(rename = "AccelerationFunction")]
        AccelerationFunction,
        #[serde(rename = "Bridge")]
        Bridge,
        #[serde(rename = "CXLDevice")]
        CXLDevice,
        #[serde(rename = "DisplayController")]
        DisplayController,
        #[serde(rename = "Drive")]
        Drive,
        #[serde(rename = "FabricBridge")]
        FabricBridge,
        #[serde(rename = "Manager")]
        Manager,
        #[serde(rename = "MediaController")]
        MediaController,
        #[serde(rename = "Memory")]
        Memory,
        #[serde(rename = "MemoryChunk")]
        MemoryChunk,
        #[serde(rename = "NetworkController")]
        NetworkController,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "RootComplex")]
        RootComplex,
        #[serde(rename = "StorageExpander")]
        StorageExpander,
        #[serde(rename = "StorageInitiator")]
        StorageInitiator,
        #[serde(rename = "StorageSubsystem")]
        StorageSubsystem,
        #[serde(rename = "Switch")]
        Switch,
        #[serde(rename = "Volume")]
        Volume,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GCID {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CID")]
        pub cid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SID")]
        pub sid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessKey")]
        pub access_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GCID")]
        pub gcid: Option<crate::endpoint::v1_8_0::GCID>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RegionKey")]
        pub region_key: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPTransportDetails {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv4Address")]
        pub ipv4_address: Option<crate::ip_addresses::IPv4Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPv6Address")]
        pub ipv6_address: Option<crate::ip_addresses::IPv6Address>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransportProtocol")]
        pub transport_protocol: Option<crate::protocol::Protocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressPools")]
        pub address_pools: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AddressPools@odata.count"
        )]
        pub address_pools_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedPorts")]
        pub connected_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedPorts@odata.count"
        )]
        pub connected_ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Connections")]
        pub connections: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Connections@odata.count"
        )]
        pub connections_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalPorts")]
        pub local_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocalPorts@odata.count"
        )]
        pub local_ports_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints"
        )]
        pub mutually_exclusive_endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MutuallyExclusiveEndpoints@odata.count"
        )]
        pub mutually_exclusive_endpoints_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction"
        )]
        pub network_device_function: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunction@odata.count"
        )]
        pub network_device_function_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports@odata.count")]
        pub ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones")]
        pub zones: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Zones@odata.count")]
        pub zones_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PciId {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassCode")]
        pub class_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceId")]
        pub device_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FunctionNumber")]
        pub function_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemId")]
        pub subsystem_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubsystemVendorId")]
        pub subsystem_vendor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VendorId")]
        pub vendor_id: Option<String>,
    }
}
