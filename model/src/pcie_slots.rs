pub type PCIeSlots = crate::pcie_slots::v1_6_1::PCIeSlots;
pub mod v1_5_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_slots::v1_5_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeLinks {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevice")]
        pub pcie_device: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevice@odata.count"
        )]
        pub pcie_device_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeSlot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotPluggable")]
        pub hot_pluggable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lanes")]
        pub lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_slots::v1_5_0::PCIeLinks>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_device::PCIeTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SlotType")]
        pub slot_type: Option<crate::pcie_slots::v1_5_0::SlotTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeSlots {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_slots::v1_5_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slots")]
        pub slots: Option<Vec<crate::pcie_slots::v1_5_0::PCIeSlot>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SlotTypes {
        #[default]
        #[serde(rename = "FullLength")]
        FullLength,
        #[serde(rename = "HalfLength")]
        HalfLength,
        #[serde(rename = "LowProfile")]
        LowProfile,
        #[serde(rename = "M2")]
        M2,
        #[serde(rename = "Mini")]
        Mini,
        #[serde(rename = "OCP3Large")]
        OCP3Large,
        #[serde(rename = "OCP3Small")]
        OCP3Small,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "U2")]
        U2,
    }
}
pub mod v1_6_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::pcie_slots::v1_6_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeLinks {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeDevice")]
        pub pcie_device: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeDevice@odata.count"
        )]
        pub pcie_device_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Processors")]
        pub processors: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Processors@odata.count"
        )]
        pub processors_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeSlot {
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotPluggable")]
        pub hot_pluggable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lanes")]
        pub lanes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::pcie_slots::v1_6_1::PCIeLinks>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeType")]
        pub pcie_type: Option<crate::pcie_slots::v1_6_1::PCIeSlotPCIeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SlotType")]
        pub slot_type: Option<crate::pcie_slots::v1_6_1::PCIeSlotSlotType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PCIeSlotPCIeType {
        V000001(crate::pcie_slots::v1_6_1::PCIeSlotPCIeTypeN1),
        PCIeDevicePCIeTypes(crate::pcie_device::PCIeTypes),
    }
    impl Default for PCIeSlotPCIeType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PCIeSlotPCIeTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PCIeSlotSlotType {
        V010601(crate::pcie_slots::v1_6_1::SlotTypes),
        V000001(crate::pcie_slots::v1_6_1::PCIeSlotSlotTypeN1),
    }
    impl Default for PCIeSlotSlotType {
        fn default() -> Self {
            Self::V010601(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PCIeSlotSlotTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PCIeSlots {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::pcie_slots::v1_6_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::pcie_slots::v1_6_1::PCIeSlotsDescription>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Slots")]
        pub slots: Option<Vec<crate::pcie_slots::v1_6_1::PCIeSlot>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PCIeSlotsDescription {
        V000001(crate::pcie_slots::v1_6_1::PCIeSlotsDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for PCIeSlotsDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PCIeSlotsDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SlotTypes {
        #[default]
        #[serde(rename = "FullLength")]
        FullLength,
        #[serde(rename = "HalfLength")]
        HalfLength,
        #[serde(rename = "LowProfile")]
        LowProfile,
        #[serde(rename = "M2")]
        M2,
        #[serde(rename = "Mini")]
        Mini,
        #[serde(rename = "OCP3Large")]
        OCP3Large,
        #[serde(rename = "OCP3Small")]
        OCP3Small,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "U2")]
        U2,
    }
}
