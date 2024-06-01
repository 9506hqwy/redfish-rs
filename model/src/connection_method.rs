pub type ConnectionMethod = crate::connection_method::v1_1_0::ConnectionMethod;
pub mod v1_1_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::connection_method::v1_1_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectionMethod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::connection_method::v1_1_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectionMethodType"
        )]
        pub connection_method_type: Option<crate::connection_method::v1_1_0::ConnectionMethodType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectionMethodVariant"
        )]
        pub connection_method_variant: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::connection_method::v1_1_0::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "TunnelingProtocol")]
        pub tunneling_protocol: Option<crate::connection_method::v1_1_0::TunnelingProtocolType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionMethodType {
        #[default]
        #[serde(rename = "IPMI15")]
        IPMI15,
        #[serde(rename = "IPMI20")]
        IPMI20,
        #[serde(rename = "NETCONF")]
        NETCONF,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Redfish")]
        Redfish,
        #[serde(rename = "SNMP")]
        SNMP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AggregationSources")]
        pub aggregation_sources: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AggregationSources@odata.count"
        )]
        pub aggregation_sources_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TunnelingProtocolType {
        #[default]
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SSH")]
        SSH,
    }
}
