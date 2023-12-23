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
pub mod v1_10_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::manager_account::v1_10_0::OemActions>,
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
        pub account_types: Vec<crate::manager_account::AccountTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::manager_account::v1_10_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        pub links: Option<crate::manager_account::v1_10_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Locked")]
        pub locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MFABypass")]
        pub mfa_bypass: Option<crate::account_service::MFABypass>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PasswordChangeRequired"
        )]
        pub password_change_required: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordExpiration")]
        pub password_expiration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoleId")]
        pub role_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SNMP")]
        pub snmp: Option<crate::manager_account::v1_10_0::SNMPUserInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StrictAccountTypes")]
        pub strict_account_types: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
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
            Option<crate::manager_account::v1_10_0::SNMPAuthenticationProtocols>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKey")]
        pub encryption_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKeySet")]
        pub encryption_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionProtocol")]
        pub encryption_protocol: Option<crate::manager_account::v1_10_0::SNMPEncryptionProtocols>,
    }
}
