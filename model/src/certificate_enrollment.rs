use serde::{Deserialize, Serialize};
pub type CertificateEnrollment = crate::certificate_enrollment::v1_0_0::CertificateEnrollment;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum EnrollmentProtocolType {
    #[default]
    #[serde(rename = "ACME")]
    ACME,
    #[serde(rename = "OEM")]
    OEM,
    #[serde(rename = "SCEP")]
    SCEP,
}
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ACMEChallengeType {
        #[default]
        #[serde(rename = "Dns01")]
        Dns01,
        #[serde(rename = "Http01")]
        Http01,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ACMEConfiguration {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChallengeType")]
        pub challenge_type:
            Option<crate::certificate_enrollment::v1_0_0::ACMEConfigurationChallengeType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EABKey")]
        pub eab_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EABKeyId")]
        pub eab_key_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Email")]
        pub email: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ACMEConfigurationChallengeType {
        V010000(crate::certificate_enrollment::v1_0_0::ACMEChallengeType),
        V000001(crate::certificate_enrollment::v1_0_0::ACMEConfigurationChallengeTypeN1),
    }
    impl Default for ACMEConfigurationChallengeType {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ACMEConfigurationChallengeTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::certificate_enrollment::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CSRParameters {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlternativeNames")]
        pub alternative_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChallengePassword")]
        pub challenge_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CommonName")]
        pub common_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContactPerson")]
        pub contact_person: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Email")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GivenName")]
        pub given_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Initials")]
        pub initials: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyBitLength")]
        pub key_bit_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyCurveId")]
        pub key_curve_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyPairAlgorithm")]
        pub key_pair_algorithm: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyUsage")]
        pub key_usage: Option<Vec<crate::certificate_enrollment::v1_0_0::CSRParametersKeyUsage>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Organization")]
        pub organization: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OrganizationalUnit")]
        pub organizational_unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "State")]
        pub state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Surname")]
        pub surname: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UnstructuredName")]
        pub unstructured_name: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CSRParametersKeyUsage {
        V000001(crate::certificate_enrollment::v1_0_0::CSRParametersKeyUsageN1),
        CertificateKeyUsage(crate::certificate::KeyUsage),
    }
    impl Default for CSRParametersKeyUsage {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CSRParametersKeyUsageN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CertificateEnrollment {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ACME")]
        pub acme: Option<crate::certificate_enrollment::v1_0_0::CertificateEnrollmentACME>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::certificate_enrollment::v1_0_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CSRParameters")]
        pub csr_parameters:
            Option<crate::certificate_enrollment::v1_0_0::CertificateEnrollmentCSRParameters>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description:
            Option<crate::certificate_enrollment::v1_0_0::CertificateEnrollmentDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnrollmentState")]
        pub enrollment_state:
            Option<crate::certificate_enrollment::v1_0_0::CertificateEnrollmentEnrollmentState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnrollmentType")]
        pub enrollment_type: Option<crate::certificate_enrollment::EnrollmentProtocolType>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::certificate_enrollment::v1_0_0::Links>,
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
            rename = "RenewBeforeExpiryDays"
        )]
        pub renew_before_expiry_days: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SCEP")]
        pub scep: Option<crate::certificate_enrollment::v1_0_0::CertificateEnrollmentSCEP>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServerURI")]
        pub server_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VerifyCertificate")]
        pub verify_certificate: Option<bool>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CertificateEnrollmentACME {
        V010000(crate::certificate_enrollment::v1_0_0::ACMEConfiguration),
        V000001(crate::certificate_enrollment::v1_0_0::CertificateEnrollmentACMEN1),
    }
    impl Default for CertificateEnrollmentACME {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CertificateEnrollmentACMEN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CertificateEnrollmentCSRParameters {
        V010000(crate::certificate_enrollment::v1_0_0::CSRParameters),
        V000001(crate::certificate_enrollment::v1_0_0::CertificateEnrollmentCSRParametersN1),
    }
    impl Default for CertificateEnrollmentCSRParameters {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CertificateEnrollmentCSRParametersN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CertificateEnrollmentDescription {
        V000001(crate::certificate_enrollment::v1_0_0::CertificateEnrollmentDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for CertificateEnrollmentDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CertificateEnrollmentDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CertificateEnrollmentEnrollmentState {
        V010000(crate::certificate_enrollment::v1_0_0::EnrollmentState),
        V000001(crate::certificate_enrollment::v1_0_0::CertificateEnrollmentEnrollmentStateN1),
    }
    impl Default for CertificateEnrollmentEnrollmentState {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CertificateEnrollmentEnrollmentStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CertificateEnrollmentSCEP {
        V010000(crate::certificate_enrollment::v1_0_0::SCEPConfiguration),
        V000001(crate::certificate_enrollment::v1_0_0::CertificateEnrollmentSCEPN1),
    }
    impl Default for CertificateEnrollmentSCEP {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CertificateEnrollmentSCEPN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EnrollmentState {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastOperation")]
        pub last_operation:
            Option<crate::certificate_enrollment::v1_0_0::EnrollmentStateLastOperation>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastOperationStatus"
        )]
        pub last_operation_status:
            Option<crate::certificate_enrollment::v1_0_0::EnrollmentStateLastOperationStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastOperationTime")]
        pub last_operation_time: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnrollmentStateLastOperation {
        V010000(crate::certificate_enrollment::v1_0_0::LastOperationType),
        V000001(crate::certificate_enrollment::v1_0_0::EnrollmentStateLastOperationN1),
    }
    impl Default for EnrollmentStateLastOperation {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnrollmentStateLastOperationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnrollmentStateLastOperationStatus {
        V010000(crate::certificate_enrollment::v1_0_0::OperationStatus),
        V000001(crate::certificate_enrollment::v1_0_0::EnrollmentStateLastOperationStatusN1),
    }
    impl Default for EnrollmentStateLastOperationStatus {
        fn default() -> Self {
            Self::V010000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnrollmentStateLastOperationStatusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LastOperationType {
        #[default]
        #[serde(rename = "Renew")]
        Renew,
        #[serde(rename = "UpdateAcmeEmail")]
        UpdateAcmeEmail,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CACertificates")]
        pub ca_certificates: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CACertificates@odata.count"
        )]
        pub ca_certificates_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnrolledCertificate"
        )]
        pub enrolled_certificate:
            Option<crate::certificate_enrollment::v1_0_0::LinksEnrolledCertificate>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnrolledCertificateLocation"
        )]
        pub enrolled_certificate_location:
            Option<crate::certificate_enrollment::v1_0_0::LinksEnrolledCertificateLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksEnrolledCertificate {
        V000001(crate::certificate_enrollment::v1_0_0::LinksEnrolledCertificateN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksEnrolledCertificate {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksEnrolledCertificateLocation {
        V000001(crate::certificate_enrollment::v1_0_0::LinksEnrolledCertificateLocationN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksEnrolledCertificateLocation {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksEnrolledCertificateLocationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksEnrolledCertificateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OperationStatus {
        #[default]
        #[serde(rename = "Failed")]
        Failed,
        #[serde(rename = "InProgress")]
        InProgress,
        #[serde(rename = "Success")]
        Success,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SCEPConfiguration {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChallengePassword")]
        pub challenge_password: Option<String>,
    }
}
