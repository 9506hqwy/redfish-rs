pub type AccelerationFunction = crate::acceleration_function::v1_0_4::AccelerationFunction;
pub mod v1_0_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccelerationFunction {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccelerationFunctionType"
        )]
        pub acceleration_function_type:
            Option<crate::acceleration_function::v1_0_3::AccelerationFunctionType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::acceleration_function::v1_0_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FpgaReconfigurationSlots"
        )]
        pub fpga_reconfiguration_slots: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::acceleration_function::v1_0_3::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerWatts")]
        pub power_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccelerationFunctionType {
        #[default]
        #[serde(rename = "AudioProcessing")]
        AudioProcessing,
        #[serde(rename = "Compression")]
        Compression,
        #[serde(rename = "Encryption")]
        Encryption,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PacketInspection")]
        PacketInspection,
        #[serde(rename = "PacketSwitch")]
        PacketSwitch,
        #[serde(rename = "Scheduler")]
        Scheduler,
        #[serde(rename = "VideoProcessing")]
        VideoProcessing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::acceleration_function::v1_0_3::OemActions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_0_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccelerationFunction {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccelerationFunctionType"
        )]
        pub acceleration_function_type:
            Option<crate::acceleration_function::v1_0_4::AccelerationFunctionType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::acceleration_function::v1_0_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FpgaReconfigurationSlots"
        )]
        pub fpga_reconfiguration_slots: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::acceleration_function::v1_0_4::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerWatts")]
        pub power_watts: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccelerationFunctionType {
        #[default]
        #[serde(rename = "AudioProcessing")]
        AudioProcessing,
        #[serde(rename = "Compression")]
        Compression,
        #[serde(rename = "Encryption")]
        Encryption,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "PacketInspection")]
        PacketInspection,
        #[serde(rename = "PacketSwitch")]
        PacketSwitch,
        #[serde(rename = "Scheduler")]
        Scheduler,
        #[serde(rename = "VideoProcessing")]
        VideoProcessing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::acceleration_function::v1_0_4::OemActions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
