use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum FabricAdapter {
    V010501(crate::fabric_adapter::v1_5_1::FabricAdapter),
    V010401(crate::fabric_adapter::v1_4_1::FabricAdapter),
    V010301(crate::fabric_adapter::v1_3_1::FabricAdapter),
    V010201(crate::fabric_adapter::v1_2_1::FabricAdapter),
    V010101(crate::fabric_adapter::v1_1_1::FabricAdapter),
    V010001(crate::fabric_adapter::v1_0_1::FabricAdapter),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::fabric_adapter::v1_0_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FabricAdapter {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::fabric_adapter::v1_0_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASICManufacturer")]
        pub asic_manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASICPartNumber")]
        pub asic_part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ASICRevisionIdentifier"
        )]
        pub asic_revision_identifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::fabric_adapter::v1_0_1::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::fabric_adapter::v1_0_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MSDT")]
        pub msdt: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PIDT")]
        pub pidt: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RequestorVCAT")]
        pub requestor_vcat: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResponderVCAT")]
        pub responder_vcat: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RITable")]
        pub ri_table: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSDT")]
        pub ssdt: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_1_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::fabric_adapter::v1_1_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FabricAdapter {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::fabric_adapter::v1_1_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASICManufacturer")]
        pub asic_manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASICPartNumber")]
        pub asic_part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ASICRevisionIdentifier"
        )]
        pub asic_revision_identifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::fabric_adapter::v1_1_1::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::fabric_adapter::v1_1_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MSDT")]
        pub msdt: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PIDT")]
        pub pidt: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RequestorVCAT")]
        pub requestor_vcat: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResponderVCAT")]
        pub responder_vcat: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RITable")]
        pub ri_table: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSDT")]
        pub ssdt: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_2_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::fabric_adapter::v1_2_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FabricAdapter {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::fabric_adapter::v1_2_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASICManufacturer")]
        pub asic_manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASICPartNumber")]
        pub asic_part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ASICRevisionIdentifier"
        )]
        pub asic_revision_identifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::fabric_adapter::v1_2_1::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::fabric_adapter::v1_2_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MSDT")]
        pub msdt: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PIDT")]
        pub pidt: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RequestorVCAT")]
        pub requestor_vcat: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResponderVCAT")]
        pub responder_vcat: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RITable")]
        pub ri_table: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSDT")]
        pub ssdt: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_3_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::fabric_adapter::v1_3_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FabricAdapter {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::fabric_adapter::v1_3_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASICManufacturer")]
        pub asic_manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASICPartNumber")]
        pub asic_part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ASICRevisionIdentifier"
        )]
        pub asic_revision_identifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricType")]
        pub fabric_type: Option<crate::protocol::Protocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FabricTypeCapabilities"
        )]
        pub fabric_type_capabilities: Option<Vec<crate::protocol::Protocol>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::fabric_adapter::v1_3_1::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::fabric_adapter::v1_3_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MSDT")]
        pub msdt: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PIDT")]
        pub pidt: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RequestorVCAT")]
        pub requestor_vcat: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResponderVCAT")]
        pub responder_vcat: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RITable")]
        pub ri_table: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSDT")]
        pub ssdt: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryDomains@odata.count"
        )]
        pub memory_domains_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_4_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::fabric_adapter::v1_4_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FabricAdapter {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::fabric_adapter::v1_4_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASICManufacturer")]
        pub asic_manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASICPartNumber")]
        pub asic_part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ASICRevisionIdentifier"
        )]
        pub asic_revision_identifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricType")]
        pub fabric_type: Option<crate::protocol::Protocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FabricTypeCapabilities"
        )]
        pub fabric_type_capabilities: Option<Vec<crate::protocol::Protocol>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::fabric_adapter::v1_4_1::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::fabric_adapter::v1_4_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MSDT")]
        pub msdt: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PIDT")]
        pub pidt: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RequestorVCAT")]
        pub requestor_vcat: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResponderVCAT")]
        pub responder_vcat: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RITable")]
        pub ri_table: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSDT")]
        pub ssdt: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryDomains@odata.count"
        )]
        pub memory_domains_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_5_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::fabric_adapter::v1_5_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FabricAdapter {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::fabric_adapter::v1_5_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASICManufacturer")]
        pub asic_manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ASICPartNumber")]
        pub asic_part_number: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ASICRevisionIdentifier"
        )]
        pub asic_revision_identifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FabricType")]
        pub fabric_type: Option<crate::protocol::Protocol>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FabricTypeCapabilities"
        )]
        pub fabric_type_capabilities: Option<Vec<crate::protocol::Protocol>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::fabric_adapter::v1_5_1::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::fabric_adapter::v1_5_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MSDT")]
        pub msdt: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PIDT")]
        pub pidt: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RequestorVCAT")]
        pub requestor_vcat: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResponderVCAT")]
        pub responder_vcat: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RITable")]
        pub ri_table: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSDT")]
        pub ssdt: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemoryDomains")]
        pub memory_domains: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MemoryDomains@odata.count"
        )]
        pub memory_domains_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevices")]
        pub pcie_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevices@odata.count"
        )]
        pub pcie_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
