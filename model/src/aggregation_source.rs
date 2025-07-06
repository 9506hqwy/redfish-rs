pub type AggregationSource = crate::aggregation_source::v1_5_0::AggregationSource;
pub mod v1_4_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#AggregationSource.GenerateSSHIdentityKeyPair"
        )]
        pub aggregation_source_generate_ssh_identity_key_pair:
            Option<crate::aggregation_source::v1_4_2::GenerateSSHIdentityKeyPair>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#AggregationSource.RemoveSSHIdentityKeyPair"
        )]
        pub aggregation_source_remove_ssh_identity_key_pair:
            Option<crate::aggregation_source::v1_4_2::RemoveSSHIdentityKeyPair>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::aggregation_source::v1_4_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AggregationSource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::aggregation_source::v1_4_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AggregationType")]
        pub aggregation_type: Option<crate::aggregation_source::v1_4_2::AggregationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::aggregation_source::v1_4_2::AggregationSourceDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::aggregation_source::v1_4_2::Links>,
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
        pub snmp: Option<crate::aggregation_source::v1_4_2::SNMPSettings>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSHSettings")]
        pub ssh_settings: Option<crate::aggregation_source::v1_4_2::SSHSettingsType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AggregationSourceDescription {
        V000001(crate::aggregation_source::v1_4_2::AggregationSourceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for AggregationSourceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AggregationSourceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    pub struct GenerateSSHIdentityKeyPair {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenerateSSHIdentityKeyPairRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Curve")]
        pub curve: Option<crate::key::ECDSACurveType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyLength")]
        pub key_length: Option<i64>,
        #[serde(rename = "KeyType")]
        pub key_type: crate::key::SSHKeyType,
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
        #[serde(rename = "CFB128_AES192")]
        CFB128AES192,
        #[serde(rename = "CFB128_AES256")]
        CFB128AES256,
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
        pub authentication_protocol:
            Option<crate::aggregation_source::v1_4_2::SNMPSettingsAuthenticationProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKey")]
        pub encryption_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKeySet")]
        pub encryption_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionProtocol")]
        pub encryption_protocol:
            Option<crate::aggregation_source::v1_4_2::SNMPSettingsEncryptionProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrapCommunity")]
        pub trap_community: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SNMPSettingsAuthenticationProtocol {
        V010402(crate::aggregation_source::v1_4_2::SNMPAuthenticationProtocols),
        V000001(crate::aggregation_source::v1_4_2::SNMPSettingsAuthenticationProtocolN1),
    }
    impl Default for SNMPSettingsAuthenticationProtocol {
        fn default() -> Self {
            Self::V010402(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPSettingsAuthenticationProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SNMPSettingsEncryptionProtocol {
        V010402(crate::aggregation_source::v1_4_2::SNMPEncryptionProtocols),
        V000001(crate::aggregation_source::v1_4_2::SNMPSettingsEncryptionProtocolN1),
    }
    impl Default for SNMPSettingsEncryptionProtocol {
        fn default() -> Self {
            Self::V010402(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPSettingsEncryptionProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub user_authentication_method:
            Option<crate::aggregation_source::v1_4_2::SSHSettingsTypeUserAuthenticationMethod>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SSHSettingsTypeUserAuthenticationMethod {
        V010402(crate::aggregation_source::v1_4_2::UserAuthenticationMethod),
        V000001(crate::aggregation_source::v1_4_2::SSHSettingsTypeUserAuthenticationMethodN1),
    }
    impl Default for SSHSettingsTypeUserAuthenticationMethod {
        fn default() -> Self {
            Self::V010402(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SSHSettingsTypeUserAuthenticationMethodN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
pub mod v1_5_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#AggregationSource.GenerateSSHIdentityKeyPair"
        )]
        pub aggregation_source_generate_ssh_identity_key_pair:
            Option<crate::aggregation_source::v1_5_0::GenerateSSHIdentityKeyPair>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#AggregationSource.RemoveSSHIdentityKeyPair"
        )]
        pub aggregation_source_remove_ssh_identity_key_pair:
            Option<crate::aggregation_source::v1_5_0::RemoveSSHIdentityKeyPair>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::aggregation_source::v1_5_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AggregationSource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::aggregation_source::v1_5_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AggregationType")]
        pub aggregation_type: Option<crate::aggregation_source::v1_5_0::AggregationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::aggregation_source::v1_5_0::AggregationSourceDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::aggregation_source::v1_5_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ModbusTargetServerId"
        )]
        pub modbus_target_server_id: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SNMP")]
        pub snmp: Option<crate::aggregation_source::v1_5_0::SNMPSettings>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSHSettings")]
        pub ssh_settings: Option<crate::aggregation_source::v1_5_0::SSHSettingsType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AggregationSourceDescription {
        V000001(crate::aggregation_source::v1_5_0::AggregationSourceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for AggregationSourceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AggregationSourceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    pub struct GenerateSSHIdentityKeyPair {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenerateSSHIdentityKeyPairRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Curve")]
        pub curve: Option<crate::key::ECDSACurveType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyLength")]
        pub key_length: Option<i64>,
        #[serde(rename = "KeyType")]
        pub key_type: crate::key::SSHKeyType,
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
        #[serde(rename = "CFB128_AES192")]
        CFB128AES192,
        #[serde(rename = "CFB128_AES256")]
        CFB128AES256,
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
        pub authentication_protocol:
            Option<crate::aggregation_source::v1_5_0::SNMPSettingsAuthenticationProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKey")]
        pub encryption_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKeySet")]
        pub encryption_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionProtocol")]
        pub encryption_protocol:
            Option<crate::aggregation_source::v1_5_0::SNMPSettingsEncryptionProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TrapCommunity")]
        pub trap_community: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SNMPSettingsAuthenticationProtocol {
        V010500(crate::aggregation_source::v1_5_0::SNMPAuthenticationProtocols),
        V000001(crate::aggregation_source::v1_5_0::SNMPSettingsAuthenticationProtocolN1),
    }
    impl Default for SNMPSettingsAuthenticationProtocol {
        fn default() -> Self {
            Self::V010500(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPSettingsAuthenticationProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SNMPSettingsEncryptionProtocol {
        V010500(crate::aggregation_source::v1_5_0::SNMPEncryptionProtocols),
        V000001(crate::aggregation_source::v1_5_0::SNMPSettingsEncryptionProtocolN1),
    }
    impl Default for SNMPSettingsEncryptionProtocol {
        fn default() -> Self {
            Self::V010500(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPSettingsEncryptionProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub user_authentication_method:
            Option<crate::aggregation_source::v1_5_0::SSHSettingsTypeUserAuthenticationMethod>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SSHSettingsTypeUserAuthenticationMethod {
        V010500(crate::aggregation_source::v1_5_0::UserAuthenticationMethod),
        V000001(crate::aggregation_source::v1_5_0::SSHSettingsTypeUserAuthenticationMethodN1),
    }
    impl Default for SSHSettingsTypeUserAuthenticationMethod {
        fn default() -> Self {
            Self::V010500(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SSHSettingsTypeUserAuthenticationMethodN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
