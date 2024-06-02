pub type DataSecurityLineOfService =
    crate::swordfish::data_security_line_of_service::v1_1_1::DataSecurityLineOfService;
pub mod v1_1_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::data_security_line_of_service::v1_1_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DataSecurityLineOfService { # [serde (skip_serializing_if = "Option::is_none" , rename = "Actions")] pub actions : Option < crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: Actions > , # [serde (skip_serializing_if = "Option::is_none" , rename = "AntivirusEngineProvider")] pub antivirus_engine_provider : Option < String > , # [serde (skip_serializing_if = "Option::is_none" , rename = "AntivirusScanPolicies")] pub antivirus_scan_policies : Option < Vec < crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceAntivirusScanPolicies > > , # [serde (skip_serializing_if = "Option::is_none" , rename = "ChannelEncryptionStrength")] pub channel_encryption_strength : Option < crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceChannelEncryptionStrength > , # [serde (skip_serializing_if = "Option::is_none" , rename = "DataSanitizationPolicy")] pub data_sanitization_policy : Option < crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceDataSanitizationPolicy > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Description")] pub description : Option < crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceDescription > , # [serde (skip_serializing_if = "Option::is_none" , rename = "HostAuthenticationType")] pub host_authentication_type : Option < crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceHostAuthenticationType > , # [serde (rename = "Id")] pub id : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "MediaEncryptionStrength")] pub media_encryption_strength : Option < crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceMediaEncryptionStrength > , # [serde (rename = "Name")] pub name : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "@odata.context")] pub odata_context : Option < String > , # [serde (skip_serializing_if = "Option::is_none" , rename = "@odata.etag")] pub odata_etag : Option < String > , # [serde (rename = "@odata.id")] pub odata_id : String , # [serde (rename = "@odata.type")] pub odata_type : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "Oem")] pub oem : Option < crate :: resource :: Oem > , # [serde (skip_serializing_if = "Option::is_none" , rename = "SecureChannelProtocol")] pub secure_channel_protocol : Option < crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceSecureChannelProtocol > , # [serde (skip_serializing_if = "Option::is_none" , rename = "UserAuthenticationType")] pub user_authentication_type : Option < crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceUserAuthenticationType > }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataSecurityLineOfServiceAntivirusScanPolicies {
        V000001 (crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceAntivirusScanPoliciesN1) , DataSecurityLoSCapabilitiesAntiVirusScanTrigger (crate :: swordfish :: data_security_los_capabilities :: AntiVirusScanTrigger) }
    impl Default for DataSecurityLineOfServiceAntivirusScanPolicies {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataSecurityLineOfServiceAntivirusScanPoliciesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataSecurityLineOfServiceChannelEncryptionStrength {
        V000001 (crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceChannelEncryptionStrengthN1) , DataSecurityLoSCapabilitiesKeySize (crate :: swordfish :: data_security_los_capabilities :: KeySize) }
    impl Default for DataSecurityLineOfServiceChannelEncryptionStrength {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataSecurityLineOfServiceChannelEncryptionStrengthN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataSecurityLineOfServiceDataSanitizationPolicy {
        V000001 (crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceDataSanitizationPolicyN1) , DataSecurityLoSCapabilitiesDataSanitizationPolicy (crate :: swordfish :: data_security_los_capabilities :: DataSanitizationPolicy) }
    impl Default for DataSecurityLineOfServiceDataSanitizationPolicy {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataSecurityLineOfServiceDataSanitizationPolicyN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataSecurityLineOfServiceDescription {
        V000001 (crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceDescriptionN1) , ResourceDescription (String) }
    impl Default for DataSecurityLineOfServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataSecurityLineOfServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataSecurityLineOfServiceHostAuthenticationType {
        V000001 (crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceHostAuthenticationTypeN1) , DataSecurityLoSCapabilitiesAuthenticationType (crate :: swordfish :: data_security_los_capabilities :: AuthenticationType) }
    impl Default for DataSecurityLineOfServiceHostAuthenticationType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataSecurityLineOfServiceHostAuthenticationTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataSecurityLineOfServiceMediaEncryptionStrength {
        V000001 (crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceMediaEncryptionStrengthN1) , DataSecurityLoSCapabilitiesKeySize (crate :: swordfish :: data_security_los_capabilities :: KeySize) }
    impl Default for DataSecurityLineOfServiceMediaEncryptionStrength {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataSecurityLineOfServiceMediaEncryptionStrengthN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataSecurityLineOfServiceSecureChannelProtocol {
        V000001 (crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceSecureChannelProtocolN1) , DataSecurityLoSCapabilitiesSecureChannelProtocol (crate :: swordfish :: data_security_los_capabilities :: SecureChannelProtocol) }
    impl Default for DataSecurityLineOfServiceSecureChannelProtocol {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataSecurityLineOfServiceSecureChannelProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DataSecurityLineOfServiceUserAuthenticationType {
        V000001 (crate :: swordfish :: data_security_line_of_service :: v1_1_1 :: DataSecurityLineOfServiceUserAuthenticationTypeN1) , DataSecurityLoSCapabilitiesAuthenticationType (crate :: swordfish :: data_security_los_capabilities :: AuthenticationType) }
    impl Default for DataSecurityLineOfServiceUserAuthenticationType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataSecurityLineOfServiceUserAuthenticationTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
