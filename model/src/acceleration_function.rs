pub type AccelerationFunction = crate::acceleration_function::v1_0_5::AccelerationFunction;
pub mod v1_0_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccelerationFunction {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccelerationFunctionType"
        )]
        pub acceleration_function_type: Option<
            crate::acceleration_function::v1_0_5::AccelerationFunctionAccelerationFunctionType,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::acceleration_function::v1_0_5::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description:
            Option<crate::acceleration_function::v1_0_5::AccelerationFunctionDescription>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FpgaReconfigurationSlots"
        )]
        pub fpga_reconfiguration_slots: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::acceleration_function::v1_0_5::Links>,
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
        pub uuid: Option<crate::acceleration_function::v1_0_5::AccelerationFunctionUUID>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccelerationFunctionAccelerationFunctionType {
        V010005(crate::acceleration_function::v1_0_5::AccelerationFunctionType),
        V000001(
            crate::acceleration_function::v1_0_5::AccelerationFunctionAccelerationFunctionTypeN1,
        ),
    }
    impl Default for AccelerationFunctionAccelerationFunctionType {
        fn default() -> Self {
            Self::V010005(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccelerationFunctionAccelerationFunctionTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccelerationFunctionDescription {
        V000001(crate::acceleration_function::v1_0_5::AccelerationFunctionDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for AccelerationFunctionDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccelerationFunctionDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccelerationFunctionUUID {
        V000001(crate::acceleration_function::v1_0_5::AccelerationFunctionUUIDN1),
        ResourceUUID(String),
    }
    impl Default for AccelerationFunctionUUID {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccelerationFunctionUUIDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::acceleration_function::v1_0_5::OemActions>,
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
