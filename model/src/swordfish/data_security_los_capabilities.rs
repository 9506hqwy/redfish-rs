use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum AntiVirusScanTrigger {
    #[default]
    #[serde(rename = "None")]
    None,
    #[serde(rename = "OnFirstRead")]
    OnFirstRead,
    #[serde(rename = "OnPatternUpdate")]
    OnPatternUpdate,
    #[serde(rename = "OnRename")]
    OnRename,
    #[serde(rename = "OnUpdate")]
    OnUpdate,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum AuthenticationType {
    #[default]
    #[serde(rename = "None")]
    None,
    #[serde(rename = "PKI")]
    PKI,
    #[serde(rename = "Password")]
    Password,
    #[serde(rename = "Ticket")]
    Ticket,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum DataSanitizationPolicy {
    #[default]
    #[serde(rename = "Clear")]
    Clear,
    #[serde(rename = "CryptographicErase")]
    CryptographicErase,
    #[serde(rename = "None")]
    None,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DataSecurityLoSCapabilities {
    V010200(crate::swordfish::data_security_los_capabilities::v1_2_0::DataSecurityLoSCapabilities),
    V010103(crate::swordfish::data_security_los_capabilities::v1_1_3::DataSecurityLoSCapabilities),
    V010002(crate::swordfish::data_security_los_capabilities::v1_0_2::DataSecurityLoSCapabilities),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum KeySize {
    #[default]
    #[serde(rename = "Bits_0")]
    BitsN0,
    #[serde(rename = "Bits_112")]
    BitsN112,
    #[serde(rename = "Bits_128")]
    BitsN128,
    #[serde(rename = "Bits_192")]
    BitsN192,
    #[serde(rename = "Bits_256")]
    BitsN256,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum SecureChannelProtocol {
    #[default]
    #[serde(rename = "IPsec")]
    IPsec,
    #[serde(rename = "None")]
    None,
    #[serde(rename = "RPCSEC_GSS")]
    RPCSECGSS,
    #[serde(rename = "TLS")]
    TLS,
}
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataSecurityLoSCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedAntivirusEngineProviders"
        )]
        pub supported_antivirus_engine_providers: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedAntivirusScanPolicies"
        )]
        pub supported_antivirus_scan_policies:
            Option<Vec<crate::swordfish::data_security_los_capabilities::AntiVirusScanTrigger>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedChannelEncryptionStrengths"
        )]
        pub supported_channel_encryption_strengths:
            Option<Vec<crate::swordfish::data_security_los_capabilities::KeySize>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedDataSanitizationPolicies"
        )]
        pub supported_data_sanitization_policies:
            Option<Vec<crate::swordfish::data_security_los_capabilities::DataSanitizationPolicy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedHostAuthenticationTypes"
        )]
        pub supported_host_authentication_types:
            Option<Vec<crate::swordfish::data_security_los_capabilities::AuthenticationType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService"
        )]
        pub supported_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService@odata.count"
        )]
        pub supported_lines_of_service_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedMediaEncryptionStrengths"
        )]
        pub supported_media_encryption_strengths:
            Option<Vec<crate::swordfish::data_security_los_capabilities::KeySize>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedSecureChannelProtocols"
        )]
        pub supported_secure_channel_protocols:
            Option<Vec<crate::swordfish::data_security_los_capabilities::SecureChannelProtocol>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedUserAuthenticationTypes"
        )]
        pub supported_user_authentication_types:
            Option<Vec<crate::swordfish::data_security_los_capabilities::AuthenticationType>>,
    }
}
pub mod v1_1_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::data_security_los_capabilities::v1_1_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataSecurityLoSCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::data_security_los_capabilities::v1_1_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedAntivirusEngineProviders"
        )]
        pub supported_antivirus_engine_providers: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedAntivirusScanPolicies"
        )]
        pub supported_antivirus_scan_policies:
            Option<Vec<crate::swordfish::data_security_los_capabilities::AntiVirusScanTrigger>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedChannelEncryptionStrengths"
        )]
        pub supported_channel_encryption_strengths:
            Option<Vec<crate::swordfish::data_security_los_capabilities::KeySize>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedDataSanitizationPolicies"
        )]
        pub supported_data_sanitization_policies:
            Option<Vec<crate::swordfish::data_security_los_capabilities::DataSanitizationPolicy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedHostAuthenticationTypes"
        )]
        pub supported_host_authentication_types:
            Option<Vec<crate::swordfish::data_security_los_capabilities::AuthenticationType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService"
        )]
        pub supported_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService@odata.count"
        )]
        pub supported_lines_of_service_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedMediaEncryptionStrengths"
        )]
        pub supported_media_encryption_strengths:
            Option<Vec<crate::swordfish::data_security_los_capabilities::KeySize>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedSecureChannelProtocols"
        )]
        pub supported_secure_channel_protocols:
            Option<Vec<crate::swordfish::data_security_los_capabilities::SecureChannelProtocol>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedUserAuthenticationTypes"
        )]
        pub supported_user_authentication_types:
            Option<Vec<crate::swordfish::data_security_los_capabilities::AuthenticationType>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::data_security_los_capabilities::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataSecurityLoSCapabilities {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::data_security_los_capabilities::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedAntivirusEngineProviders"
        )]
        pub supported_antivirus_engine_providers: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedAntivirusScanPolicies"
        )]
        pub supported_antivirus_scan_policies:
            Option<Vec<crate::swordfish::data_security_los_capabilities::AntiVirusScanTrigger>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedChannelEncryptionStrengths"
        )]
        pub supported_channel_encryption_strengths:
            Option<Vec<crate::swordfish::data_security_los_capabilities::KeySize>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedDataSanitizationPolicies"
        )]
        pub supported_data_sanitization_policies:
            Option<Vec<crate::swordfish::data_security_los_capabilities::DataSanitizationPolicy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedHostAuthenticationTypes"
        )]
        pub supported_host_authentication_types:
            Option<Vec<crate::swordfish::data_security_los_capabilities::AuthenticationType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService"
        )]
        pub supported_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedLinesOfService@odata.count"
        )]
        pub supported_lines_of_service_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedMediaEncryptionStrengths"
        )]
        pub supported_media_encryption_strengths:
            Option<Vec<crate::swordfish::data_security_los_capabilities::KeySize>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedSecureChannelProtocols"
        )]
        pub supported_secure_channel_protocols:
            Option<Vec<crate::swordfish::data_security_los_capabilities::SecureChannelProtocol>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedUserAuthenticationTypes"
        )]
        pub supported_user_authentication_types:
            Option<Vec<crate::swordfish::data_security_los_capabilities::AuthenticationType>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
