use serde::{Deserialize, Serialize};
pub type Certificate = crate::certificate::v1_8_2::Certificate;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum CertificateType {
    #[default]
    #[serde(rename = "PEM")]
    PEM,
    #[serde(rename = "PEMchain")]
    PEMchain,
    #[serde(rename = "PKCS7")]
    PKCS7,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum KeyUsage {
    #[default]
    #[serde(rename = "CRLSigning")]
    CRLSigning,
    #[serde(rename = "ClientAuthentication")]
    ClientAuthentication,
    #[serde(rename = "CodeSigning")]
    CodeSigning,
    #[serde(rename = "DataEncipherment")]
    DataEncipherment,
    #[serde(rename = "DecipherOnly")]
    DecipherOnly,
    #[serde(rename = "DigitalSignature")]
    DigitalSignature,
    #[serde(rename = "EmailProtection")]
    EmailProtection,
    #[serde(rename = "EncipherOnly")]
    EncipherOnly,
    #[serde(rename = "KeyAgreement")]
    KeyAgreement,
    #[serde(rename = "KeyCertSign")]
    KeyCertSign,
    #[serde(rename = "KeyEncipherment")]
    KeyEncipherment,
    #[serde(rename = "NonRepudiation")]
    NonRepudiation,
    #[serde(rename = "OCSPSigning")]
    OCSPSigning,
    #[serde(rename = "ServerAuthentication")]
    ServerAuthentication,
    #[serde(rename = "Timestamping")]
    Timestamping,
}
pub mod v1_7_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Certificate.Rekey")]
        pub certificate_rekey: Option<crate::certificate::v1_7_0::Rekey>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Certificate.Renew")]
        pub certificate_renew: Option<crate::certificate::v1_7_0::Renew>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::certificate::v1_7_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Certificate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::certificate::v1_7_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CertificateString")]
        pub certificate_string: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CertificateType")]
        pub certificate_type: Option<crate::certificate::CertificateType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CertificateUsageTypes"
        )]
        pub certificate_usage_types: Option<Vec<crate::certificate::v1_7_0::CertificateUsageType>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fingerprint")]
        pub fingerprint: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FingerprintHashAlgorithm"
        )]
        pub fingerprint_hash_algorithm: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Issuer")]
        pub issuer: Option<crate::certificate::v1_7_0::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyUsage")]
        pub key_usage: Option<Vec<crate::certificate::KeyUsage>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::certificate::v1_7_0::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignatureAlgorithm")]
        pub signature_algorithm: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SPDM")]
        pub spdm: Option<crate::certificate::v1_7_0::SPDM>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Subject")]
        pub subject: Option<crate::certificate::v1_7_0::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UefiSignatureOwner")]
        pub uefi_signature_owner: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ValidNotAfter")]
        pub valid_not_after: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ValidNotBefore")]
        pub valid_not_before: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CertificateUsageType {
        #[default]
        #[serde(rename = "BIOS")]
        BIOS,
        #[serde(rename = "Device")]
        Device,
        #[serde(rename = "Platform")]
        Platform,
        #[serde(rename = "SSH")]
        SSH,
        #[serde(rename = "User")]
        User,
        #[serde(rename = "Web")]
        Web,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalCommonNames"
        )]
        pub additional_common_names: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalOrganizationalUnits"
        )]
        pub additional_organizational_units: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlternativeNames")]
        pub alternative_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CommonName")]
        pub common_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DisplayString")]
        pub display_string: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DomainComponents")]
        pub domain_components: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Email")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Organization")]
        pub organization: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OrganizationalUnit")]
        pub organizational_unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "State")]
        pub state: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Issuer")]
        pub issuer: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Subjects")]
        pub subjects: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Subjects@odata.count"
        )]
        pub subjects_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Rekey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RekeyRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChallengePassword")]
        pub challenge_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyBitLength")]
        pub key_bit_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyCurveId")]
        pub key_curve_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyPairAlgorithm")]
        pub key_pair_algorithm: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RekeyResponse {
        #[serde(rename = "Certificate")]
        pub certificate: crate::odata_v4::IdRef,
        #[serde(rename = "CSRString")]
        pub csr_string: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Renew {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RenewRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChallengePassword")]
        pub challenge_password: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RenewResponse {
        #[serde(rename = "Certificate")]
        pub certificate: crate::odata_v4::IdRef,
        #[serde(rename = "CSRString")]
        pub csr_string: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDM {
        #[serde(skip_serializing_if = "Option::is_none", rename = "SlotId")]
        pub slot_id: Option<i64>,
    }
}
pub mod v1_8_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Certificate.Rekey")]
        pub certificate_rekey: Option<crate::certificate::v1_8_2::Rekey>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Certificate.Renew")]
        pub certificate_renew: Option<crate::certificate::v1_8_2::Renew>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::certificate::v1_8_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Certificate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::certificate::v1_8_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CertificateString")]
        pub certificate_string: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CertificateType")]
        pub certificate_type: Option<crate::certificate::v1_8_2::CertificateCertificateType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CertificateUsageTypes"
        )]
        pub certificate_usage_types:
            Option<Vec<crate::certificate::v1_8_2::CertificateCertificateUsageTypes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::certificate::v1_8_2::CertificateDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fingerprint")]
        pub fingerprint: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FingerprintHashAlgorithm"
        )]
        pub fingerprint_hash_algorithm: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Issuer")]
        pub issuer: Option<crate::certificate::v1_8_2::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyUsage")]
        pub key_usage: Option<Vec<crate::certificate::v1_8_2::CertificateKeyUsage>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::certificate::v1_8_2::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignatureAlgorithm")]
        pub signature_algorithm: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SPDM")]
        pub spdm: Option<crate::certificate::v1_8_2::SPDM>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Subject")]
        pub subject: Option<crate::certificate::v1_8_2::Identifier>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UefiSignatureOwner")]
        pub uefi_signature_owner: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ValidNotAfter")]
        pub valid_not_after: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ValidNotBefore")]
        pub valid_not_before: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CertificateCertificateType {
        V000001(crate::certificate::v1_8_2::CertificateCertificateTypeN1),
        CertificateCertificateType(crate::certificate::CertificateType),
    }
    impl Default for CertificateCertificateType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CertificateCertificateTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CertificateCertificateUsageTypes {
        V010802(crate::certificate::v1_8_2::CertificateUsageType),
        V000001(crate::certificate::v1_8_2::CertificateCertificateUsageTypesN1),
    }
    impl Default for CertificateCertificateUsageTypes {
        fn default() -> Self {
            Self::V010802(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CertificateCertificateUsageTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CertificateDescription {
        V000001(crate::certificate::v1_8_2::CertificateDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for CertificateDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CertificateDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CertificateKeyUsage {
        V000001(crate::certificate::v1_8_2::CertificateKeyUsageN1),
        CertificateKeyUsage(crate::certificate::KeyUsage),
    }
    impl Default for CertificateKeyUsage {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CertificateKeyUsageN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CertificateUsageType {
        #[default]
        #[serde(rename = "BIOS")]
        BIOS,
        #[serde(rename = "Device")]
        Device,
        #[serde(rename = "IAK")]
        IAK,
        #[serde(rename = "IDevID")]
        IDevID,
        #[serde(rename = "LAK")]
        LAK,
        #[serde(rename = "LDevID")]
        LDevID,
        #[serde(rename = "Platform")]
        Platform,
        #[serde(rename = "SSH")]
        SSH,
        #[serde(rename = "User")]
        User,
        #[serde(rename = "Web")]
        Web,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalCommonNames"
        )]
        pub additional_common_names: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalOrganizationalUnits"
        )]
        pub additional_organizational_units: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlternativeNames")]
        pub alternative_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CommonName")]
        pub common_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DisplayString")]
        pub display_string: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DomainComponents")]
        pub domain_components: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Email")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Organization")]
        pub organization: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OrganizationalUnit")]
        pub organizational_unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "State")]
        pub state: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Issuer")]
        pub issuer: Option<crate::certificate::v1_8_2::LinksIssuer>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Subjects")]
        pub subjects: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Subjects@odata.count"
        )]
        pub subjects_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksIssuer {
        V000001(crate::certificate::v1_8_2::LinksIssuerN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksIssuer {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksIssuerN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Rekey {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RekeyRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChallengePassword")]
        pub challenge_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyBitLength")]
        pub key_bit_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyCurveId")]
        pub key_curve_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyPairAlgorithm")]
        pub key_pair_algorithm: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RekeyResponse {
        #[serde(rename = "Certificate")]
        pub certificate: crate::odata_v4::IdRef,
        #[serde(rename = "CSRString")]
        pub csr_string: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Renew {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RenewRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChallengePassword")]
        pub challenge_password: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RenewResponse {
        #[serde(rename = "Certificate")]
        pub certificate: crate::odata_v4::IdRef,
        #[serde(rename = "CSRString")]
        pub csr_string: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDM {
        #[serde(skip_serializing_if = "Option::is_none", rename = "SlotId")]
        pub slot_id: Option<i64>,
    }
}
