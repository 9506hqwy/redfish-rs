use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum AccountTypes {
    #[default]
    #[serde(rename = "HostConsole")]
    HostConsole,
    #[serde(rename = "IPMI")]
    IPMI,
    #[serde(rename = "KVMIP")]
    KVMIP,
    #[serde(rename = "ManagerConsole")]
    ManagerConsole,
    #[serde(rename = "OEM")]
    OEM,
    #[serde(rename = "Redfish")]
    Redfish,
    #[serde(rename = "SNMP")]
    SNMP,
    #[serde(rename = "VirtualMedia")]
    VirtualMedia,
    #[serde(rename = "WebUI")]
    WebUI,
}
pub type ManagerAccount = crate::manager_account::v1_13_0::ManagerAccount;
pub mod v1_12_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ManagerAccount.ChangePassword"
        )]
        pub manager_account_change_password:
            Option<crate::manager_account::v1_12_1::ChangePassword>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::manager_account::v1_12_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ChangePassword {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ChangePasswordRequestBody {
        #[serde(rename = "NewPassword")]
        pub new_password: String,
        #[serde(rename = "SessionAccountPassword")]
        pub session_account_password: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Role")]
        pub role: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ManagerAccount {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountExpiration")]
        pub account_expiration: Option<String>,
        #[serde(rename = "AccountTypes")]
        pub account_types: Vec<crate::manager_account::v1_12_1::ManagerAccountAccountTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::manager_account::v1_12_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::manager_account::v1_12_1::ManagerAccountDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAddress")]
        pub email_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostBootstrapAccount"
        )]
        pub host_bootstrap_account: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Keys")]
        pub keys: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::manager_account::v1_12_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Locked")]
        pub locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MFABypass")]
        pub mfa_bypass: Option<crate::manager_account::v1_12_1::ManagerAccountMFABypass>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OEMAccountTypes")]
        pub oem_account_types: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OneTimePasscodeDeliveryAddress"
        )]
        pub one_time_passcode_delivery_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PasswordChangeRequired"
        )]
        pub password_change_required: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordExpiration")]
        pub password_expiration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoleId")]
        pub role_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SNMP")]
        pub snmp: Option<crate::manager_account::v1_12_1::ManagerAccountSNMP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StrictAccountTypes")]
        pub strict_account_types: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerAccountAccountTypes {
        V000001(crate::manager_account::v1_12_1::ManagerAccountAccountTypesN1),
        ManagerAccountAccountTypes(crate::manager_account::AccountTypes),
    }
    impl Default for ManagerAccountAccountTypes {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerAccountAccountTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerAccountDescription {
        V000001(crate::manager_account::v1_12_1::ManagerAccountDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ManagerAccountDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerAccountDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerAccountMFABypass {
        V000001(crate::manager_account::v1_12_1::ManagerAccountMFABypassN1),
        AccountServiceMFABypass(crate::account_service::v1_17_0::MFABypass),
    }
    impl Default for ManagerAccountMFABypass {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerAccountMFABypassN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerAccountSNMP {
        V011201(crate::manager_account::v1_12_1::SNMPUserInfo),
        V000001(crate::manager_account::v1_12_1::ManagerAccountSNMPN1),
    }
    impl Default for ManagerAccountSNMP {
        fn default() -> Self {
            Self::V011201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerAccountSNMPN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPAuthenticationProtocols {
        #[default]
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
    pub struct SNMPUserInfo {
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
            Option<crate::manager_account::v1_12_1::SNMPUserInfoAuthenticationProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKey")]
        pub encryption_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKeySet")]
        pub encryption_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionProtocol")]
        pub encryption_protocol:
            Option<crate::manager_account::v1_12_1::SNMPUserInfoEncryptionProtocol>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SNMPUserInfoAuthenticationProtocol {
        V011201(crate::manager_account::v1_12_1::SNMPAuthenticationProtocols),
        V000001(crate::manager_account::v1_12_1::SNMPUserInfoAuthenticationProtocolN1),
    }
    impl Default for SNMPUserInfoAuthenticationProtocol {
        fn default() -> Self {
            Self::V011201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPUserInfoAuthenticationProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SNMPUserInfoEncryptionProtocol {
        V011201(crate::manager_account::v1_12_1::SNMPEncryptionProtocols),
        V000001(crate::manager_account::v1_12_1::SNMPUserInfoEncryptionProtocolN1),
    }
    impl Default for SNMPUserInfoEncryptionProtocol {
        fn default() -> Self {
            Self::V011201(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPUserInfoEncryptionProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
pub mod v1_13_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ManagerAccount.ChangePassword"
        )]
        pub manager_account_change_password:
            Option<crate::manager_account::v1_13_0::ChangePassword>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ManagerAccount.ClearSecretKey"
        )]
        pub manager_account_clear_secret_key:
            Option<crate::manager_account::v1_13_0::ClearSecretKey>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ManagerAccount.GenerateSecretKey"
        )]
        pub manager_account_generate_secret_key:
            Option<crate::manager_account::v1_13_0::GenerateSecretKey>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ManagerAccount.VerifyTimeBasedOneTimePassword"
        )]
        pub manager_account_verify_time_based_one_time_password:
            Option<crate::manager_account::v1_13_0::VerifyTimeBasedOneTimePassword>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::manager_account::v1_13_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ChangePassword {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ChangePasswordRequestBody {
        #[serde(rename = "NewPassword")]
        pub new_password: String,
        #[serde(rename = "SessionAccountPassword")]
        pub session_account_password: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearSecretKey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearSecretKeyRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenerateSecretKey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenerateSecretKeyRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenerateSecretKeyResponse {
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecretKey")]
        pub secret_key: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Role")]
        pub role: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ManagerAccount {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccountExpiration")]
        pub account_expiration: Option<String>,
        #[serde(rename = "AccountTypes")]
        pub account_types: Vec<crate::manager_account::v1_13_0::ManagerAccountAccountTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::manager_account::v1_13_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::manager_account::v1_13_0::ManagerAccountDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAddress")]
        pub email_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HostBootstrapAccount"
        )]
        pub host_bootstrap_account: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Keys")]
        pub keys: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::manager_account::v1_13_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Locked")]
        pub locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MFABypass")]
        pub mfa_bypass: Option<crate::manager_account::v1_13_0::ManagerAccountMFABypass>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OEMAccountTypes")]
        pub oem_account_types: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OneTimePasscodeDeliveryAddress"
        )]
        pub one_time_passcode_delivery_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PasswordChangeRequired"
        )]
        pub password_change_required: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordExpiration")]
        pub password_expiration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoleId")]
        pub role_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecretKeySet")]
        pub secret_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SNMP")]
        pub snmp: Option<crate::manager_account::v1_13_0::ManagerAccountSNMP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StrictAccountTypes")]
        pub strict_account_types: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerAccountAccountTypes {
        V000001(crate::manager_account::v1_13_0::ManagerAccountAccountTypesN1),
        ManagerAccountAccountTypes(crate::manager_account::AccountTypes),
    }
    impl Default for ManagerAccountAccountTypes {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerAccountAccountTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerAccountDescription {
        V000001(crate::manager_account::v1_13_0::ManagerAccountDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ManagerAccountDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerAccountDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerAccountMFABypass {
        V000001(crate::manager_account::v1_13_0::ManagerAccountMFABypassN1),
        AccountServiceMFABypass(crate::account_service::v1_17_0::MFABypass),
    }
    impl Default for ManagerAccountMFABypass {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerAccountMFABypassN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ManagerAccountSNMP {
        V011300(crate::manager_account::v1_13_0::SNMPUserInfo),
        V000001(crate::manager_account::v1_13_0::ManagerAccountSNMPN1),
    }
    impl Default for ManagerAccountSNMP {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ManagerAccountSNMPN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPAuthenticationProtocols {
        #[default]
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
    pub struct SNMPUserInfo {
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
            Option<crate::manager_account::v1_13_0::SNMPUserInfoAuthenticationProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKey")]
        pub encryption_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKeySet")]
        pub encryption_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionProtocol")]
        pub encryption_protocol:
            Option<crate::manager_account::v1_13_0::SNMPUserInfoEncryptionProtocol>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SNMPUserInfoAuthenticationProtocol {
        V011300(crate::manager_account::v1_13_0::SNMPAuthenticationProtocols),
        V000001(crate::manager_account::v1_13_0::SNMPUserInfoAuthenticationProtocolN1),
    }
    impl Default for SNMPUserInfoAuthenticationProtocol {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPUserInfoAuthenticationProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SNMPUserInfoEncryptionProtocol {
        V011300(crate::manager_account::v1_13_0::SNMPEncryptionProtocols),
        V000001(crate::manager_account::v1_13_0::SNMPUserInfoEncryptionProtocolN1),
    }
    impl Default for SNMPUserInfoEncryptionProtocol {
        fn default() -> Self {
            Self::V011300(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPUserInfoEncryptionProtocolN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VerifyTimeBasedOneTimePassword {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VerifyTimeBasedOneTimePasswordRequestBody {
        #[serde(rename = "TimeBasedOneTimePassword")]
        pub time_based_one_time_password: String,
    }
}
