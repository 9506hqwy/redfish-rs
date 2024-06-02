pub type ComponentIntegrity = crate::component_integrity::v1_2_3::ComponentIntegrity;
pub mod v1_2_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComponentIntegrity.SPDMGetSignedMeasurements"
        )]
        pub component_integrity_spdm_get_signed_measurements:
            Option<crate::component_integrity::v1_2_2::SPDMGetSignedMeasurements>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComponentIntegrity.TPMGetSignedMeasurements"
        )]
        pub component_integrity_tpm_get_signed_measurements:
            Option<crate::component_integrity::v1_2_2::TPMGetSignedMeasurements>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::component_integrity::v1_2_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CommonAuthInfo {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentCertificate"
        )]
        pub component_certificate: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VerificationStatus")]
        pub verification_status: Option<crate::component_integrity::v1_2_2::VerificationStatus>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CommunicationInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sessions")]
        pub sessions: Option<Vec<crate::component_integrity::v1_2_2::SingleSessionInfo>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComponentIntegrity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::component_integrity::v1_2_2::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentIntegrityEnabled"
        )]
        pub component_integrity_enabled: Option<bool>,
        #[serde(rename = "ComponentIntegrityType")]
        pub component_integrity_type: crate::component_integrity::v1_2_2::ComponentIntegrityType,
        #[serde(rename = "ComponentIntegrityTypeVersion")]
        pub component_integrity_type_version: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastUpdated")]
        pub last_updated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::component_integrity::v1_2_2::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SPDM")]
        pub spdm: Option<crate::component_integrity::v1_2_2::SPDMinfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TargetComponentURI")]
        pub target_component_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TPM")]
        pub tpm: Option<crate::component_integrity::v1_2_2::TPMinfo>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ComponentIntegrityType {
        #[default]
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SPDM")]
        SPDM,
        #[serde(rename = "TPM")]
        TPM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DMTFmeasurementTypes {
        #[default]
        #[serde(rename = "FirmwareConfiguration")]
        FirmwareConfiguration,
        #[serde(rename = "HardwareConfiguration")]
        HardwareConfiguration,
        #[serde(rename = "ImmutableROM")]
        ImmutableROM,
        #[serde(rename = "MeasurementManifest")]
        MeasurementManifest,
        #[serde(rename = "MutableFirmware")]
        MutableFirmware,
        #[serde(rename = "MutableFirmwareSecurityVersionNumber")]
        MutableFirmwareSecurityVersionNumber,
        #[serde(rename = "MutableFirmwareVersion")]
        MutableFirmwareVersion,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentsProtected"
        )]
        pub components_protected: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentsProtected@odata.count"
        )]
        pub components_protected_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MeasurementSpecification {
        #[default]
        #[serde(rename = "DMTF")]
        DMTF,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMGetSignedMeasurements {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMGetSignedMeasurementsRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementIndices")]
        pub measurement_indices: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Nonce")]
        pub nonce: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SlotId")]
        pub slot_id: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMGetSignedMeasurementsResponse {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificate")]
        pub certificate: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "HashingAlgorithm")]
        pub hashing_algorithm: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PublicKey")]
        pub public_key: Option<String>,
        #[serde(rename = "SignedMeasurements")]
        pub signed_measurements: String,
        #[serde(rename = "SigningAlgorithm")]
        pub signing_algorithm: String,
        #[serde(rename = "Version")]
        pub version: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMcommunication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sessions")]
        pub sessions: Option<Vec<crate::component_integrity::v1_2_2::SingleSessionInfo>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMidentity {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequesterAuthentication"
        )]
        pub requester_authentication: Option<crate::component_integrity::v1_2_2::SPDMrequesterAuth>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResponderAuthentication"
        )]
        pub responder_authentication: Option<crate::component_integrity::v1_2_2::SPDMresponderAuth>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMinfo {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentCommunication"
        )]
        pub component_communication: Option<crate::component_integrity::v1_2_2::SPDMcommunication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IdentityAuthentication"
        )]
        pub identity_authentication: Option<crate::component_integrity::v1_2_2::SPDMidentity>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementSet")]
        pub measurement_set: Option<crate::component_integrity::v1_2_2::SPDMmeasurementSet>,
        #[serde(rename = "Requester")]
        pub requester: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMmeasurementSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MeasurementSpecification"
        )]
        pub measurement_specification:
            Option<crate::component_integrity::v1_2_2::MeasurementSpecification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementSummary")]
        pub measurement_summary: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MeasurementSummaryHashAlgorithm"
        )]
        pub measurement_summary_hash_algorithm: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MeasurementSummaryType"
        )]
        pub measurement_summary_type:
            Option<crate::component_integrity::v1_2_2::SPDMmeasurementSummaryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::component_integrity::v1_2_2::SPDMsingleMeasurement>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SPDMmeasurementSummaryType {
        #[default]
        #[serde(rename = "All")]
        All,
        #[serde(rename = "TCB")]
        TCB,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMrequesterAuth {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProvidedCertificate"
        )]
        pub provided_certificate: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMresponderAuth {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentCertificate"
        )]
        pub component_certificate: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VerificationStatus")]
        pub verification_status: Option<crate::component_integrity::v1_2_2::VerificationStatus>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMsingleMeasurement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastUpdated")]
        pub last_updated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurement")]
        pub measurement: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MeasurementHashAlgorithm"
        )]
        pub measurement_hash_algorithm: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementIndex")]
        pub measurement_index: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementType")]
        pub measurement_type: Option<crate::component_integrity::v1_2_2::DMTFmeasurementTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartofSummaryHash")]
        pub partof_summary_hash: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityVersionNumber"
        )]
        pub security_version_number: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecureSessionType {
        #[default]
        #[serde(rename = "AuthenticatedOnly")]
        AuthenticatedOnly,
        #[serde(rename = "EncryptedAuthenticated")]
        EncryptedAuthenticated,
        #[serde(rename = "Plain")]
        Plain,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SingleSessionInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionId")]
        pub session_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionType")]
        pub session_type: Option<crate::component_integrity::v1_2_2::SecureSessionType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMGetSignedMeasurements {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMGetSignedMeasurementsRequestBody {
        #[serde(rename = "Certificate")]
        pub certificate: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Nonce")]
        pub nonce: Option<String>,
        #[serde(rename = "PCRSelection")]
        pub pcr_selection: String,
        #[serde(rename = "Scheme")]
        pub scheme: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMGetSignedMeasurementsResponse {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "SignedMeasurements")]
        pub signed_measurements: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMauth {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentCertificate"
        )]
        pub component_certificate: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VerificationStatus")]
        pub verification_status: Option<crate::component_integrity::v1_2_2::VerificationStatus>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMcommunication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sessions")]
        pub sessions: Option<Vec<crate::component_integrity::v1_2_2::SingleSessionInfo>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMinfo {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentCommunication"
        )]
        pub component_communication: Option<crate::component_integrity::v1_2_2::TPMcommunication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IdentityAuthentication"
        )]
        pub identity_authentication: Option<crate::component_integrity::v1_2_2::TPMauth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementSet")]
        pub measurement_set: Option<crate::component_integrity::v1_2_2::TPMmeasurementSet>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NonceSizeBytesMaximum"
        )]
        pub nonce_size_bytes_maximum: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMmeasurementSet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::component_integrity::v1_2_2::TPMsingleMeasurement>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMsingleMeasurement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastUpdated")]
        pub last_updated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurement")]
        pub measurement: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MeasurementHashAlgorithm"
        )]
        pub measurement_hash_algorithm: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCR")]
        pub pcr: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VerificationStatus {
        #[default]
        #[serde(rename = "Failed")]
        Failed,
        #[serde(rename = "Success")]
        Success,
    }
}
pub mod v1_2_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComponentIntegrity.SPDMGetSignedMeasurements"
        )]
        pub component_integrity_spdm_get_signed_measurements:
            Option<crate::component_integrity::v1_2_3::SPDMGetSignedMeasurements>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComponentIntegrity.TPMGetSignedMeasurements"
        )]
        pub component_integrity_tpm_get_signed_measurements:
            Option<crate::component_integrity::v1_2_3::TPMGetSignedMeasurements>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::component_integrity::v1_2_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CommonAuthInfo {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentCertificate"
        )]
        pub component_certificate: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VerificationStatus")]
        pub verification_status:
            Option<crate::component_integrity::v1_2_3::CommonAuthInfoVerificationStatus>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CommonAuthInfoVerificationStatus {
        V010203(crate::component_integrity::v1_2_3::VerificationStatus),
        V000001(crate::component_integrity::v1_2_3::CommonAuthInfoVerificationStatusN1),
    }
    impl Default for CommonAuthInfoVerificationStatus {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CommonAuthInfoVerificationStatusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CommunicationInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sessions")]
        pub sessions: Option<Vec<crate::component_integrity::v1_2_3::CommunicationInfoSessions>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CommunicationInfoSessions {
        V010203(crate::component_integrity::v1_2_3::SingleSessionInfo),
        V000001(crate::component_integrity::v1_2_3::CommunicationInfoSessionsN1),
    }
    impl Default for CommunicationInfoSessions {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CommunicationInfoSessionsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComponentIntegrity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::component_integrity::v1_2_3::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentIntegrityEnabled"
        )]
        pub component_integrity_enabled: Option<bool>,
        #[serde(rename = "ComponentIntegrityType")]
        pub component_integrity_type: crate::component_integrity::v1_2_3::ComponentIntegrityType,
        #[serde(rename = "ComponentIntegrityTypeVersion")]
        pub component_integrity_type_version: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::component_integrity::v1_2_3::ComponentIntegrityDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastUpdated")]
        pub last_updated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::component_integrity::v1_2_3::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "SPDM")]
        pub spdm: Option<crate::component_integrity::v1_2_3::SPDMinfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TargetComponentURI")]
        pub target_component_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TPM")]
        pub tpm: Option<crate::component_integrity::v1_2_3::TPMinfo>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ComponentIntegrityDescription {
        V000001(crate::component_integrity::v1_2_3::ComponentIntegrityDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ComponentIntegrityDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ComponentIntegrityDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ComponentIntegrityType {
        #[default]
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SPDM")]
        SPDM,
        #[serde(rename = "TPM")]
        TPM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DMTFmeasurementTypes {
        #[default]
        #[serde(rename = "FirmwareConfiguration")]
        FirmwareConfiguration,
        #[serde(rename = "HardwareConfiguration")]
        HardwareConfiguration,
        #[serde(rename = "ImmutableROM")]
        ImmutableROM,
        #[serde(rename = "MeasurementManifest")]
        MeasurementManifest,
        #[serde(rename = "MutableFirmware")]
        MutableFirmware,
        #[serde(rename = "MutableFirmwareSecurityVersionNumber")]
        MutableFirmwareSecurityVersionNumber,
        #[serde(rename = "MutableFirmwareVersion")]
        MutableFirmwareVersion,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentsProtected"
        )]
        pub components_protected: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentsProtected@odata.count"
        )]
        pub components_protected_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MeasurementSpecification {
        #[default]
        #[serde(rename = "DMTF")]
        DMTF,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMGetSignedMeasurements {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMGetSignedMeasurementsRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementIndices")]
        pub measurement_indices: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Nonce")]
        pub nonce: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SlotId")]
        pub slot_id: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMGetSignedMeasurementsResponse {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificate")]
        pub certificate: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "HashingAlgorithm")]
        pub hashing_algorithm: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PublicKey")]
        pub public_key: Option<String>,
        #[serde(rename = "SignedMeasurements")]
        pub signed_measurements: String,
        #[serde(rename = "SigningAlgorithm")]
        pub signing_algorithm: String,
        #[serde(rename = "Version")]
        pub version: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMcommunication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sessions")]
        pub sessions: Option<Vec<crate::component_integrity::v1_2_3::SPDMcommunicationSessions>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMcommunicationSessions {
        V010203(crate::component_integrity::v1_2_3::SingleSessionInfo),
        V000001(crate::component_integrity::v1_2_3::SPDMcommunicationSessionsN1),
    }
    impl Default for SPDMcommunicationSessions {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SPDMcommunicationSessionsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMidentity {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequesterAuthentication"
        )]
        pub requester_authentication:
            Option<crate::component_integrity::v1_2_3::SPDMidentityRequesterAuthentication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResponderAuthentication"
        )]
        pub responder_authentication:
            Option<crate::component_integrity::v1_2_3::SPDMidentityResponderAuthentication>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMidentityRequesterAuthentication {
        V010203(crate::component_integrity::v1_2_3::SPDMrequesterAuth),
        V000001(crate::component_integrity::v1_2_3::SPDMidentityRequesterAuthenticationN1),
    }
    impl Default for SPDMidentityRequesterAuthentication {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SPDMidentityRequesterAuthenticationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMidentityResponderAuthentication {
        V010203(crate::component_integrity::v1_2_3::SPDMresponderAuth),
        V000001(crate::component_integrity::v1_2_3::SPDMidentityResponderAuthenticationN1),
    }
    impl Default for SPDMidentityResponderAuthentication {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SPDMidentityResponderAuthenticationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMinfo {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentCommunication"
        )]
        pub component_communication:
            Option<crate::component_integrity::v1_2_3::SPDMinfoComponentCommunication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IdentityAuthentication"
        )]
        pub identity_authentication:
            Option<crate::component_integrity::v1_2_3::SPDMinfoIdentityAuthentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementSet")]
        pub measurement_set: Option<crate::component_integrity::v1_2_3::SPDMinfoMeasurementSet>,
        #[serde(rename = "Requester")]
        pub requester: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMinfoComponentCommunication {
        V010203(crate::component_integrity::v1_2_3::SPDMcommunication),
        V000001(crate::component_integrity::v1_2_3::SPDMinfoComponentCommunicationN1),
    }
    impl Default for SPDMinfoComponentCommunication {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SPDMinfoComponentCommunicationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMinfoIdentityAuthentication {
        V010203(crate::component_integrity::v1_2_3::SPDMidentity),
        V000001(crate::component_integrity::v1_2_3::SPDMinfoIdentityAuthenticationN1),
    }
    impl Default for SPDMinfoIdentityAuthentication {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SPDMinfoIdentityAuthenticationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMinfoMeasurementSet {
        V010203(crate::component_integrity::v1_2_3::SPDMmeasurementSet),
        V000001(crate::component_integrity::v1_2_3::SPDMinfoMeasurementSetN1),
    }
    impl Default for SPDMinfoMeasurementSet {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SPDMinfoMeasurementSetN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMmeasurementSet {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MeasurementSpecification"
        )]
        pub measurement_specification:
            Option<crate::component_integrity::v1_2_3::SPDMmeasurementSetMeasurementSpecification>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementSummary")]
        pub measurement_summary: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MeasurementSummaryHashAlgorithm"
        )]
        pub measurement_summary_hash_algorithm: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MeasurementSummaryType"
        )]
        pub measurement_summary_type:
            Option<crate::component_integrity::v1_2_3::SPDMmeasurementSetMeasurementSummaryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements:
            Option<Vec<crate::component_integrity::v1_2_3::SPDMmeasurementSetMeasurements>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMmeasurementSetMeasurementSpecification {
        V010203(crate::component_integrity::v1_2_3::MeasurementSpecification),
        V000001(crate::component_integrity::v1_2_3::SPDMmeasurementSetMeasurementSpecificationN1),
    }
    impl Default for SPDMmeasurementSetMeasurementSpecification {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SPDMmeasurementSetMeasurementSpecificationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMmeasurementSetMeasurementSummaryType {
        V010203(crate::component_integrity::v1_2_3::SPDMmeasurementSummaryType),
        V000001(crate::component_integrity::v1_2_3::SPDMmeasurementSetMeasurementSummaryTypeN1),
    }
    impl Default for SPDMmeasurementSetMeasurementSummaryType {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SPDMmeasurementSetMeasurementSummaryTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMmeasurementSetMeasurements {
        V010203(crate::component_integrity::v1_2_3::SPDMsingleMeasurement),
        V000001(crate::component_integrity::v1_2_3::SPDMmeasurementSetMeasurementsN1),
    }
    impl Default for SPDMmeasurementSetMeasurements {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SPDMmeasurementSetMeasurementsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SPDMmeasurementSummaryType {
        #[default]
        #[serde(rename = "All")]
        All,
        #[serde(rename = "TCB")]
        TCB,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMrequesterAuth {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ProvidedCertificate"
        )]
        pub provided_certificate: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMresponderAuth {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentCertificate"
        )]
        pub component_certificate: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VerificationStatus")]
        pub verification_status:
            Option<crate::component_integrity::v1_2_3::SPDMresponderAuthVerificationStatus>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMresponderAuthVerificationStatus {
        V010203(crate::component_integrity::v1_2_3::VerificationStatus),
        V000001(crate::component_integrity::v1_2_3::SPDMresponderAuthVerificationStatusN1),
    }
    impl Default for SPDMresponderAuthVerificationStatus {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SPDMresponderAuthVerificationStatusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMsingleMeasurement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastUpdated")]
        pub last_updated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurement")]
        pub measurement: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MeasurementHashAlgorithm"
        )]
        pub measurement_hash_algorithm: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementIndex")]
        pub measurement_index: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementType")]
        pub measurement_type:
            Option<crate::component_integrity::v1_2_3::SPDMsingleMeasurementMeasurementType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartofSummaryHash")]
        pub partof_summary_hash: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecurityVersionNumber"
        )]
        pub security_version_number: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMsingleMeasurementMeasurementType {
        V010203(crate::component_integrity::v1_2_3::DMTFmeasurementTypes),
        V000001(crate::component_integrity::v1_2_3::SPDMsingleMeasurementMeasurementTypeN1),
    }
    impl Default for SPDMsingleMeasurementMeasurementType {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SPDMsingleMeasurementMeasurementTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SecureSessionType {
        #[default]
        #[serde(rename = "AuthenticatedOnly")]
        AuthenticatedOnly,
        #[serde(rename = "EncryptedAuthenticated")]
        EncryptedAuthenticated,
        #[serde(rename = "Plain")]
        Plain,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SingleSessionInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionId")]
        pub session_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SessionType")]
        pub session_type: Option<crate::component_integrity::v1_2_3::SingleSessionInfoSessionType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SingleSessionInfoSessionType {
        V010203(crate::component_integrity::v1_2_3::SecureSessionType),
        V000001(crate::component_integrity::v1_2_3::SingleSessionInfoSessionTypeN1),
    }
    impl Default for SingleSessionInfoSessionType {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SingleSessionInfoSessionTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMGetSignedMeasurements {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMGetSignedMeasurementsRequestBody {
        #[serde(rename = "Certificate")]
        pub certificate: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Nonce")]
        pub nonce: Option<String>,
        #[serde(rename = "PCRSelection")]
        pub pcr_selection: String,
        #[serde(rename = "Scheme")]
        pub scheme: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMGetSignedMeasurementsResponse {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(rename = "SignedMeasurements")]
        pub signed_measurements: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMauth {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentCertificate"
        )]
        pub component_certificate: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VerificationStatus")]
        pub verification_status:
            Option<crate::component_integrity::v1_2_3::TPMauthVerificationStatus>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TPMauthVerificationStatus {
        V010203(crate::component_integrity::v1_2_3::VerificationStatus),
        V000001(crate::component_integrity::v1_2_3::TPMauthVerificationStatusN1),
    }
    impl Default for TPMauthVerificationStatus {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TPMauthVerificationStatusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMcommunication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sessions")]
        pub sessions: Option<Vec<crate::component_integrity::v1_2_3::TPMcommunicationSessions>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TPMcommunicationSessions {
        V010203(crate::component_integrity::v1_2_3::SingleSessionInfo),
        V000001(crate::component_integrity::v1_2_3::TPMcommunicationSessionsN1),
    }
    impl Default for TPMcommunicationSessions {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TPMcommunicationSessionsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMinfo {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentCommunication"
        )]
        pub component_communication:
            Option<crate::component_integrity::v1_2_3::TPMinfoComponentCommunication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IdentityAuthentication"
        )]
        pub identity_authentication:
            Option<crate::component_integrity::v1_2_3::TPMinfoIdentityAuthentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementSet")]
        pub measurement_set: Option<crate::component_integrity::v1_2_3::TPMinfoMeasurementSet>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NonceSizeBytesMaximum"
        )]
        pub nonce_size_bytes_maximum: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TPMinfoComponentCommunication {
        V010203(crate::component_integrity::v1_2_3::TPMcommunication),
        V000001(crate::component_integrity::v1_2_3::TPMinfoComponentCommunicationN1),
    }
    impl Default for TPMinfoComponentCommunication {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TPMinfoComponentCommunicationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TPMinfoIdentityAuthentication {
        V010203(crate::component_integrity::v1_2_3::TPMauth),
        V000001(crate::component_integrity::v1_2_3::TPMinfoIdentityAuthenticationN1),
    }
    impl Default for TPMinfoIdentityAuthentication {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TPMinfoIdentityAuthenticationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TPMinfoMeasurementSet {
        V010203(crate::component_integrity::v1_2_3::TPMmeasurementSet),
        V000001(crate::component_integrity::v1_2_3::TPMinfoMeasurementSetN1),
    }
    impl Default for TPMinfoMeasurementSet {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TPMinfoMeasurementSetN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMmeasurementSet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements:
            Option<Vec<crate::component_integrity::v1_2_3::TPMmeasurementSetMeasurements>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TPMmeasurementSetMeasurements {
        V010203(crate::component_integrity::v1_2_3::TPMsingleMeasurement),
        V000001(crate::component_integrity::v1_2_3::TPMmeasurementSetMeasurementsN1),
    }
    impl Default for TPMmeasurementSetMeasurements {
        fn default() -> Self {
            Self::V010203(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TPMmeasurementSetMeasurementsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMsingleMeasurement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastUpdated")]
        pub last_updated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurement")]
        pub measurement: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MeasurementHashAlgorithm"
        )]
        pub measurement_hash_algorithm: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCR")]
        pub pcr: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VerificationStatus {
        #[default]
        #[serde(rename = "Failed")]
        Failed,
        #[serde(rename = "Success")]
        Success,
    }
}
