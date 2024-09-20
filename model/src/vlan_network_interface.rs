pub type VLAN = crate::vlan_network_interface::v1_3_1::VLAN;
pub type VLanNetworkInterface = crate::vlan_network_interface::v1_3_1::VLanNetworkInterface;
pub mod v1_3_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::vlan_network_interface::v1_3_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VLAN {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Tagged")]
        pub tagged: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANEnable")]
        pub vlan_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANId")]
        pub vlan_id: Option<crate::vlan_network_interface::v1_3_1::VLANVLANId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANPriority")]
        pub vlan_priority: Option<crate::vlan_network_interface::v1_3_1::VLANVLANPriority>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VLANVLANId {
        V010301(i64),
        V000001(crate::vlan_network_interface::v1_3_1::VLANVLANIdN1),
    }
    impl Default for VLANVLANId {
        fn default() -> Self {
            Self::V010301(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VLANVLANIdN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VLANVLANPriority {
        V010301(i64),
        V000001(crate::vlan_network_interface::v1_3_1::VLANVLANPriorityN1),
    }
    impl Default for VLANVLANPriority {
        fn default() -> Self {
            Self::V010301(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VLANVLANPriorityN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VLanNetworkInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::vlan_network_interface::v1_3_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description:
            Option<crate::vlan_network_interface::v1_3_1::VLanNetworkInterfaceDescription>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANEnable")]
        pub vlan_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANId")]
        pub vlan_id: Option<crate::vlan_network_interface::v1_3_1::VLanNetworkInterfaceVLANId>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANPriority")]
        pub vlan_priority:
            Option<crate::vlan_network_interface::v1_3_1::VLanNetworkInterfaceVLANPriority>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VLanNetworkInterfaceDescription {
        V000001(crate::vlan_network_interface::v1_3_1::VLanNetworkInterfaceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for VLanNetworkInterfaceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VLanNetworkInterfaceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VLanNetworkInterfaceVLANId {
        V010301(i64),
        V000001(crate::vlan_network_interface::v1_3_1::VLanNetworkInterfaceVLANIdN1),
    }
    impl Default for VLanNetworkInterfaceVLANId {
        fn default() -> Self {
            Self::V010301(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VLanNetworkInterfaceVLANIdN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VLanNetworkInterfaceVLANPriority {
        V010301(i64),
        V000001(crate::vlan_network_interface::v1_3_1::VLanNetworkInterfaceVLANPriorityN1),
    }
    impl Default for VLanNetworkInterfaceVLANPriority {
        fn default() -> Self {
            Self::V010301(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VLanNetworkInterfaceVLANPriorityN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
