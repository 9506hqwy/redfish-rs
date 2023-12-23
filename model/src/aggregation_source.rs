pub mod v1_3_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#AggregationSource.GenerateSSHIdentityKeyPair"
        )]
        pub aggregation_source_generate_ssh_identity_key_pair:
            Option<crate::aggregation_source::v1_3_1::GenerateSSHIdentityKeyPair>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#AggregationSource.RemoveSSHIdentityKeyPair"
        )]
        pub aggregation_source_remove_ssh_identity_key_pair:
            Option<crate::aggregation_source::v1_3_1::RemoveSSHIdentityKeyPair>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::aggregation_source::v1_3_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AggregationSource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::aggregation_source::v1_3_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AggregationType")]
        pub aggregation_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::aggregation_source::v1_3_1::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SNMP")]
        pub snmp: Option<crate::aggregation_source::v1_3_1::SNMPSettings>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSHSettings")]
        pub ssh_settings: Option<crate::aggregation_source::v1_3_1::SSHSettingsType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AggregationType {
        #[default]
        #[serde(rename = "Full")]
        Full,
        #[serde(rename = "NotificationsOnly")]
        NotificationsOnly,
    }
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenerateSSHIdentityKeyPair {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenerateSSHIdentityKeyPairRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Curve")]
        pub curve: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyLength")]
        pub key_length: Option<i64>,
        #[serde(rename = "KeyType")]
        pub key_type: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectionMethod")]
        pub connection_method: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourcesAccessed")]
        pub resources_accessed: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourcesAccessed@odata.count"
        )]
        pub resources_accessed_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveSSHIdentityKeyPair {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveSSHIdentityKeyPairRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPAuthenticationProtocols {
        #[default]
        #[serde(rename = "CommunityString")]
        CommunityString,
        #[serde(rename = "HMAC128_SHA224")]
        HMAC128SHA224,
        #[serde(rename = "HMAC192_SHA256")]
        HMAC192SHA256,
        #[serde(rename = "HMAC256_SHA384")]
        HMAC256SHA384,
        #[serde(rename = "HMAC384_SHA512")]
        HMAC384SHA512,
        #[serde(rename = "HMAC_MD5")]
        HMACMD5,
        #[serde(rename = "HMAC_SHA96")]
        HMACSHA96,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPEncryptionProtocols {
        #[default]
        #[serde(rename = "CBC_DES")]
        CBCDES,
        #[serde(rename = "CFB128_AES128")]
        CFB128AES128,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SNMPSettings {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationKey")]
        pub authentication_key: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationKeySet"
        )]
        pub authentication_key_set: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationProtocol"
        )]
        pub authentication_protocol: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKey")]
        pub encryption_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKeySet")]
        pub encryption_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionProtocol")]
        pub encryption_protocol: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrapCommunity")]
        pub trap_community: Option<String>,
    }
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SSHSettingsType {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PresentedPublicHostKey"
        )]
        pub presented_public_host_key: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PresentedPublicHostKeyTimestamp"
        )]
        pub presented_public_host_key_timestamp: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PublicIdentityKey")]
        pub public_identity_key: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrustedPublicHostKeys"
        )]
        pub trusted_public_host_keys: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UserAuthenticationMethod"
        )]
        pub user_authentication_method: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum UserAuthenticationMethod {
        #[default]
        #[serde(rename = "Password")]
        Password,
        #[serde(rename = "PublicKey")]
        PublicKey,
    }
}
