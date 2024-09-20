pub type KeyPolicy = crate::key_policy::v1_0_1::KeyPolicy;
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::key_policy::v1_0_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct KeyPolicy {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::key_policy::v1_0_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::key_policy::v1_0_1::KeyPolicyDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsDefault")]
        pub is_default: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyPolicyType")]
        pub key_policy_type: Option<crate::key_policy::v1_0_1::KeyPolicyKeyPolicyType>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeoF")]
        pub nvme_of: Option<crate::key_policy::v1_0_1::NVMeoF>,
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
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum KeyPolicyDescription {
        V000001(crate::key_policy::v1_0_1::KeyPolicyDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for KeyPolicyDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum KeyPolicyDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum KeyPolicyKeyPolicyType {
        V010001(crate::key_policy::v1_0_1::KeyPolicyType),
        V000001(crate::key_policy::v1_0_1::KeyPolicyKeyPolicyTypeN1),
    }
    impl Default for KeyPolicyKeyPolicyType {
        fn default() -> Self {
            Self::V010001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum KeyPolicyKeyPolicyTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum KeyPolicyType {
        #[default]
        #[serde(rename = "NVMeoF")]
        NVMeoF,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeoF {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CipherSuiteAllowList"
        )]
        pub cipher_suite_allow_list:
            Option<Vec<crate::key_policy::v1_0_1::NVMeoFCipherSuiteAllowList>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHGroupAllowList")]
        pub dh_group_allow_list: Option<Vec<crate::key_policy::v1_0_1::NVMeoFDHGroupAllowList>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OEMSecurityProtocolAllowList"
        )]
        pub oem_security_protocol_allow_list: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecureHashAllowList"
        )]
        pub secure_hash_allow_list:
            Option<Vec<crate::key_policy::v1_0_1::NVMeoFSecureHashAllowList>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityProtocolAllowList"
        )]
        pub security_protocol_allow_list:
            Option<Vec<crate::key_policy::v1_0_1::NVMeoFSecurityProtocolAllowList>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityTransportAllowList"
        )]
        pub security_transport_allow_list:
            Option<Vec<crate::key_policy::v1_0_1::NVMeoFSecurityTransportAllowList>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeoFCipherSuiteAllowList {
        V010001(crate::key_policy::v1_0_1::NVMeoFCipherSuiteType),
        V000001(crate::key_policy::v1_0_1::NVMeoFCipherSuiteAllowListN1),
    }
    impl Default for NVMeoFCipherSuiteAllowList {
        fn default() -> Self {
            Self::V010001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeoFCipherSuiteAllowListN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeoFCipherSuiteType {
        #[default]
        #[serde(rename = "TLS_AES_128_GCM_SHA256")]
        TLSAESN128GCMSHA256,
        #[serde(rename = "TLS_AES_256_GCM_SHA384")]
        TLSAESN256GCMSHA384,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeoFDHGroupAllowList {
        V010001(crate::key_policy::v1_0_1::NVMeoFDHGroupType),
        V000001(crate::key_policy::v1_0_1::NVMeoFDHGroupAllowListN1),
    }
    impl Default for NVMeoFDHGroupAllowList {
        fn default() -> Self {
            Self::V010001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeoFDHGroupAllowListN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeoFDHGroupType {
        #[default]
        #[serde(rename = "FFDHE2048")]
        FFDHE2048,
        #[serde(rename = "FFDHE3072")]
        FFDHE3072,
        #[serde(rename = "FFDHE4096")]
        FFDHE4096,
        #[serde(rename = "FFDHE6144")]
        FFDHE6144,
        #[serde(rename = "FFDHE8192")]
        FFDHE8192,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeoFSecureHashAllowList {
        V010001(crate::key_policy::v1_0_1::NVMeoFSecureHashType),
        V000001(crate::key_policy::v1_0_1::NVMeoFSecureHashAllowListN1),
    }
    impl Default for NVMeoFSecureHashAllowList {
        fn default() -> Self {
            Self::V010001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeoFSecureHashAllowListN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeoFSecureHashType {
        #[default]
        #[serde(rename = "SHA256")]
        SHA256,
        #[serde(rename = "SHA384")]
        SHA384,
        #[serde(rename = "SHA512")]
        SHA512,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeoFSecurityProtocolAllowList {
        V010001(crate::key_policy::v1_0_1::NVMeoFSecurityProtocolType),
        V000001(crate::key_policy::v1_0_1::NVMeoFSecurityProtocolAllowListN1),
    }
    impl Default for NVMeoFSecurityProtocolAllowList {
        fn default() -> Self {
            Self::V010001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeoFSecurityProtocolAllowListN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeoFSecurityProtocolType {
        #[default]
        #[serde(rename = "DHHC")]
        DHHC,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "TLS_PSK")]
        TLSPSK,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeoFSecurityTransportAllowList {
        V010001(crate::key_policy::v1_0_1::NVMeoFSecurityTransportType),
        V000001(crate::key_policy::v1_0_1::NVMeoFSecurityTransportAllowListN1),
    }
    impl Default for NVMeoFSecurityTransportAllowList {
        fn default() -> Self {
            Self::V010001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeoFSecurityTransportAllowListN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeoFSecurityTransportType {
        #[default]
        #[serde(rename = "TLSv2")]
        TLSv2,
        #[serde(rename = "TLSv3")]
        TLSv3,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
