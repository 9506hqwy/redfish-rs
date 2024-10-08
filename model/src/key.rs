use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ECDSACurveType {
    #[default]
    #[serde(rename = "NISTB233")]
    NISTB233,
    #[serde(rename = "NISTB409")]
    NISTB409,
    #[serde(rename = "NISTK163")]
    NISTK163,
    #[serde(rename = "NISTK233")]
    NISTK233,
    #[serde(rename = "NISTK283")]
    NISTK283,
    #[serde(rename = "NISTK409")]
    NISTK409,
    #[serde(rename = "NISTP192")]
    NISTP192,
    #[serde(rename = "NISTP224")]
    NISTP224,
    #[serde(rename = "NISTP256")]
    NISTP256,
    #[serde(rename = "NISTP384")]
    NISTP384,
    #[serde(rename = "NISTP521")]
    NISTP521,
    #[serde(rename = "NISTT571")]
    NISTT571,
}
pub type Key = crate::key::v1_4_1::Key;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum SSHKeyType {
    #[default]
    #[serde(rename = "DSA")]
    DSA,
    #[serde(rename = "ECDSA")]
    ECDSA,
    #[serde(rename = "Ed25519")]
    Ed25519,
    #[serde(rename = "RSA")]
    RSA,
}
pub mod v1_4_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::key::v1_4_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Key {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::key::v1_4_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::key::v1_4_1::KeyDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyString")]
        pub key_string: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyType")]
        pub key_type: Option<crate::key::v1_4_1::KeyKeyType>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeoF")]
        pub nvme_of: Option<crate::key::v1_4_1::NVMeoF>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSH")]
        pub ssh: Option<crate::key::v1_4_1::SSHType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserDescription")]
        pub user_description: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum KeyDescription {
        V000001(crate::key::v1_4_1::KeyDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for KeyDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum KeyDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum KeyKeyType {
        V010401(crate::key::v1_4_1::KeyType),
        V000001(crate::key::v1_4_1::KeyKeyTypeN1),
    }
    impl Default for KeyKeyType {
        fn default() -> Self {
            Self::V010401(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum KeyKeyTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum KeyType {
        #[default]
        #[serde(rename = "NVMeoF")]
        NVMeoF,
        #[serde(rename = "SSH")]
        SSH,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeoF {
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostKeyId")]
        pub host_key_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NQN")]
        pub nqn: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OEMSecurityProtocolType"
        )]
        pub oem_security_protocol_type: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecureHashAllowList"
        )]
        pub secure_hash_allow_list: Option<Vec<crate::key::v1_4_1::NVMeoFSecureHashAllowList>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityProtocolType"
        )]
        pub security_protocol_type: Option<crate::key::v1_4_1::NVMeoFSecurityProtocolTypeAnony>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeoFSecureHashAllowList {
        V010401(crate::key::v1_4_1::NVMeoFSecureHashType),
        V000001(crate::key::v1_4_1::NVMeoFSecureHashAllowListN1),
    }
    impl Default for NVMeoFSecureHashAllowList {
        fn default() -> Self {
            Self::V010401(Default::default())
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
    pub enum NVMeoFSecurityProtocolTypeAnony {
        V010401(crate::key::v1_4_1::NVMeoFSecurityProtocolType),
        V000001(crate::key::v1_4_1::NVMeoFSecurityProtocolTypeN1),
    }
    impl Default for NVMeoFSecurityProtocolTypeAnony {
        fn default() -> Self {
            Self::V010401(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeoFSecurityProtocolTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SSHType {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
        pub comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fingerprint")]
        pub fingerprint: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemoteServerHostName"
        )]
        pub remote_server_host_name: Option<String>,
    }
}
