use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IPv4Address {
    V010103(crate::ip_addresses::v1_1_3::IPv4Address),
    V010010(crate::ip_addresses::v1_0_10::IPv4Address),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IPv6Address {
    V010103(crate::ip_addresses::v1_1_3::IPv6Address),
    V010010(crate::ip_addresses::v1_0_10::IPv6Address),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IPv6GatewayStaticAddress {
    V010103(crate::ip_addresses::v1_1_3::IPv6GatewayStaticAddress),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IPv6StaticAddress {
    V010103(crate::ip_addresses::v1_1_3::IPv6StaticAddress),
    V010010(crate::ip_addresses::v1_0_10::IPv6StaticAddress),
}
pub mod v1_0_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AddressState {
        #[default]
        #[serde(rename = "Deprecated")]
        Deprecated,
        #[serde(rename = "Failed")]
        Failed,
        #[serde(rename = "Preferred")]
        Preferred,
        #[serde(rename = "Tentative")]
        Tentative,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv4Address {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressOrigin")]
        pub address_origin: Option<crate::ip_addresses::v1_0_10::IPv4AddressOrigin>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Gateway")]
        pub gateway: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubnetMask")]
        pub subnet_mask: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IPv4AddressOrigin {
        #[default]
        #[serde(rename = "BOOTP")]
        BOOTP,
        #[serde(rename = "DHCP")]
        DHCP,
        #[serde(rename = "IPv4LinkLocal")]
        IPv4LinkLocal,
        #[serde(rename = "Static")]
        Static,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv6Address {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressOrigin")]
        pub address_origin: Option<crate::ip_addresses::v1_0_10::IPv6AddressOrigin>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressState")]
        pub address_state: Option<crate::ip_addresses::v1_0_10::AddressState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrefixLength")]
        pub prefix_length: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IPv6AddressOrigin {
        #[default]
        #[serde(rename = "DHCPv6")]
        DHCPv6,
        #[serde(rename = "LinkLocal")]
        LinkLocal,
        #[serde(rename = "SLAAC")]
        SLAAC,
        #[serde(rename = "Static")]
        Static,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv6StaticAddress {
        #[serde(rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "PrefixLength")]
        pub prefix_length: Option<i64>,
    }
}
pub mod v1_1_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AddressState {
        #[default]
        #[serde(rename = "Deprecated")]
        Deprecated,
        #[serde(rename = "Failed")]
        Failed,
        #[serde(rename = "Preferred")]
        Preferred,
        #[serde(rename = "Tentative")]
        Tentative,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv4Address {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressOrigin")]
        pub address_origin: Option<crate::ip_addresses::v1_1_3::IPv4AddressOrigin>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Gateway")]
        pub gateway: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubnetMask")]
        pub subnet_mask: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IPv4AddressOrigin {
        #[default]
        #[serde(rename = "BOOTP")]
        BOOTP,
        #[serde(rename = "DHCP")]
        DHCP,
        #[serde(rename = "IPv4LinkLocal")]
        IPv4LinkLocal,
        #[serde(rename = "Static")]
        Static,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv6Address {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressOrigin")]
        pub address_origin: Option<crate::ip_addresses::v1_1_3::IPv6AddressOrigin>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressState")]
        pub address_state: Option<crate::ip_addresses::v1_1_3::AddressState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrefixLength")]
        pub prefix_length: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IPv6AddressOrigin {
        #[default]
        #[serde(rename = "DHCPv6")]
        DHCPv6,
        #[serde(rename = "LinkLocal")]
        LinkLocal,
        #[serde(rename = "SLAAC")]
        SLAAC,
        #[serde(rename = "Static")]
        Static,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv6GatewayStaticAddress {
        #[serde(rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrefixLength")]
        pub prefix_length: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv6StaticAddress {
        #[serde(rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "PrefixLength")]
        pub prefix_length: Option<i64>,
    }
}
