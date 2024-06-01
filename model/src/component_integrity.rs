pub type ComponentIntegrity = crate::component_integrity::v1_2_1::ComponentIntegrity;
pub mod v1_2_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComponentIntegrity.SPDMGetSignedMeasurements"
        )]
        pub component_integrity_spdm_get_signed_measurements:
            Option<crate::component_integrity::v1_2_1::SPDMGetSignedMeasurements>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComponentIntegrity.TPMGetSignedMeasurements"
        )]
        pub component_integrity_tpm_get_signed_measurements:
            Option<crate::component_integrity::v1_2_1::TPMGetSignedMeasurements>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::component_integrity::v1_2_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CommonAuthInfo {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentCertificate"
        )]
        pub component_certificate: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VerificationStatus")]
        pub verification_status: Option<crate::component_integrity::v1_2_1::VerificationStatus>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CommunicationInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sessions")]
        pub sessions: Option<Vec<crate::component_integrity::v1_2_1::SingleSessionInfo>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ComponentIntegrity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::component_integrity::v1_2_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentIntegrityEnabled"
        )]
        pub component_integrity_enabled: Option<bool>,
        #[serde(rename = "ComponentIntegrityType")]
        pub component_integrity_type: crate::component_integrity::v1_2_1::ComponentIntegrityType,
        #[serde(rename = "ComponentIntegrityTypeVersion")]
        pub component_integrity_type_version: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastUpdated")]
        pub last_updated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::component_integrity::v1_2_1::Links>,
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
        pub spdm: Option<crate::component_integrity::v1_2_1::SPDMinfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TargetComponentURI")]
        pub target_component_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TPM")]
        pub tpm: Option<crate::component_integrity::v1_2_1::TPMinfo>,
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
        pub sessions: Option<Vec<crate::component_integrity::v1_2_1::SingleSessionInfo>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMidentity {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RequesterAuthentication"
        )]
        pub requester_authentication: Option<crate::component_integrity::v1_2_1::SPDMrequesterAuth>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResponderAuthentication"
        )]
        pub responder_authentication: Option<crate::component_integrity::v1_2_1::SPDMresponderAuth>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SPDMinfo {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentCommunication"
        )]
        pub component_communication: Option<crate::component_integrity::v1_2_1::SPDMcommunication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IdentityAuthentication"
        )]
        pub identity_authentication: Option<crate::component_integrity::v1_2_1::SPDMidentity>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementSet")]
        pub measurement_set: Option<crate::component_integrity::v1_2_1::SPDMmeasurementSet>,
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
            Option<crate::component_integrity::v1_2_1::MeasurementSpecification>,
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
            Option<crate::component_integrity::v1_2_1::SPDMmeasurementSummaryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::component_integrity::v1_2_1::SPDMsingleMeasurement>>,
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
        pub verification_status: Option<crate::component_integrity::v1_2_1::VerificationStatus>,
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
        pub measurement_type: Option<crate::component_integrity::v1_2_1::DMTFmeasurementTypes>,
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
        pub session_type: Option<crate::component_integrity::v1_2_1::SecureSessionType>,
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
        pub verification_status: Option<crate::component_integrity::v1_2_1::VerificationStatus>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMcommunication {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sessions")]
        pub sessions: Option<Vec<crate::component_integrity::v1_2_1::SingleSessionInfo>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMinfo {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentCommunication"
        )]
        pub component_communication: Option<crate::component_integrity::v1_2_1::TPMcommunication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IdentityAuthentication"
        )]
        pub identity_authentication: Option<crate::component_integrity::v1_2_1::TPMauth>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementSet")]
        pub measurement_set: Option<crate::component_integrity::v1_2_1::TPMmeasurementSet>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NonceSizeBytesMaximum"
        )]
        pub nonce_size_bytes_maximum: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TPMmeasurementSet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::component_integrity::v1_2_1::TPMsingleMeasurement>>,
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
