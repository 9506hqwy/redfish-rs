pub mod v1_1_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::data_security_line_of_service::v1_1_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataSecurityLineOfService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::data_security_line_of_service::v1_1_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AntivirusEngineProvider"
        )]
        pub antivirus_engine_provider: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AntivirusScanPolicies"
        )]
        pub antivirus_scan_policies: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ChannelEncryptionStrength"
        )]
        pub channel_encryption_strength: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DataSanitizationPolicy"
        )]
        pub data_sanitization_policy: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostAuthenticationType"
        )]
        pub host_authentication_type: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MediaEncryptionStrength"
        )]
        pub media_encryption_strength: Option<String>,
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
            rename = "SecureChannelProtocol"
        )]
        pub secure_channel_protocol: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UserAuthenticationType"
        )]
        pub user_authentication_type: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
