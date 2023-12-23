pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::allow_deny::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AllowDeny {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::allow_deny::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowType")]
        pub allow_type: Option<crate::allow_deny::v1_0_0::AllowType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DestinationPortLower"
        )]
        pub destination_port_lower: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DestinationPortUpper"
        )]
        pub destination_port_upper: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Direction")]
        pub direction: Option<crate::allow_deny::v1_0_0::DataDirection>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IANAProtocolNumber")]
        pub iana_protocol_number: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPAddressLower")]
        pub ip_address_lower: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPAddressType")]
        pub ip_address_type: Option<crate::allow_deny::v1_0_0::IPAddressType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPAddressUpper")]
        pub ip_address_upper: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourcePortLower")]
        pub source_port_lower: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourcePortUpper")]
        pub source_port_upper: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StatefulSession")]
        pub stateful_session: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AllowType {
        #[default]
        #[serde(rename = "Allow")]
        Allow,
        #[serde(rename = "Deny")]
        Deny,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataDirection {
        #[default]
        #[serde(rename = "Egress")]
        Egress,
        #[serde(rename = "Ingress")]
        Ingress,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IPAddressType {
        #[default]
        #[serde(rename = "IPv4")]
        IPv4,
        #[serde(rename = "IPv6")]
        IPv6,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
