use serde::{Deserialize, Serialize};
pub type AccountService = crate::account_service::v1_18_0::AccountService;
pub type MFABypass = crate::account_service::v1_18_0::MFABypass;
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
    #[serde(rename = "OneTimePasscode")]
    OneTimePasscode,
    #[serde(rename = "SecurID")]
    SecurID,
    #[serde(rename = "TimeBasedOneTimePassword")]
    TimeBasedOneTimePassword,
}
pub mod v1_15_1 {
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
        pub actions: Option<crate::account_service::v1_15_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveDirectory")]
        pub active_directory: Option<crate::account_service::v1_15_1::ExternalAccountProvider>,
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
        pub description: Option<crate::account_service::v1_15_1::AccountServiceDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HTTPBasicAuth")]
        pub http_basic_auth: Option<crate::account_service::v1_15_1::AccountServiceHTTPBasicAuth>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAP")]
        pub ldap: Option<crate::account_service::v1_15_1::ExternalAccountProvider>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAccountAuth")]
        pub local_account_auth: Option<crate::account_service::v1_15_1::LocalAccountAuth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultiFactorAuth")]
        pub multi_factor_auth:
            Option<crate::account_service::v1_15_1::AccountServiceMultiFactorAuth>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OAuth2")]
        pub oauth2: Option<crate::account_service::v1_15_1::AccountServiceOAuth2>,
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
            rename = "OutboundConnections"
        )]
        pub outbound_connections:
            Option<crate::account_service::v1_15_1::AccountServiceOutboundConnections>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PasswordExpirationDays"
        )]
        pub password_expiration_days: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequireChangePasswordAction"
        )]
        pub require_change_password_action: Option<bool>,
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
        pub tacac_splus: Option<crate::account_service::v1_15_1::AccountServiceTACACSplus>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccountServiceDescription {
        V000001(crate::account_service::v1_15_1::AccountServiceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for AccountServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccountServiceHTTPBasicAuth {
        V011501(crate::account_service::v1_15_1::BasicAuthState),
        V000001(crate::account_service::v1_15_1::AccountServiceHTTPBasicAuthN1),
    }
    impl Default for AccountServiceHTTPBasicAuth {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountServiceHTTPBasicAuthN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccountServiceMultiFactorAuth {
        V011501(crate::account_service::v1_15_1::MultiFactorAuth),
        V000001(crate::account_service::v1_15_1::AccountServiceMultiFactorAuthN1),
    }
    impl Default for AccountServiceMultiFactorAuth {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountServiceMultiFactorAuthN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccountServiceOAuth2 {
        V011501(crate::account_service::v1_15_1::ExternalAccountProvider),
        V000001(crate::account_service::v1_15_1::AccountServiceOAuth2N1),
    }
    impl Default for AccountServiceOAuth2 {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountServiceOAuth2N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccountServiceOutboundConnections {
        V000001(crate::account_service::v1_15_1::AccountServiceOutboundConnectionsN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for AccountServiceOutboundConnections {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountServiceOutboundConnectionsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccountServiceTACACSplus {
        V011501(crate::account_service::v1_15_1::ExternalAccountProvider),
        V000001(crate::account_service::v1_15_1::AccountServiceTACACSplusN1),
    }
    impl Default for AccountServiceTACACSplus {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountServiceTACACSplusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::account_service::v1_15_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type:
            Option<crate::account_service::v1_15_1::AuthenticationAuthenticationType>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AuthenticationAuthenticationType {
        V011501(crate::account_service::v1_15_1::AuthenticationTypes),
        V000001(crate::account_service::v1_15_1::AuthenticationAuthenticationTypeN1),
    }
    impl Default for AuthenticationAuthenticationType {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationAuthenticationTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    pub enum BasicAuthState {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Unadvertised")]
        Unadvertised,
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
            Option<crate::account_service::v1_15_1::ClientCertificateCertificateMappingAttribute>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ClientCertificateCertificateMappingAttribute {
        V011501(crate::account_service::v1_15_1::CertificateMappingAttribute),
        V000001(crate::account_service::v1_15_1::ClientCertificateCertificateMappingAttributeN1),
    }
    impl Default for ClientCertificateCertificateMappingAttribute {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ClientCertificateCertificateMappingAttributeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExternalAccountProvider {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountProviderType"
        )]
        pub account_provider_type:
            Option<crate::account_service::v1_15_1::ExternalAccountProviderAccountProviderType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::account_service::v1_15_1::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::account_service::v1_15_1::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OAuth2Service")]
        pub oauth2_service:
            Option<crate::account_service::v1_15_1::ExternalAccountProviderOAuth2Service>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordSet")]
        pub password_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
        pub priority: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping:
            Option<Vec<crate::account_service::v1_15_1::ExternalAccountProviderRemoteRoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Retries")]
        pub retries: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplusService")]
        pub tacac_splus_service:
            Option<crate::account_service::v1_15_1::ExternalAccountProviderTACACSplusService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimeoutSeconds")]
        pub timeout_seconds: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ExternalAccountProviderAccountProviderType {
        V011501(crate::account_service::v1_15_1::AccountProviderTypes),
        V000001(crate::account_service::v1_15_1::ExternalAccountProviderAccountProviderTypeN1),
    }
    impl Default for ExternalAccountProviderAccountProviderType {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ExternalAccountProviderAccountProviderTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ExternalAccountProviderOAuth2Service {
        V011501(crate::account_service::v1_15_1::OAuth2Service),
        V000001(crate::account_service::v1_15_1::ExternalAccountProviderOAuth2ServiceN1),
    }
    impl Default for ExternalAccountProviderOAuth2Service {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ExternalAccountProviderOAuth2ServiceN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ExternalAccountProviderRemoteRoleMapping {
        V011501(crate::account_service::v1_15_1::RoleMapping),
        V000001(crate::account_service::v1_15_1::ExternalAccountProviderRemoteRoleMappingN1),
    }
    impl Default for ExternalAccountProviderRemoteRoleMapping {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ExternalAccountProviderRemoteRoleMappingN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ExternalAccountProviderTACACSplusService {
        V011501(crate::account_service::v1_15_1::TACACSplusService),
        V000001(crate::account_service::v1_15_1::ExternalAccountProviderTACACSplusServiceN1),
    }
    impl Default for ExternalAccountProviderTACACSplusService {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ExternalAccountProviderTACACSplusServiceN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub search_settings: Option<crate::account_service::v1_15_1::LDAPSearchSettings>,
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
        pub bypass_types: Option<Vec<crate::account_service::v1_15_1::MFABypassBypassTypes>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MFABypassBypassTypes {
        V000001(crate::account_service::v1_15_1::MFABypassBypassTypesN1),
        AccountServiceMFABypassType(crate::account_service::MFABypassType),
    }
    impl Default for MFABypassBypassTypes {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MFABypassBypassTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub client_certificate:
            Option<crate::account_service::v1_15_1::MultiFactorAuthClientCertificate>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GoogleAuthenticator"
        )]
        pub google_authenticator:
            Option<crate::account_service::v1_15_1::MultiFactorAuthGoogleAuthenticator>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MicrosoftAuthenticator"
        )]
        pub microsoft_authenticator:
            Option<crate::account_service::v1_15_1::MultiFactorAuthMicrosoftAuthenticator>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OneTimePasscode")]
        pub one_time_passcode:
            Option<crate::account_service::v1_15_1::MultiFactorAuthOneTimePasscode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurID")]
        pub secur_id: Option<crate::account_service::v1_15_1::MultiFactorAuthSecurID>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MultiFactorAuthClientCertificate {
        V011501(crate::account_service::v1_15_1::ClientCertificate),
        V000001(crate::account_service::v1_15_1::MultiFactorAuthClientCertificateN1),
    }
    impl Default for MultiFactorAuthClientCertificate {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MultiFactorAuthClientCertificateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MultiFactorAuthGoogleAuthenticator {
        V011501(crate::account_service::v1_15_1::GoogleAuthenticator),
        V000001(crate::account_service::v1_15_1::MultiFactorAuthGoogleAuthenticatorN1),
    }
    impl Default for MultiFactorAuthGoogleAuthenticator {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MultiFactorAuthGoogleAuthenticatorN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MultiFactorAuthMicrosoftAuthenticator {
        V011501(crate::account_service::v1_15_1::MicrosoftAuthenticator),
        V000001(crate::account_service::v1_15_1::MultiFactorAuthMicrosoftAuthenticatorN1),
    }
    impl Default for MultiFactorAuthMicrosoftAuthenticator {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MultiFactorAuthMicrosoftAuthenticatorN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MultiFactorAuthOneTimePasscode {
        V011501(crate::account_service::v1_15_1::OneTimePasscode),
        V000001(crate::account_service::v1_15_1::MultiFactorAuthOneTimePasscodeN1),
    }
    impl Default for MultiFactorAuthOneTimePasscode {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MultiFactorAuthOneTimePasscodeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MultiFactorAuthSecurID {
        V011501(crate::account_service::v1_15_1::SecurID),
        V000001(crate::account_service::v1_15_1::MultiFactorAuthSecurIDN1),
    }
    impl Default for MultiFactorAuthSecurID {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MultiFactorAuthSecurIDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub mode: Option<crate::account_service::v1_15_1::OAuth2Mode>,
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
    pub struct OneTimePasscode {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RoleMapping {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalRole")]
        pub local_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MFABypass")]
        pub mfa_bypass: Option<crate::account_service::v1_15_1::RoleMappingMFABypass>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteGroup")]
        pub remote_group: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteUser")]
        pub remote_user: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum RoleMappingMFABypass {
        V000001(crate::account_service::v1_15_1::RoleMappingMFABypassN1),
        AccountServiceMFABypass(crate::account_service::v1_18_0::MFABypass),
    }
    impl Default for RoleMappingMFABypass {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RoleMappingMFABypassN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub password_exchange_protocols: Option<
            Vec<crate::account_service::v1_15_1::TACACSplusServicePasswordExchangeProtocols>,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrivilegeLevelArgument"
        )]
        pub privilege_level_argument: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TACACSplusServicePasswordExchangeProtocols {
        V011501(crate::account_service::v1_15_1::TACACSplusPasswordExchangeProtocol),
        V000001(crate::account_service::v1_15_1::TACACSplusServicePasswordExchangeProtocolsN1),
    }
    impl Default for TACACSplusServicePasswordExchangeProtocols {
        fn default() -> Self {
            Self::V011501(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TACACSplusServicePasswordExchangeProtocolsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
pub mod v1_18_0 {
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
        pub actions: Option<crate::account_service::v1_18_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveDirectory")]
        pub active_directory: Option<crate::account_service::v1_18_0::ExternalAccountProvider>,
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
        pub description: Option<crate::account_service::v1_18_0::AccountServiceDescription>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnforcePasswordHistoryCount"
        )]
        pub enforce_password_history_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HTTPBasicAuth")]
        pub http_basic_auth: Option<crate::account_service::v1_18_0::AccountServiceHTTPBasicAuth>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAP")]
        pub ldap: Option<crate::account_service::v1_18_0::ExternalAccountProvider>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAccountAuth")]
        pub local_account_auth: Option<crate::account_service::v1_18_0::LocalAccountAuth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxPasswordLength")]
        pub max_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinPasswordLength")]
        pub min_password_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MultiFactorAuth")]
        pub multi_factor_auth:
            Option<crate::account_service::v1_18_0::AccountServiceMultiFactorAuth>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OAuth2")]
        pub oauth2: Option<crate::account_service::v1_18_0::AccountServiceOAuth2>,
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
            rename = "OutboundConnections"
        )]
        pub outbound_connections:
            Option<crate::account_service::v1_18_0::AccountServiceOutboundConnections>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PasswordExpirationDays"
        )]
        pub password_expiration_days: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PasswordGuidanceMessage"
        )]
        pub password_guidance_message: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PasswordGuidanceMessageId"
        )]
        pub password_guidance_message_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PrivilegeMap")]
        pub privilege_map: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequireChangePasswordAction"
        )]
        pub require_change_password_action: Option<bool>,
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
        pub tacac_splus: Option<crate::account_service::v1_18_0::AccountServiceTACACSplus>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccountServiceDescription {
        V000001(crate::account_service::v1_18_0::AccountServiceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for AccountServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccountServiceHTTPBasicAuth {
        V011800(crate::account_service::v1_18_0::BasicAuthState),
        V000001(crate::account_service::v1_18_0::AccountServiceHTTPBasicAuthN1),
    }
    impl Default for AccountServiceHTTPBasicAuth {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountServiceHTTPBasicAuthN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccountServiceMultiFactorAuth {
        V011800(crate::account_service::v1_18_0::MultiFactorAuth),
        V000001(crate::account_service::v1_18_0::AccountServiceMultiFactorAuthN1),
    }
    impl Default for AccountServiceMultiFactorAuth {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountServiceMultiFactorAuthN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccountServiceOAuth2 {
        V011800(crate::account_service::v1_18_0::ExternalAccountProvider),
        V000001(crate::account_service::v1_18_0::AccountServiceOAuth2N1),
    }
    impl Default for AccountServiceOAuth2 {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountServiceOAuth2N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccountServiceOutboundConnections {
        V000001(crate::account_service::v1_18_0::AccountServiceOutboundConnectionsN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for AccountServiceOutboundConnections {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountServiceOutboundConnectionsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AccountServiceTACACSplus {
        V011800(crate::account_service::v1_18_0::ExternalAccountProvider),
        V000001(crate::account_service::v1_18_0::AccountServiceTACACSplusN1),
    }
    impl Default for AccountServiceTACACSplus {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AccountServiceTACACSplusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::account_service::v1_18_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Authentication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthenticationType")]
        pub authentication_type:
            Option<crate::account_service::v1_18_0::AuthenticationAuthenticationType>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum AuthenticationAuthenticationType {
        V011800(crate::account_service::v1_18_0::AuthenticationTypes),
        V000001(crate::account_service::v1_18_0::AuthenticationAuthenticationTypeN1),
    }
    impl Default for AuthenticationAuthenticationType {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthenticationAuthenticationTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    pub enum BasicAuthState {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Enabled")]
        Enabled,
        #[serde(rename = "Unadvertised")]
        Unadvertised,
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
            Option<crate::account_service::v1_18_0::ClientCertificateCertificateMappingAttribute>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ClientCertificateCertificateMappingAttribute {
        V011800(crate::account_service::v1_18_0::CertificateMappingAttribute),
        V000001(crate::account_service::v1_18_0::ClientCertificateCertificateMappingAttributeN1),
    }
    impl Default for ClientCertificateCertificateMappingAttribute {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ClientCertificateCertificateMappingAttributeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ExternalAccountProvider {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccountProviderType"
        )]
        pub account_provider_type:
            Option<crate::account_service::v1_18_0::ExternalAccountProviderAccountProviderType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Authentication")]
        pub authentication: Option<crate::account_service::v1_18_0::Authentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LDAPService")]
        pub ldap_service: Option<crate::account_service::v1_18_0::LDAPService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OAuth2Service")]
        pub oauth2_service:
            Option<crate::account_service::v1_18_0::ExternalAccountProviderOAuth2Service>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordSet")]
        pub password_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
        pub priority: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteRoleMapping")]
        pub remote_role_mapping:
            Option<Vec<crate::account_service::v1_18_0::ExternalAccountProviderRemoteRoleMapping>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Retries")]
        pub retries: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceAddresses")]
        pub service_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TACACSplusService")]
        pub tacac_splus_service:
            Option<crate::account_service::v1_18_0::ExternalAccountProviderTACACSplusService>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimeoutSeconds")]
        pub timeout_seconds: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ExternalAccountProviderAccountProviderType {
        V011800(crate::account_service::v1_18_0::AccountProviderTypes),
        V000001(crate::account_service::v1_18_0::ExternalAccountProviderAccountProviderTypeN1),
    }
    impl Default for ExternalAccountProviderAccountProviderType {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ExternalAccountProviderAccountProviderTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ExternalAccountProviderOAuth2Service {
        V011800(crate::account_service::v1_18_0::OAuth2Service),
        V000001(crate::account_service::v1_18_0::ExternalAccountProviderOAuth2ServiceN1),
    }
    impl Default for ExternalAccountProviderOAuth2Service {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ExternalAccountProviderOAuth2ServiceN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ExternalAccountProviderRemoteRoleMapping {
        V011800(crate::account_service::v1_18_0::RoleMapping),
        V000001(crate::account_service::v1_18_0::ExternalAccountProviderRemoteRoleMappingN1),
    }
    impl Default for ExternalAccountProviderRemoteRoleMapping {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ExternalAccountProviderRemoteRoleMappingN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ExternalAccountProviderTACACSplusService {
        V011800(crate::account_service::v1_18_0::TACACSplusService),
        V000001(crate::account_service::v1_18_0::ExternalAccountProviderTACACSplusServiceN1),
    }
    impl Default for ExternalAccountProviderTACACSplusService {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ExternalAccountProviderTACACSplusServiceN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub search_settings: Option<crate::account_service::v1_18_0::LDAPSearchSettings>,
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
        pub bypass_types: Option<Vec<crate::account_service::v1_18_0::MFABypassBypassTypes>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MFABypassBypassTypes {
        V000001(crate::account_service::v1_18_0::MFABypassBypassTypesN1),
        AccountServiceMFABypassType(crate::account_service::MFABypassType),
    }
    impl Default for MFABypassBypassTypes {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MFABypassBypassTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub client_certificate:
            Option<crate::account_service::v1_18_0::MultiFactorAuthClientCertificate>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "GoogleAuthenticator"
        )]
        pub google_authenticator:
            Option<crate::account_service::v1_18_0::MultiFactorAuthGoogleAuthenticator>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MicrosoftAuthenticator"
        )]
        pub microsoft_authenticator:
            Option<crate::account_service::v1_18_0::MultiFactorAuthMicrosoftAuthenticator>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OneTimePasscode")]
        pub one_time_passcode:
            Option<crate::account_service::v1_18_0::MultiFactorAuthOneTimePasscode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SecurID")]
        pub secur_id: Option<crate::account_service::v1_18_0::MultiFactorAuthSecurID>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TimeBasedOneTimePassword"
        )]
        pub time_based_one_time_password:
            Option<crate::account_service::v1_18_0::MultiFactorAuthTimeBasedOneTimePassword>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MultiFactorAuthClientCertificate {
        V011800(crate::account_service::v1_18_0::ClientCertificate),
        V000001(crate::account_service::v1_18_0::MultiFactorAuthClientCertificateN1),
    }
    impl Default for MultiFactorAuthClientCertificate {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MultiFactorAuthClientCertificateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MultiFactorAuthGoogleAuthenticator {
        V011800(crate::account_service::v1_18_0::GoogleAuthenticator),
        V000001(crate::account_service::v1_18_0::MultiFactorAuthGoogleAuthenticatorN1),
    }
    impl Default for MultiFactorAuthGoogleAuthenticator {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MultiFactorAuthGoogleAuthenticatorN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MultiFactorAuthMicrosoftAuthenticator {
        V011800(crate::account_service::v1_18_0::MicrosoftAuthenticator),
        V000001(crate::account_service::v1_18_0::MultiFactorAuthMicrosoftAuthenticatorN1),
    }
    impl Default for MultiFactorAuthMicrosoftAuthenticator {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MultiFactorAuthMicrosoftAuthenticatorN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MultiFactorAuthOneTimePasscode {
        V011800(crate::account_service::v1_18_0::OneTimePasscode),
        V000001(crate::account_service::v1_18_0::MultiFactorAuthOneTimePasscodeN1),
    }
    impl Default for MultiFactorAuthOneTimePasscode {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MultiFactorAuthOneTimePasscodeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MultiFactorAuthSecurID {
        V011800(crate::account_service::v1_18_0::SecurID),
        V000001(crate::account_service::v1_18_0::MultiFactorAuthSecurIDN1),
    }
    impl Default for MultiFactorAuthSecurID {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MultiFactorAuthSecurIDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MultiFactorAuthTimeBasedOneTimePassword {
        V011800(crate::account_service::v1_18_0::TimeBasedOneTimePassword),
        V000001(crate::account_service::v1_18_0::MultiFactorAuthTimeBasedOneTimePasswordN1),
    }
    impl Default for MultiFactorAuthTimeBasedOneTimePassword {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MultiFactorAuthTimeBasedOneTimePasswordN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub mode: Option<crate::account_service::v1_18_0::OAuth2Mode>,
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
    pub struct OneTimePasscode {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RoleMapping {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalAccountTypes")]
        pub local_account_types:
            Option<Vec<crate::account_service::v1_18_0::RoleMappingLocalAccountTypes>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocalOEMAccountTypes"
        )]
        pub local_oem_account_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocalRole")]
        pub local_role: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MFABypass")]
        pub mfa_bypass: Option<crate::account_service::v1_18_0::RoleMappingMFABypass>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteGroup")]
        pub remote_group: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemoteUser")]
        pub remote_user: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum RoleMappingLocalAccountTypes {
        V000001(crate::account_service::v1_18_0::RoleMappingLocalAccountTypesN1),
        ManagerAccountAccountTypes(crate::manager_account::AccountTypes),
    }
    impl Default for RoleMappingLocalAccountTypes {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RoleMappingLocalAccountTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum RoleMappingMFABypass {
        V000001(crate::account_service::v1_18_0::RoleMappingMFABypassN1),
        AccountServiceMFABypass(crate::account_service::v1_18_0::MFABypass),
    }
    impl Default for RoleMappingMFABypass {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RoleMappingMFABypassN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub password_exchange_protocols: Option<
            Vec<crate::account_service::v1_18_0::TACACSplusServicePasswordExchangeProtocols>,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrivilegeLevelArgument"
        )]
        pub privilege_level_argument: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TACACSplusServicePasswordExchangeProtocols {
        V011800(crate::account_service::v1_18_0::TACACSplusPasswordExchangeProtocol),
        V000001(crate::account_service::v1_18_0::TACACSplusServicePasswordExchangeProtocolsN1),
    }
    impl Default for TACACSplusServicePasswordExchangeProtocols {
        fn default() -> Self {
            Self::V011800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TACACSplusServicePasswordExchangeProtocolsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TimeBasedOneTimePassword {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimeStepSeconds")]
        pub time_step_seconds: Option<i64>,
    }
}
