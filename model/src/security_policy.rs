pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::security_policy::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMAlgorithmSet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AEAD")]
        pub aead: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseAsym")]
        pub base_asym: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseHash")]
        pub base_hash: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMParameterSet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Algorithms")]
        pub algorithms: Option<crate::security_policy::v1_0_0::SPDMAlgorithmSet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Versions")]
        pub versions: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMPolicy {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllowExtendedAlgorithms"
        )]
        pub allow_extended_algorithms: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Allowed")]
        pub allowed: Option<crate::security_policy::v1_0_0::SPDMParameterSet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Denied")]
        pub denied: Option<crate::security_policy::v1_0_0::SPDMParameterSet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RevokedCertificates"
        )]
        pub revoked_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecureSessionEnabled"
        )]
        pub secure_session_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrustedCertificates"
        )]
        pub trusted_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VerifyCertificate")]
        pub verify_certificate: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityPolicy {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::security_policy::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
            rename = "OverrideParentManager"
        )]
        pub override_parent_manager: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SPDM")]
        pub spdm: Option<crate::security_policy::v1_0_0::SPDMPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TLS")]
        pub tls: Option<crate::security_policy::v1_0_0::TLSCommunication>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TLSAlgorithmSet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CipherSuites")]
        pub cipher_suites: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SignatureAlgorithms"
        )]
        pub signature_algorithms: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TLSCommunication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Client")]
        pub client: Option<crate::security_policy::v1_0_0::TLSPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Server")]
        pub server: Option<crate::security_policy::v1_0_0::TLSPolicy>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TLSParameterSet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Algorithms")]
        pub algorithms: Option<crate::security_policy::v1_0_0::TLSAlgorithmSet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Versions")]
        pub versions: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TLSPolicy {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Allowed")]
        pub allowed: Option<crate::security_policy::v1_0_0::TLSParameterSet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Denied")]
        pub denied: Option<crate::security_policy::v1_0_0::TLSParameterSet>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RevokedCertificates"
        )]
        pub revoked_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrustedCertificates"
        )]
        pub trusted_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VerifyCertificate")]
        pub verify_certificate: Option<bool>,
    }
}
