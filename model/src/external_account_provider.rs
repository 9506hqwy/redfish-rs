pub type ExternalAccountProvider =
    crate::external_account_provider::v1_7_1::ExternalAccountProvider;
pub mod v1_6_0 {
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
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::external_account_provider::v1_6_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type:
            Option<crate::external_account_provider::v1_6_0::AuthenticationTypes>,
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
        pub account_provider_type:
            Option<crate::external_account_provider::v1_6_0::AccountProviderTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::external_account_provider::v1_6_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::external_account_provider::v1_6_0::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::external_account_provider::v1_6_0::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::external_account_provider::v1_6_0::Links>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OAuth2Service")]
        pub oauth2_service: Option<crate::external_account_provider::v1_6_0::OAuth2Service>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
        pub priority: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping: Option<Vec<crate::external_account_provider::v1_6_0::RoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Retries")]
        pub retries: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplusService")]
        pub tacac_splus_service:
            Option<crate::external_account_provider::v1_6_0::TACACSplusService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimeoutSeconds")]
        pub timeout_seconds: Option<i64>,
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
        pub search_settings: Option<crate::external_account_provider::v1_6_0::LDAPSearchSettings>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
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
        pub mode: Option<crate::external_account_provider::v1_6_0::OAuth2Mode>,
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
        pub password_exchange_protocols: Option<
            Vec<crate::external_account_provider::v1_6_0::TACACSplusPasswordExchangeProtocol>,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrivilegeLevelArgument"
        )]
        pub privilege_level_argument: Option<String>,
    }
}
pub mod v1_7_1 {
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
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::external_account_provider::v1_7_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type:
            Option<crate::external_account_provider::v1_7_1::AuthenticationTypes>,
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
        pub account_provider_type:
            Option<crate::external_account_provider::v1_7_1::AccountProviderTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::external_account_provider::v1_7_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::external_account_provider::v1_7_1::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::external_account_provider::v1_7_1::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::external_account_provider::v1_7_1::Links>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OAuth2Service")]
        pub oauth2_service: Option<crate::external_account_provider::v1_7_1::OAuth2Service>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
        pub priority: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping: Option<Vec<crate::external_account_provider::v1_7_1::RoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Retries")]
        pub retries: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplusService")]
        pub tacac_splus_service:
            Option<crate::external_account_provider::v1_7_1::TACACSplusService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimeoutSeconds")]
        pub timeout_seconds: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LDAPSearchSettings {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BaseDistinguishedNames"
        )]
        pub base_distinguished_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAttribute")]
        pub email_attribute: Option<String>,
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
        pub search_settings: Option<crate::external_account_provider::v1_7_1::LDAPSearchSettings>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
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
        pub mode: Option<crate::external_account_provider::v1_7_1::OAuth2Mode>,
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
        pub password_exchange_protocols: Option<
            Vec<crate::external_account_provider::v1_7_1::TACACSplusPasswordExchangeProtocol>,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrivilegeLevelArgument"
        )]
        pub privilege_level_argument: Option<String>,
    }
}
