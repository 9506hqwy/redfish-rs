pub type ConnectionMethod = crate::connection_method::v1_2_0::ConnectionMethod;
pub mod v1_1_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::connection_method::v1_1_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectionMethod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::connection_method::v1_1_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectionMethodType"
        )]
        pub connection_method_type:
            Option<crate::connection_method::v1_1_1::ConnectionMethodConnectionMethodType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectionMethodVariant"
        )]
        pub connection_method_variant: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::connection_method::v1_1_1::ConnectionMethodDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::connection_method::v1_1_1::Links>,
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
        pub tunneling_protocol:
            Option<crate::connection_method::v1_1_1::ConnectionMethodTunnelingProtocol>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionMethodConnectionMethodType {
        V010101(crate::connection_method::v1_1_1::ConnectionMethodType),
        V000001(crate::connection_method::v1_1_1::ConnectionMethodConnectionMethodTypeN1),
    }
    impl Default for ConnectionMethodConnectionMethodType {
        fn default() -> Self {
            Self::V010101(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionMethodConnectionMethodTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionMethodDescription {
        V000001(crate::connection_method::v1_1_1::ConnectionMethodDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ConnectionMethodDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionMethodDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionMethodTunnelingProtocol {
        V010101(crate::connection_method::v1_1_1::TunnelingProtocolType),
        V000001(crate::connection_method::v1_1_1::ConnectionMethodTunnelingProtocolN1),
    }
    impl Default for ConnectionMethodTunnelingProtocol {
        fn default() -> Self {
            Self::V010101(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionMethodTunnelingProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::connection_method::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ConnectionMethod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::connection_method::v1_2_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectionMethodType"
        )]
        pub connection_method_type:
            Option<crate::connection_method::v1_2_0::ConnectionMethodConnectionMethodType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectionMethodVariant"
        )]
        pub connection_method_variant: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::connection_method::v1_2_0::ConnectionMethodDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::connection_method::v1_2_0::Links>,
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
        pub tunneling_protocol:
            Option<crate::connection_method::v1_2_0::ConnectionMethodTunnelingProtocol>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionMethodConnectionMethodType {
        V010200(crate::connection_method::v1_2_0::ConnectionMethodType),
        V000001(crate::connection_method::v1_2_0::ConnectionMethodConnectionMethodTypeN1),
    }
    impl Default for ConnectionMethodConnectionMethodType {
        fn default() -> Self {
            Self::V010200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionMethodConnectionMethodTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionMethodDescription {
        V000001(crate::connection_method::v1_2_0::ConnectionMethodDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ConnectionMethodDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionMethodDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ConnectionMethodTunnelingProtocol {
        V010200(crate::connection_method::v1_2_0::TunnelingProtocolType),
        V000001(crate::connection_method::v1_2_0::ConnectionMethodTunnelingProtocolN1),
    }
    impl Default for ConnectionMethodTunnelingProtocol {
        fn default() -> Self {
            Self::V010200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionMethodTunnelingProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectionMethodType {
        #[default]
        #[serde(rename = "IPMI15")]
        IPMI15,
        #[serde(rename = "IPMI20")]
        IPMI20,
        #[serde(rename = "ModbusSerial")]
        ModbusSerial,
        #[serde(rename = "ModbusTCP")]
        ModbusTCP,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialInterface")]
        pub serial_interface: Option<crate::odata_v4::IdRef>,
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
