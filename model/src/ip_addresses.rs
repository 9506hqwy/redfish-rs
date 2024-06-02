pub type IPv4Address = crate::ip_addresses::v1_1_5::IPv4Address;
pub type IPv6Address = crate::ip_addresses::v1_1_5::IPv6Address;
pub type IPv6GatewayStaticAddress = crate::ip_addresses::v1_1_5::IPv6GatewayStaticAddress;
pub type IPv6StaticAddress = crate::ip_addresses::v1_1_5::IPv6StaticAddress;
pub mod v1_1_5 {
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
        pub address_origin: Option<crate::ip_addresses::v1_1_5::IPv4AddressAddressOrigin>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Gateway")]
        pub gateway: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SubnetMask")]
        pub subnet_mask: Option<crate::ip_addresses::v1_1_5::IPv4AddressSubnetMask>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum IPv4AddressAddressOrigin {
        V010105(crate::ip_addresses::v1_1_5::IPv4AddressOrigin),
        V000001(crate::ip_addresses::v1_1_5::IPv4AddressAddressOriginN1),
    }
    impl Default for IPv4AddressAddressOrigin {
        fn default() -> Self {
            Self::V010105(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IPv4AddressAddressOriginN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum IPv4AddressSubnetMask {
        V010105(String),
        V000001(crate::ip_addresses::v1_1_5::IPv4AddressSubnetMaskN1),
    }
    impl Default for IPv4AddressSubnetMask {
        fn default() -> Self {
            Self::V010105(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IPv4AddressSubnetMaskN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv6Address {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressOrigin")]
        pub address_origin: Option<crate::ip_addresses::v1_1_5::IPv6AddressAddressOrigin>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressState")]
        pub address_state: Option<crate::ip_addresses::v1_1_5::IPv6AddressAddressState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrefixLength")]
        pub prefix_length: Option<crate::ip_addresses::v1_1_5::IPv6AddressPrefixLength>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum IPv6AddressAddressOrigin {
        V010105(crate::ip_addresses::v1_1_5::IPv6AddressOrigin),
        V000001(crate::ip_addresses::v1_1_5::IPv6AddressAddressOriginN1),
    }
    impl Default for IPv6AddressAddressOrigin {
        fn default() -> Self {
            Self::V010105(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IPv6AddressAddressOriginN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum IPv6AddressAddressState {
        V010105(crate::ip_addresses::v1_1_5::AddressState),
        V000001(crate::ip_addresses::v1_1_5::IPv6AddressAddressStateN1),
    }
    impl Default for IPv6AddressAddressState {
        fn default() -> Self {
            Self::V010105(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IPv6AddressAddressStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum IPv6AddressPrefixLength {
        V010105(i64),
        V000001(crate::ip_addresses::v1_1_5::IPv6AddressPrefixLengthN1),
    }
    impl Default for IPv6AddressPrefixLength {
        fn default() -> Self {
            Self::V010105(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IPv6AddressPrefixLengthN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv6GatewayStaticAddress {
        #[serde(rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrefixLength")]
        pub prefix_length:
            Option<crate::ip_addresses::v1_1_5::IPv6GatewayStaticAddressPrefixLength>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum IPv6GatewayStaticAddressPrefixLength {
        V010105(i64),
        V000001(crate::ip_addresses::v1_1_5::IPv6GatewayStaticAddressPrefixLengthN1),
    }
    impl Default for IPv6GatewayStaticAddressPrefixLength {
        fn default() -> Self {
            Self::V010105(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IPv6GatewayStaticAddressPrefixLengthN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IPv6StaticAddress {
        #[serde(rename = "Address")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "PrefixLength")]
        pub prefix_length: crate::ip_addresses::v1_1_5::IPv6StaticAddressPrefixLength,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum IPv6StaticAddressPrefixLength {
        V010105(i64),
        V000001(crate::ip_addresses::v1_1_5::IPv6StaticAddressPrefixLengthN1),
    }
    impl Default for IPv6StaticAddressPrefixLength {
        fn default() -> Self {
            Self::V010105(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IPv6StaticAddressPrefixLengthN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
