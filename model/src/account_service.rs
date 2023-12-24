use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AccountService {
    AccountServiceV1N0N13AccountService(crate::account_service::v1_0_13::AccountService),
    AccountServiceV1N10N4AccountService(crate::account_service::v1_10_4::AccountService),
    AccountServiceV1N11N4AccountService(crate::account_service::v1_11_4::AccountService),
    AccountServiceV1N12N1AccountService(crate::account_service::v1_12_1::AccountService),
    AccountServiceV1N13N0AccountService(crate::account_service::v1_13_0::AccountService),
    AccountServiceV1N1N10AccountService(crate::account_service::v1_1_10::AccountService),
    AccountServiceV1N2N10AccountService(crate::account_service::v1_2_10::AccountService),
    AccountServiceV1N3N11AccountService(crate::account_service::v1_3_11::AccountService),
    AccountServiceV1N4N9AccountService(crate::account_service::v1_4_9::AccountService),
    AccountServiceV1N5N8AccountService(crate::account_service::v1_5_8::AccountService),
    AccountServiceV1N6N7AccountService(crate::account_service::v1_6_7::AccountService),
    AccountServiceV1N7N7AccountService(crate::account_service::v1_7_7::AccountService),
    AccountServiceV1N8N4AccountService(crate::account_service::v1_8_4::AccountService),
    AccountServiceV1N9N3AccountService(crate::account_service::v1_9_3::AccountService),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MFABypass {
    AccountServiceV1N12N1MFABypass(crate::account_service::v1_12_1::MFABypass),
    AccountServiceV1N13N0MFABypass(crate::account_service::v1_13_0::MFABypass),
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum MFABypassType {
    #[default]
    #[serde(rename = "All")]
    All,
    #[serde(rename = "ClientCertificate")]
    ClientCertificate,
    #[serde(rename = "GoogleAuthenticator")]
    GoogleAuthenticator,
    #[serde(rename = "MicrosoftAuthenticator")]
    MicrosoftAuthenticator,
    #[serde(rename = "OEM")]
    OEM,
    #[serde(rename = "SecurID")]
    SecurID,
}
pub mod v1_0_13 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccountService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetAfter"
        )]
        pub account_lockout_counter_reset_after: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutDuration"
        )]
        pub account_lockout_duration: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutThreshold"
        )]
        pub account_lockout_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accounts")]
        pub accounts: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthFailureLoggingThreshold"
        )]
        pub auth_failure_logging_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_10_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountProviderTypes {
        #[default]
        #[serde(rename = "ActiveDirectoryService")]
        ActiveDirectoryService,
        #[serde(rename = "LDAPService")]
        LDAPService,
        #[serde(rename = "OAuth2")]
        OAuth2,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "RedfishService")]
        RedfishService,
        #[serde(rename = "TACACSplus")]
        TACACSplus,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccountService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetAfter"
        )]
        pub account_lockout_counter_reset_after: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetEnabled"
        )]
        pub account_lockout_counter_reset_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutDuration"
        )]
        pub account_lockout_duration: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutThreshold"
        )]
        pub account_lockout_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accounts")]
        pub accounts: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::account_service::v1_10_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveDirectory")]
        pub active_directory: Option<crate::account_service::v1_10_4::ExternalAccountProvider>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalExternalAccountProviders"
        )]
        pub additional_external_account_providers: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthFailureLoggingThreshold"
        )]
        pub auth_failure_logging_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAP")]
        pub ldap: Option<crate::account_service::v1_10_4::ExternalAccountProvider>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAccountAuth")]
        pub local_account_auth: Option<crate::account_service::v1_10_4::LocalAccountAuth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OAuth2")]
        pub oauth2: Option<crate::account_service::v1_10_4::ExternalAccountProvider>,
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
            rename = "PasswordExpirationDays"
        )]
        pub password_expiration_days: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestrictedOemPrivileges"
        )]
        pub restricted_oem_privileges: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestrictedPrivileges"
        )]
        pub restricted_privileges: Option<Vec<crate::privileges::PrivilegeType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedAccountTypes"
        )]
        pub supported_account_types: Option<Vec<crate::manager_account::AccountTypes>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedOEMAccountTypes"
        )]
        pub supported_oem_account_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplus")]
        pub tacac_splus: Option<crate::account_service::v1_10_4::ExternalAccountProvider>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::account_service::v1_10_4::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type: Option<crate::account_service::v1_10_4::AuthenticationTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKey")]
        pub encryption_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKeySet")]
        pub encryption_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KerberosKeytab")]
        pub kerberos_keytab: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Token")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationTypes {
        #[default]
        #[serde(rename = "KerberosKeytab")]
        KerberosKeytab,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Token")]
        Token,
        #[serde(rename = "UsernameAndPassword")]
        UsernameAndPassword,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExternalAccountProvider {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountProviderType"
        )]
        pub account_provider_type: Option<crate::account_service::v1_10_4::AccountProviderTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::account_service::v1_10_4::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::account_service::v1_10_4::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OAuth2Service")]
        pub oauth2_service: Option<crate::account_service::v1_10_4::OAuth2Service>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordSet")]
        pub password_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
        pub priority: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping: Option<Vec<crate::account_service::v1_10_4::RoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplusService")]
        pub tacac_splus_service: Option<crate::account_service::v1_10_4::TACACSplusService>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPSearchSettings {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BaseDistinguishedNames"
        )]
        pub base_distinguished_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupNameAttribute")]
        pub group_name_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupsAttribute")]
        pub groups_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UsernameAttribute")]
        pub username_attribute: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SearchSettings")]
        pub search_settings: Option<crate::account_service::v1_10_4::LDAPSearchSettings>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocalAccountAuth {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Fallback")]
        Fallback,
        #[serde(rename = "LocalFirst")]
        LocalFirst,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OAuth2Mode {
        #[default]
        #[serde(rename = "Discovery")]
        Discovery,
        #[serde(rename = "Offline")]
        Offline,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OAuth2Service {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Audience")]
        pub audience: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Issuer")]
        pub issuer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Mode")]
        pub mode: Option<crate::account_service::v1_10_4::OAuth2Mode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OAuthServiceSigningKeys"
        )]
        pub oauth_service_signing_keys: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RoleMapping {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalRole")]
        pub local_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteGroup")]
        pub remote_group: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteUser")]
        pub remote_user: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TACACSplusPasswordExchangeProtocol {
        #[default]
        #[serde(rename = "ASCII")]
        ASCII,
        #[serde(rename = "CHAP")]
        CHAP,
        #[serde(rename = "MSCHAPv1")]
        MSCHAPv1,
        #[serde(rename = "MSCHAPv2")]
        MSCHAPv2,
        #[serde(rename = "PAP")]
        PAP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TACACSplusService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PasswordExchangeProtocols"
        )]
        pub password_exchange_protocols:
            Option<Vec<crate::account_service::v1_10_4::TACACSplusPasswordExchangeProtocol>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrivilegeLevelArgument"
        )]
        pub privilege_level_argument: Option<String>,
    }
}
pub mod v1_11_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountProviderTypes {
        #[default]
        #[serde(rename = "ActiveDirectoryService")]
        ActiveDirectoryService,
        #[serde(rename = "LDAPService")]
        LDAPService,
        #[serde(rename = "OAuth2")]
        OAuth2,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "RedfishService")]
        RedfishService,
        #[serde(rename = "TACACSplus")]
        TACACSplus,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccountService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetAfter"
        )]
        pub account_lockout_counter_reset_after: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetEnabled"
        )]
        pub account_lockout_counter_reset_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutDuration"
        )]
        pub account_lockout_duration: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutThreshold"
        )]
        pub account_lockout_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accounts")]
        pub accounts: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::account_service::v1_11_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveDirectory")]
        pub active_directory: Option<crate::account_service::v1_11_4::ExternalAccountProvider>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalExternalAccountProviders"
        )]
        pub additional_external_account_providers: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthFailureLoggingThreshold"
        )]
        pub auth_failure_logging_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAP")]
        pub ldap: Option<crate::account_service::v1_11_4::ExternalAccountProvider>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAccountAuth")]
        pub local_account_auth: Option<crate::account_service::v1_11_4::LocalAccountAuth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OAuth2")]
        pub oauth2: Option<crate::account_service::v1_11_4::ExternalAccountProvider>,
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
            rename = "PasswordExpirationDays"
        )]
        pub password_expiration_days: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestrictedOemPrivileges"
        )]
        pub restricted_oem_privileges: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestrictedPrivileges"
        )]
        pub restricted_privileges: Option<Vec<crate::privileges::PrivilegeType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedAccountTypes"
        )]
        pub supported_account_types: Option<Vec<crate::manager_account::AccountTypes>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedOEMAccountTypes"
        )]
        pub supported_oem_account_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplus")]
        pub tacac_splus: Option<crate::account_service::v1_11_4::ExternalAccountProvider>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::account_service::v1_11_4::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type: Option<crate::account_service::v1_11_4::AuthenticationTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKey")]
        pub encryption_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKeySet")]
        pub encryption_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KerberosKeytab")]
        pub kerberos_keytab: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Token")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationTypes {
        #[default]
        #[serde(rename = "KerberosKeytab")]
        KerberosKeytab,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Token")]
        Token,
        #[serde(rename = "UsernameAndPassword")]
        UsernameAndPassword,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExternalAccountProvider {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountProviderType"
        )]
        pub account_provider_type: Option<crate::account_service::v1_11_4::AccountProviderTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::account_service::v1_11_4::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::account_service::v1_11_4::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OAuth2Service")]
        pub oauth2_service: Option<crate::account_service::v1_11_4::OAuth2Service>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordSet")]
        pub password_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
        pub priority: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping: Option<Vec<crate::account_service::v1_11_4::RoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplusService")]
        pub tacac_splus_service: Option<crate::account_service::v1_11_4::TACACSplusService>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPSearchSettings {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BaseDistinguishedNames"
        )]
        pub base_distinguished_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupNameAttribute")]
        pub group_name_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupsAttribute")]
        pub groups_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSHKeyAttribute")]
        pub ssh_key_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UsernameAttribute")]
        pub username_attribute: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SearchSettings")]
        pub search_settings: Option<crate::account_service::v1_11_4::LDAPSearchSettings>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocalAccountAuth {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Fallback")]
        Fallback,
        #[serde(rename = "LocalFirst")]
        LocalFirst,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OAuth2Mode {
        #[default]
        #[serde(rename = "Discovery")]
        Discovery,
        #[serde(rename = "Offline")]
        Offline,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OAuth2Service {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Audience")]
        pub audience: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Issuer")]
        pub issuer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Mode")]
        pub mode: Option<crate::account_service::v1_11_4::OAuth2Mode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OAuthServiceSigningKeys"
        )]
        pub oauth_service_signing_keys: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RoleMapping {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalRole")]
        pub local_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteGroup")]
        pub remote_group: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteUser")]
        pub remote_user: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TACACSplusPasswordExchangeProtocol {
        #[default]
        #[serde(rename = "ASCII")]
        ASCII,
        #[serde(rename = "CHAP")]
        CHAP,
        #[serde(rename = "MSCHAPv1")]
        MSCHAPv1,
        #[serde(rename = "MSCHAPv2")]
        MSCHAPv2,
        #[serde(rename = "PAP")]
        PAP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TACACSplusService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PasswordExchangeProtocols"
        )]
        pub password_exchange_protocols:
            Option<Vec<crate::account_service::v1_11_4::TACACSplusPasswordExchangeProtocol>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrivilegeLevelArgument"
        )]
        pub privilege_level_argument: Option<String>,
    }
}
pub mod v1_12_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountProviderTypes {
        #[default]
        #[serde(rename = "ActiveDirectoryService")]
        ActiveDirectoryService,
        #[serde(rename = "LDAPService")]
        LDAPService,
        #[serde(rename = "OAuth2")]
        OAuth2,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "RedfishService")]
        RedfishService,
        #[serde(rename = "TACACSplus")]
        TACACSplus,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccountService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetAfter"
        )]
        pub account_lockout_counter_reset_after: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetEnabled"
        )]
        pub account_lockout_counter_reset_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutDuration"
        )]
        pub account_lockout_duration: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutThreshold"
        )]
        pub account_lockout_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accounts")]
        pub accounts: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::account_service::v1_12_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveDirectory")]
        pub active_directory: Option<crate::account_service::v1_12_1::ExternalAccountProvider>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalExternalAccountProviders"
        )]
        pub additional_external_account_providers: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthFailureLoggingThreshold"
        )]
        pub auth_failure_logging_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAP")]
        pub ldap: Option<crate::account_service::v1_12_1::ExternalAccountProvider>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAccountAuth")]
        pub local_account_auth: Option<crate::account_service::v1_12_1::LocalAccountAuth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultiFactorAuth")]
        pub multi_factor_auth: Option<crate::account_service::v1_12_1::MultiFactorAuth>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OAuth2")]
        pub oauth2: Option<crate::account_service::v1_12_1::ExternalAccountProvider>,
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
            rename = "PasswordExpirationDays"
        )]
        pub password_expiration_days: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestrictedOemPrivileges"
        )]
        pub restricted_oem_privileges: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestrictedPrivileges"
        )]
        pub restricted_privileges: Option<Vec<crate::privileges::PrivilegeType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedAccountTypes"
        )]
        pub supported_account_types: Option<Vec<crate::manager_account::AccountTypes>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedOEMAccountTypes"
        )]
        pub supported_oem_account_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplus")]
        pub tacac_splus: Option<crate::account_service::v1_12_1::ExternalAccountProvider>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::account_service::v1_12_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type: Option<crate::account_service::v1_12_1::AuthenticationTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKey")]
        pub encryption_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKeySet")]
        pub encryption_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KerberosKeytab")]
        pub kerberos_keytab: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Token")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationTypes {
        #[default]
        #[serde(rename = "KerberosKeytab")]
        KerberosKeytab,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Token")]
        Token,
        #[serde(rename = "UsernameAndPassword")]
        UsernameAndPassword,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CertificateMappingAttribute {
        #[default]
        #[serde(rename = "CommonName")]
        CommonName,
        #[serde(rename = "UserPrincipalName")]
        UserPrincipalName,
        #[serde(rename = "Whole")]
        Whole,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClientCertificate {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CertificateMappingAttribute"
        )]
        pub certificate_mapping_attribute:
            Option<crate::account_service::v1_12_1::CertificateMappingAttribute>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RespondToUnauthenticatedClients"
        )]
        pub respond_to_unauthenticated_clients: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExternalAccountProvider {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountProviderType"
        )]
        pub account_provider_type: Option<crate::account_service::v1_12_1::AccountProviderTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::account_service::v1_12_1::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::account_service::v1_12_1::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OAuth2Service")]
        pub oauth2_service: Option<crate::account_service::v1_12_1::OAuth2Service>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordSet")]
        pub password_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
        pub priority: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping: Option<Vec<crate::account_service::v1_12_1::RoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplusService")]
        pub tacac_splus_service: Option<crate::account_service::v1_12_1::TACACSplusService>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GoogleAuthenticator {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecretKey")]
        pub secret_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecretKeySet")]
        pub secret_key_set: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPSearchSettings {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BaseDistinguishedNames"
        )]
        pub base_distinguished_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupNameAttribute")]
        pub group_name_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupsAttribute")]
        pub groups_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSHKeyAttribute")]
        pub ssh_key_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UsernameAttribute")]
        pub username_attribute: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SearchSettings")]
        pub search_settings: Option<crate::account_service::v1_12_1::LDAPSearchSettings>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocalAccountAuth {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Fallback")]
        Fallback,
        #[serde(rename = "LocalFirst")]
        LocalFirst,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MFABypass {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BypassTypes")]
        pub bypass_types: Option<Vec<crate::account_service::MFABypassType>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MicrosoftAuthenticator {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecretKey")]
        pub secret_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecretKeySet")]
        pub secret_key_set: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MultiFactorAuth {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientCertificate")]
        pub client_certificate: Option<crate::account_service::v1_12_1::ClientCertificate>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GoogleAuthenticator"
        )]
        pub google_authenticator: Option<crate::account_service::v1_12_1::GoogleAuthenticator>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MicrosoftAuthenticator"
        )]
        pub microsoft_authenticator:
            Option<crate::account_service::v1_12_1::MicrosoftAuthenticator>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurID")]
        pub secur_id: Option<crate::account_service::v1_12_1::SecurID>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OAuth2Mode {
        #[default]
        #[serde(rename = "Discovery")]
        Discovery,
        #[serde(rename = "Offline")]
        Offline,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OAuth2Service {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Audience")]
        pub audience: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Issuer")]
        pub issuer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Mode")]
        pub mode: Option<crate::account_service::v1_12_1::OAuth2Mode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OAuthServiceSigningKeys"
        )]
        pub oauth_service_signing_keys: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RoleMapping {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalRole")]
        pub local_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MFABypass")]
        pub mfa_bypass: Option<crate::account_service::MFABypass>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteGroup")]
        pub remote_group: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteUser")]
        pub remote_user: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurID {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientId")]
        pub client_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientSecret")]
        pub client_secret: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientSecretSet")]
        pub client_secret_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServerURI")]
        pub server_uri: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TACACSplusPasswordExchangeProtocol {
        #[default]
        #[serde(rename = "ASCII")]
        ASCII,
        #[serde(rename = "CHAP")]
        CHAP,
        #[serde(rename = "MSCHAPv1")]
        MSCHAPv1,
        #[serde(rename = "MSCHAPv2")]
        MSCHAPv2,
        #[serde(rename = "PAP")]
        PAP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TACACSplusService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PasswordExchangeProtocols"
        )]
        pub password_exchange_protocols:
            Option<Vec<crate::account_service::v1_12_1::TACACSplusPasswordExchangeProtocol>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrivilegeLevelArgument"
        )]
        pub privilege_level_argument: Option<String>,
    }
}
pub mod v1_13_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountProviderTypes {
        #[default]
        #[serde(rename = "ActiveDirectoryService")]
        ActiveDirectoryService,
        #[serde(rename = "LDAPService")]
        LDAPService,
        #[serde(rename = "OAuth2")]
        OAuth2,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "RedfishService")]
        RedfishService,
        #[serde(rename = "TACACSplus")]
        TACACSplus,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccountService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetAfter"
        )]
        pub account_lockout_counter_reset_after: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetEnabled"
        )]
        pub account_lockout_counter_reset_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutDuration"
        )]
        pub account_lockout_duration: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutThreshold"
        )]
        pub account_lockout_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accounts")]
        pub accounts: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::account_service::v1_13_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveDirectory")]
        pub active_directory: Option<crate::account_service::v1_13_0::ExternalAccountProvider>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalExternalAccountProviders"
        )]
        pub additional_external_account_providers: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthFailureLoggingThreshold"
        )]
        pub auth_failure_logging_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAP")]
        pub ldap: Option<crate::account_service::v1_13_0::ExternalAccountProvider>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAccountAuth")]
        pub local_account_auth: Option<crate::account_service::v1_13_0::LocalAccountAuth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultiFactorAuth")]
        pub multi_factor_auth: Option<crate::account_service::v1_13_0::MultiFactorAuth>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OAuth2")]
        pub oauth2: Option<crate::account_service::v1_13_0::ExternalAccountProvider>,
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
            rename = "PasswordExpirationDays"
        )]
        pub password_expiration_days: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestrictedOemPrivileges"
        )]
        pub restricted_oem_privileges: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestrictedPrivileges"
        )]
        pub restricted_privileges: Option<Vec<crate::privileges::PrivilegeType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedAccountTypes"
        )]
        pub supported_account_types: Option<Vec<crate::manager_account::AccountTypes>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedOEMAccountTypes"
        )]
        pub supported_oem_account_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplus")]
        pub tacac_splus: Option<crate::account_service::v1_13_0::ExternalAccountProvider>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::account_service::v1_13_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type: Option<crate::account_service::v1_13_0::AuthenticationTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKey")]
        pub encryption_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKeySet")]
        pub encryption_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KerberosKeytab")]
        pub kerberos_keytab: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Token")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationTypes {
        #[default]
        #[serde(rename = "KerberosKeytab")]
        KerberosKeytab,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Token")]
        Token,
        #[serde(rename = "UsernameAndPassword")]
        UsernameAndPassword,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CertificateMappingAttribute {
        #[default]
        #[serde(rename = "CommonName")]
        CommonName,
        #[serde(rename = "UserPrincipalName")]
        UserPrincipalName,
        #[serde(rename = "Whole")]
        Whole,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClientCertificate {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CertificateMappingAttribute"
        )]
        pub certificate_mapping_attribute:
            Option<crate::account_service::v1_13_0::CertificateMappingAttribute>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RespondToUnauthenticatedClients"
        )]
        pub respond_to_unauthenticated_clients: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExternalAccountProvider {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountProviderType"
        )]
        pub account_provider_type: Option<crate::account_service::v1_13_0::AccountProviderTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::account_service::v1_13_0::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::account_service::v1_13_0::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OAuth2Service")]
        pub oauth2_service: Option<crate::account_service::v1_13_0::OAuth2Service>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordSet")]
        pub password_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
        pub priority: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping: Option<Vec<crate::account_service::v1_13_0::RoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Retries")]
        pub retries: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplusService")]
        pub tacac_splus_service: Option<crate::account_service::v1_13_0::TACACSplusService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimeoutSeconds")]
        pub timeout_seconds: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GoogleAuthenticator {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecretKey")]
        pub secret_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecretKeySet")]
        pub secret_key_set: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPSearchSettings {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BaseDistinguishedNames"
        )]
        pub base_distinguished_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupNameAttribute")]
        pub group_name_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupsAttribute")]
        pub groups_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSHKeyAttribute")]
        pub ssh_key_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UsernameAttribute")]
        pub username_attribute: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SearchSettings")]
        pub search_settings: Option<crate::account_service::v1_13_0::LDAPSearchSettings>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocalAccountAuth {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Fallback")]
        Fallback,
        #[serde(rename = "LocalFirst")]
        LocalFirst,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MFABypass {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BypassTypes")]
        pub bypass_types: Option<Vec<crate::account_service::MFABypassType>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MicrosoftAuthenticator {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecretKey")]
        pub secret_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecretKeySet")]
        pub secret_key_set: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MultiFactorAuth {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientCertificate")]
        pub client_certificate: Option<crate::account_service::v1_13_0::ClientCertificate>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GoogleAuthenticator"
        )]
        pub google_authenticator: Option<crate::account_service::v1_13_0::GoogleAuthenticator>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MicrosoftAuthenticator"
        )]
        pub microsoft_authenticator:
            Option<crate::account_service::v1_13_0::MicrosoftAuthenticator>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurID")]
        pub secur_id: Option<crate::account_service::v1_13_0::SecurID>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OAuth2Mode {
        #[default]
        #[serde(rename = "Discovery")]
        Discovery,
        #[serde(rename = "Offline")]
        Offline,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OAuth2Service {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Audience")]
        pub audience: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Issuer")]
        pub issuer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Mode")]
        pub mode: Option<crate::account_service::v1_13_0::OAuth2Mode>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OAuthServiceSigningKeys"
        )]
        pub oauth_service_signing_keys: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RoleMapping {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalRole")]
        pub local_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MFABypass")]
        pub mfa_bypass: Option<crate::account_service::MFABypass>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteGroup")]
        pub remote_group: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteUser")]
        pub remote_user: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurID {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientId")]
        pub client_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientSecret")]
        pub client_secret: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientSecretSet")]
        pub client_secret_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServerURI")]
        pub server_uri: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TACACSplusPasswordExchangeProtocol {
        #[default]
        #[serde(rename = "ASCII")]
        ASCII,
        #[serde(rename = "CHAP")]
        CHAP,
        #[serde(rename = "MSCHAPv1")]
        MSCHAPv1,
        #[serde(rename = "MSCHAPv2")]
        MSCHAPv2,
        #[serde(rename = "PAP")]
        PAP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TACACSplusService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthorizationService"
        )]
        pub authorization_service: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PasswordExchangeProtocols"
        )]
        pub password_exchange_protocols:
            Option<Vec<crate::account_service::v1_13_0::TACACSplusPasswordExchangeProtocol>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrivilegeLevelArgument"
        )]
        pub privilege_level_argument: Option<String>,
    }
}
pub mod v1_1_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccountService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetAfter"
        )]
        pub account_lockout_counter_reset_after: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutDuration"
        )]
        pub account_lockout_duration: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutThreshold"
        )]
        pub account_lockout_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accounts")]
        pub accounts: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthFailureLoggingThreshold"
        )]
        pub auth_failure_logging_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_2_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccountService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetAfter"
        )]
        pub account_lockout_counter_reset_after: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutDuration"
        )]
        pub account_lockout_duration: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutThreshold"
        )]
        pub account_lockout_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accounts")]
        pub accounts: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::account_service::v1_2_10::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthFailureLoggingThreshold"
        )]
        pub auth_failure_logging_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::account_service::v1_2_10::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_3_11 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountProviderTypes {
        #[default]
        #[serde(rename = "ActiveDirectoryService")]
        ActiveDirectoryService,
        #[serde(rename = "LDAPService")]
        LDAPService,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "RedfishService")]
        RedfishService,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccountService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetAfter"
        )]
        pub account_lockout_counter_reset_after: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutDuration"
        )]
        pub account_lockout_duration: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutThreshold"
        )]
        pub account_lockout_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accounts")]
        pub accounts: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::account_service::v1_3_11::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveDirectory")]
        pub active_directory: Option<crate::account_service::v1_3_11::ExternalAccountProvider>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalExternalAccountProviders"
        )]
        pub additional_external_account_providers: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthFailureLoggingThreshold"
        )]
        pub auth_failure_logging_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAP")]
        pub ldap: Option<crate::account_service::v1_3_11::ExternalAccountProvider>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAccountAuth")]
        pub local_account_auth: Option<crate::account_service::v1_3_11::LocalAccountAuth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::account_service::v1_3_11::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type: Option<crate::account_service::v1_3_11::AuthenticationTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KerberosKeytab")]
        pub kerberos_keytab: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Token")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationTypes {
        #[default]
        #[serde(rename = "KerberosKeytab")]
        KerberosKeytab,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Token")]
        Token,
        #[serde(rename = "UsernameAndPassword")]
        UsernameAndPassword,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExternalAccountProvider {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountProviderType"
        )]
        pub account_provider_type: Option<crate::account_service::v1_3_11::AccountProviderTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::account_service::v1_3_11::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::account_service::v1_3_11::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping: Option<Vec<crate::account_service::v1_3_11::RoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPSearchSettings {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BaseDistinguishedNames"
        )]
        pub base_distinguished_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupNameAttribute")]
        pub group_name_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupsAttribute")]
        pub groups_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UsernameAttribute")]
        pub username_attribute: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SearchSettings")]
        pub search_settings: Option<crate::account_service::v1_3_11::LDAPSearchSettings>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocalAccountAuth {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Fallback")]
        Fallback,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RoleMapping {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalRole")]
        pub local_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteGroup")]
        pub remote_group: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteUser")]
        pub remote_user: Option<String>,
    }
}
pub mod v1_4_9 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountProviderTypes {
        #[default]
        #[serde(rename = "ActiveDirectoryService")]
        ActiveDirectoryService,
        #[serde(rename = "LDAPService")]
        LDAPService,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "RedfishService")]
        RedfishService,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccountService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetAfter"
        )]
        pub account_lockout_counter_reset_after: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutDuration"
        )]
        pub account_lockout_duration: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutThreshold"
        )]
        pub account_lockout_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accounts")]
        pub accounts: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::account_service::v1_4_9::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveDirectory")]
        pub active_directory: Option<crate::account_service::v1_4_9::ExternalAccountProvider>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalExternalAccountProviders"
        )]
        pub additional_external_account_providers: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthFailureLoggingThreshold"
        )]
        pub auth_failure_logging_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAP")]
        pub ldap: Option<crate::account_service::v1_4_9::ExternalAccountProvider>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAccountAuth")]
        pub local_account_auth: Option<crate::account_service::v1_4_9::LocalAccountAuth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::account_service::v1_4_9::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type: Option<crate::account_service::v1_4_9::AuthenticationTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KerberosKeytab")]
        pub kerberos_keytab: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Token")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationTypes {
        #[default]
        #[serde(rename = "KerberosKeytab")]
        KerberosKeytab,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Token")]
        Token,
        #[serde(rename = "UsernameAndPassword")]
        UsernameAndPassword,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExternalAccountProvider {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountProviderType"
        )]
        pub account_provider_type: Option<crate::account_service::v1_4_9::AccountProviderTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::account_service::v1_4_9::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::account_service::v1_4_9::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping: Option<Vec<crate::account_service::v1_4_9::RoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPSearchSettings {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BaseDistinguishedNames"
        )]
        pub base_distinguished_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupNameAttribute")]
        pub group_name_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupsAttribute")]
        pub groups_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UsernameAttribute")]
        pub username_attribute: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SearchSettings")]
        pub search_settings: Option<crate::account_service::v1_4_9::LDAPSearchSettings>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocalAccountAuth {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Fallback")]
        Fallback,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RoleMapping {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalRole")]
        pub local_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteGroup")]
        pub remote_group: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteUser")]
        pub remote_user: Option<String>,
    }
}
pub mod v1_5_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountProviderTypes {
        #[default]
        #[serde(rename = "ActiveDirectoryService")]
        ActiveDirectoryService,
        #[serde(rename = "LDAPService")]
        LDAPService,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "RedfishService")]
        RedfishService,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccountService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetAfter"
        )]
        pub account_lockout_counter_reset_after: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetEnabled"
        )]
        pub account_lockout_counter_reset_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutDuration"
        )]
        pub account_lockout_duration: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutThreshold"
        )]
        pub account_lockout_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accounts")]
        pub accounts: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::account_service::v1_5_8::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveDirectory")]
        pub active_directory: Option<crate::account_service::v1_5_8::ExternalAccountProvider>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalExternalAccountProviders"
        )]
        pub additional_external_account_providers: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthFailureLoggingThreshold"
        )]
        pub auth_failure_logging_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAP")]
        pub ldap: Option<crate::account_service::v1_5_8::ExternalAccountProvider>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAccountAuth")]
        pub local_account_auth: Option<crate::account_service::v1_5_8::LocalAccountAuth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::account_service::v1_5_8::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type: Option<crate::account_service::v1_5_8::AuthenticationTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KerberosKeytab")]
        pub kerberos_keytab: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Token")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationTypes {
        #[default]
        #[serde(rename = "KerberosKeytab")]
        KerberosKeytab,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Token")]
        Token,
        #[serde(rename = "UsernameAndPassword")]
        UsernameAndPassword,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExternalAccountProvider {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountProviderType"
        )]
        pub account_provider_type: Option<crate::account_service::v1_5_8::AccountProviderTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::account_service::v1_5_8::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::account_service::v1_5_8::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping: Option<Vec<crate::account_service::v1_5_8::RoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPSearchSettings {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BaseDistinguishedNames"
        )]
        pub base_distinguished_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupNameAttribute")]
        pub group_name_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupsAttribute")]
        pub groups_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UsernameAttribute")]
        pub username_attribute: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SearchSettings")]
        pub search_settings: Option<crate::account_service::v1_5_8::LDAPSearchSettings>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocalAccountAuth {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Fallback")]
        Fallback,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RoleMapping {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalRole")]
        pub local_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteGroup")]
        pub remote_group: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteUser")]
        pub remote_user: Option<String>,
    }
}
pub mod v1_6_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountProviderTypes {
        #[default]
        #[serde(rename = "ActiveDirectoryService")]
        ActiveDirectoryService,
        #[serde(rename = "LDAPService")]
        LDAPService,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "RedfishService")]
        RedfishService,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccountService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetAfter"
        )]
        pub account_lockout_counter_reset_after: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetEnabled"
        )]
        pub account_lockout_counter_reset_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutDuration"
        )]
        pub account_lockout_duration: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutThreshold"
        )]
        pub account_lockout_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accounts")]
        pub accounts: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::account_service::v1_6_7::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveDirectory")]
        pub active_directory: Option<crate::account_service::v1_6_7::ExternalAccountProvider>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalExternalAccountProviders"
        )]
        pub additional_external_account_providers: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthFailureLoggingThreshold"
        )]
        pub auth_failure_logging_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAP")]
        pub ldap: Option<crate::account_service::v1_6_7::ExternalAccountProvider>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAccountAuth")]
        pub local_account_auth: Option<crate::account_service::v1_6_7::LocalAccountAuth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::account_service::v1_6_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type: Option<crate::account_service::v1_6_7::AuthenticationTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KerberosKeytab")]
        pub kerberos_keytab: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Token")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationTypes {
        #[default]
        #[serde(rename = "KerberosKeytab")]
        KerberosKeytab,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Token")]
        Token,
        #[serde(rename = "UsernameAndPassword")]
        UsernameAndPassword,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExternalAccountProvider {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountProviderType"
        )]
        pub account_provider_type: Option<crate::account_service::v1_6_7::AccountProviderTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::account_service::v1_6_7::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::account_service::v1_6_7::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping: Option<Vec<crate::account_service::v1_6_7::RoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPSearchSettings {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BaseDistinguishedNames"
        )]
        pub base_distinguished_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupNameAttribute")]
        pub group_name_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupsAttribute")]
        pub groups_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UsernameAttribute")]
        pub username_attribute: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SearchSettings")]
        pub search_settings: Option<crate::account_service::v1_6_7::LDAPSearchSettings>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocalAccountAuth {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Fallback")]
        Fallback,
        #[serde(rename = "LocalFirst")]
        LocalFirst,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RoleMapping {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalRole")]
        pub local_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteGroup")]
        pub remote_group: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteUser")]
        pub remote_user: Option<String>,
    }
}
pub mod v1_7_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountProviderTypes {
        #[default]
        #[serde(rename = "ActiveDirectoryService")]
        ActiveDirectoryService,
        #[serde(rename = "LDAPService")]
        LDAPService,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "RedfishService")]
        RedfishService,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccountService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetAfter"
        )]
        pub account_lockout_counter_reset_after: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetEnabled"
        )]
        pub account_lockout_counter_reset_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutDuration"
        )]
        pub account_lockout_duration: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutThreshold"
        )]
        pub account_lockout_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accounts")]
        pub accounts: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::account_service::v1_7_7::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveDirectory")]
        pub active_directory: Option<crate::account_service::v1_7_7::ExternalAccountProvider>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalExternalAccountProviders"
        )]
        pub additional_external_account_providers: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthFailureLoggingThreshold"
        )]
        pub auth_failure_logging_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAP")]
        pub ldap: Option<crate::account_service::v1_7_7::ExternalAccountProvider>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAccountAuth")]
        pub local_account_auth: Option<crate::account_service::v1_7_7::LocalAccountAuth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::account_service::v1_7_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type: Option<crate::account_service::v1_7_7::AuthenticationTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KerberosKeytab")]
        pub kerberos_keytab: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Token")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationTypes {
        #[default]
        #[serde(rename = "KerberosKeytab")]
        KerberosKeytab,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Token")]
        Token,
        #[serde(rename = "UsernameAndPassword")]
        UsernameAndPassword,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExternalAccountProvider {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountProviderType"
        )]
        pub account_provider_type: Option<crate::account_service::v1_7_7::AccountProviderTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::account_service::v1_7_7::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::account_service::v1_7_7::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordSet")]
        pub password_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping: Option<Vec<crate::account_service::v1_7_7::RoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPSearchSettings {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BaseDistinguishedNames"
        )]
        pub base_distinguished_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupNameAttribute")]
        pub group_name_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupsAttribute")]
        pub groups_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UsernameAttribute")]
        pub username_attribute: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SearchSettings")]
        pub search_settings: Option<crate::account_service::v1_7_7::LDAPSearchSettings>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocalAccountAuth {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Fallback")]
        Fallback,
        #[serde(rename = "LocalFirst")]
        LocalFirst,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RoleMapping {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalRole")]
        pub local_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteGroup")]
        pub remote_group: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteUser")]
        pub remote_user: Option<String>,
    }
}
pub mod v1_8_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountProviderTypes {
        #[default]
        #[serde(rename = "ActiveDirectoryService")]
        ActiveDirectoryService,
        #[serde(rename = "LDAPService")]
        LDAPService,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "RedfishService")]
        RedfishService,
        #[serde(rename = "TACACSplus")]
        TACACSplus,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccountService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetAfter"
        )]
        pub account_lockout_counter_reset_after: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetEnabled"
        )]
        pub account_lockout_counter_reset_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutDuration"
        )]
        pub account_lockout_duration: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutThreshold"
        )]
        pub account_lockout_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accounts")]
        pub accounts: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::account_service::v1_8_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveDirectory")]
        pub active_directory: Option<crate::account_service::v1_8_4::ExternalAccountProvider>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalExternalAccountProviders"
        )]
        pub additional_external_account_providers: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthFailureLoggingThreshold"
        )]
        pub auth_failure_logging_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAP")]
        pub ldap: Option<crate::account_service::v1_8_4::ExternalAccountProvider>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAccountAuth")]
        pub local_account_auth: Option<crate::account_service::v1_8_4::LocalAccountAuth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestrictedOemPrivileges"
        )]
        pub restricted_oem_privileges: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestrictedPrivileges"
        )]
        pub restricted_privileges: Option<Vec<crate::privileges::PrivilegeType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedAccountTypes"
        )]
        pub supported_account_types: Option<Vec<crate::manager_account::AccountTypes>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedOEMAccountTypes"
        )]
        pub supported_oem_account_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplus")]
        pub tacac_splus: Option<crate::account_service::v1_8_4::ExternalAccountProvider>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::account_service::v1_8_4::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type: Option<crate::account_service::v1_8_4::AuthenticationTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKey")]
        pub encryption_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKeySet")]
        pub encryption_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KerberosKeytab")]
        pub kerberos_keytab: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Token")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationTypes {
        #[default]
        #[serde(rename = "KerberosKeytab")]
        KerberosKeytab,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Token")]
        Token,
        #[serde(rename = "UsernameAndPassword")]
        UsernameAndPassword,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExternalAccountProvider {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountProviderType"
        )]
        pub account_provider_type: Option<crate::account_service::v1_8_4::AccountProviderTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::account_service::v1_8_4::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::account_service::v1_8_4::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordSet")]
        pub password_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
        pub priority: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping: Option<Vec<crate::account_service::v1_8_4::RoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplusService")]
        pub tacac_splus_service: Option<crate::account_service::v1_8_4::TACACSplusService>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPSearchSettings {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BaseDistinguishedNames"
        )]
        pub base_distinguished_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupNameAttribute")]
        pub group_name_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupsAttribute")]
        pub groups_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UsernameAttribute")]
        pub username_attribute: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SearchSettings")]
        pub search_settings: Option<crate::account_service::v1_8_4::LDAPSearchSettings>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocalAccountAuth {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Fallback")]
        Fallback,
        #[serde(rename = "LocalFirst")]
        LocalFirst,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RoleMapping {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalRole")]
        pub local_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteGroup")]
        pub remote_group: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteUser")]
        pub remote_user: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TACACSplusPasswordExchangeProtocol {
        #[default]
        #[serde(rename = "ASCII")]
        ASCII,
        #[serde(rename = "CHAP")]
        CHAP,
        #[serde(rename = "MSCHAPv1")]
        MSCHAPv1,
        #[serde(rename = "MSCHAPv2")]
        MSCHAPv2,
        #[serde(rename = "PAP")]
        PAP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TACACSplusService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PasswordExchangeProtocols"
        )]
        pub password_exchange_protocols:
            Option<Vec<crate::account_service::v1_8_4::TACACSplusPasswordExchangeProtocol>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrivilegeLevelArgument"
        )]
        pub privilege_level_argument: Option<String>,
    }
}
pub mod v1_9_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountProviderTypes {
        #[default]
        #[serde(rename = "ActiveDirectoryService")]
        ActiveDirectoryService,
        #[serde(rename = "LDAPService")]
        LDAPService,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "RedfishService")]
        RedfishService,
        #[serde(rename = "TACACSplus")]
        TACACSplus,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AccountService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetAfter"
        )]
        pub account_lockout_counter_reset_after: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutCounterResetEnabled"
        )]
        pub account_lockout_counter_reset_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutDuration"
        )]
        pub account_lockout_duration: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountLockoutThreshold"
        )]
        pub account_lockout_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accounts")]
        pub accounts: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::account_service::v1_9_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveDirectory")]
        pub active_directory: Option<crate::account_service::v1_9_3::ExternalAccountProvider>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalExternalAccountProviders"
        )]
        pub additional_external_account_providers: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthFailureLoggingThreshold"
        )]
        pub auth_failure_logging_threshold: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAP")]
        pub ldap: Option<crate::account_service::v1_9_3::ExternalAccountProvider>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAccountAuth")]
        pub local_account_auth: Option<crate::account_service::v1_9_3::LocalAccountAuth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
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
            rename = "PasswordExpirationDays"
        )]
        pub password_expiration_days: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestrictedOemPrivileges"
        )]
        pub restricted_oem_privileges: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RestrictedPrivileges"
        )]
        pub restricted_privileges: Option<Vec<crate::privileges::PrivilegeType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Roles")]
        pub roles: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedAccountTypes"
        )]
        pub supported_account_types: Option<Vec<crate::manager_account::AccountTypes>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedOEMAccountTypes"
        )]
        pub supported_oem_account_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplus")]
        pub tacac_splus: Option<crate::account_service::v1_9_3::ExternalAccountProvider>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::account_service::v1_9_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type: Option<crate::account_service::v1_9_3::AuthenticationTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKey")]
        pub encryption_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionKeySet")]
        pub encryption_key_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KerberosKeytab")]
        pub kerberos_keytab: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Token")]
        pub token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationTypes {
        #[default]
        #[serde(rename = "KerberosKeytab")]
        KerberosKeytab,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Token")]
        Token,
        #[serde(rename = "UsernameAndPassword")]
        UsernameAndPassword,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExternalAccountProvider {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountProviderType"
        )]
        pub account_provider_type: Option<crate::account_service::v1_9_3::AccountProviderTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::account_service::v1_9_3::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::account_service::v1_9_3::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordSet")]
        pub password_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
        pub priority: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping: Option<Vec<crate::account_service::v1_9_3::RoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplusService")]
        pub tacac_splus_service: Option<crate::account_service::v1_9_3::TACACSplusService>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPSearchSettings {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BaseDistinguishedNames"
        )]
        pub base_distinguished_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupNameAttribute")]
        pub group_name_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupsAttribute")]
        pub groups_attribute: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UsernameAttribute")]
        pub username_attribute: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SearchSettings")]
        pub search_settings: Option<crate::account_service::v1_9_3::LDAPSearchSettings>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocalAccountAuth {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Fallback")]
        Fallback,
        #[serde(rename = "LocalFirst")]
        LocalFirst,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RoleMapping {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalRole")]
        pub local_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteGroup")]
        pub remote_group: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteUser")]
        pub remote_user: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TACACSplusPasswordExchangeProtocol {
        #[default]
        #[serde(rename = "ASCII")]
        ASCII,
        #[serde(rename = "CHAP")]
        CHAP,
        #[serde(rename = "MSCHAPv1")]
        MSCHAPv1,
        #[serde(rename = "MSCHAPv2")]
        MSCHAPv2,
        #[serde(rename = "PAP")]
        PAP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TACACSplusService {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PasswordExchangeProtocols"
        )]
        pub password_exchange_protocols:
            Option<Vec<crate::account_service::v1_9_3::TACACSplusPasswordExchangeProtocol>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrivilegeLevelArgument"
        )]
        pub privilege_level_argument: Option<String>,
    }
}
