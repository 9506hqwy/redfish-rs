pub type VirtualPCI2PCIBridge = crate::virtual_pc_i2_pci_bridge::v1_0_0::VirtualPCI2PCIBridge;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::virtual_pc_i2_pci_bridge::v1_0_0::OemActions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<crate::virtual_pc_i2_pci_bridge::v1_0_0::LinksPort>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksPort {
        V000001(crate::virtual_pc_i2_pci_bridge::v1_0_0::LinksPortN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksPort {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksPortN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VPPBStatusTypes {
        #[default]
        #[serde(rename = "BoundLD")]
        BoundLD,
        #[serde(rename = "BoundPID")]
        BoundPID,
        #[serde(rename = "BoundPhysicalPort")]
        BoundPhysicalPort,
        #[serde(rename = "Busy")]
        Busy,
        #[serde(rename = "Unbound")]
        Unbound,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualPCI2PCIBridge {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::virtual_pc_i2_pci_bridge::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BindingStatus")]
        pub binding_status: Option<crate::virtual_pc_i2_pci_bridge::v1_0_0::VPPBStatusTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BoundLDId")]
        pub bound_ld_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BoundPBRId")]
        pub bound_pbr_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BoundPortId")]
        pub bound_port_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description:
            Option<crate::virtual_pc_i2_pci_bridge::v1_0_0::VirtualPCI2PCIBridgeDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GCXLID")]
        pub gcxlid: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::virtual_pc_i2_pci_bridge::v1_0_0::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VPPBId")]
        pub vppb_id: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VirtualPCI2PCIBridgeDescription {
        V000001(crate::virtual_pc_i2_pci_bridge::v1_0_0::VirtualPCI2PCIBridgeDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for VirtualPCI2PCIBridgeDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VirtualPCI2PCIBridgeDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
