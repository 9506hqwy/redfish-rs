use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum VLAN {
    V010300(crate::vlan_network_interface::v1_3_0::VLAN),
    V010200(crate::vlan_network_interface::v1_2_0::VLAN),
    V010105(crate::vlan_network_interface::v1_1_5::VLAN),
    V010009(crate::vlan_network_interface::v1_0_9::VLAN),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum VLanNetworkInterface {
    V010300(crate::vlan_network_interface::v1_3_0::VLanNetworkInterface),
    V010200(crate::vlan_network_interface::v1_2_0::VLanNetworkInterface),
    V010105(crate::vlan_network_interface::v1_1_5::VLanNetworkInterface),
    V010009(crate::vlan_network_interface::v1_0_9::VLanNetworkInterface),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_9 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VLAN {
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANEnable")]
        pub vlan_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANId")]
        pub vlan_id: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VLanNetworkInterface {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANEnable")]
        pub vlan_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANId")]
        pub vlan_id: Option<i64>,
    }
}
pub mod v1_1_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::vlan_network_interface::v1_1_5::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VLAN {
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANEnable")]
        pub vlan_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANId")]
        pub vlan_id: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VLanNetworkInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::vlan_network_interface::v1_1_5::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANEnable")]
        pub vlan_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANId")]
        pub vlan_id: Option<i64>,
    }
}
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::vlan_network_interface::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VLAN {
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANEnable")]
        pub vlan_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANId")]
        pub vlan_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANPriority")]
        pub vlan_priority: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VLanNetworkInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::vlan_network_interface::v1_2_0::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANEnable")]
        pub vlan_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANId")]
        pub vlan_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANPriority")]
        pub vlan_priority: Option<i64>,
    }
}
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::vlan_network_interface::v1_3_0::OemActions>,
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
        pub vlan_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANPriority")]
        pub vlan_priority: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VLanNetworkInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::vlan_network_interface::v1_3_0::Actions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANEnable")]
        pub vlan_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANId")]
        pub vlan_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VLANPriority")]
        pub vlan_priority: Option<i64>,
    }
}
