pub type ComponentIntegrity = crate::component_integrity::v1_3_2::ComponentIntegrity;
pub mod v1_3_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComponentIntegrity.SPDMGetSignedMeasurements"
        )]
        pub component_integrity_spdm_get_signed_measurements:
            Option<crate::component_integrity::v1_3_1::SPDMGetSignedMeasurements>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComponentIntegrity.TPMGetSignedMeasurements"
        )]
        pub component_integrity_tpm_get_signed_measurements:
            Option<crate::component_integrity::v1_3_1::TPMGetSignedMeasurements>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::component_integrity::v1_3_1::OemActions>,
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
            Option<crate::component_integrity::v1_3_1::CommonAuthInfoVerificationStatus>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CommonAuthInfoVerificationStatus {
        V010301(crate::component_integrity::v1_3_1::VerificationStatus),
        V000001(crate::component_integrity::v1_3_1::CommonAuthInfoVerificationStatusN1),
    }
    impl Default for CommonAuthInfoVerificationStatus {
        fn default() -> Self {
            Self::V010301(Default::default())
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
        pub sessions: Option<Vec<crate::component_integrity::v1_3_1::CommunicationInfoSessions>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CommunicationInfoSessions {
        V010301(crate::component_integrity::v1_3_1::SingleSessionInfo),
        V000001(crate::component_integrity::v1_3_1::CommunicationInfoSessionsN1),
    }
    impl Default for CommunicationInfoSessions {
        fn default() -> Self {
            Self::V010301(Default::default())
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
        pub actions: Option<crate::component_integrity::v1_3_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentIntegrityEnabled"
        )]
        pub component_integrity_enabled: Option<bool>,
        #[serde(rename = "ComponentIntegrityType")]
        pub component_integrity_type: crate::component_integrity::v1_3_1::ComponentIntegrityType,
        #[serde(rename = "ComponentIntegrityTypeVersion")]
        pub component_integrity_type_version: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::component_integrity::v1_3_1::ComponentIntegrityDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastUpdated")]
        pub last_updated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::component_integrity::v1_3_1::Links>,
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
        pub spdm: Option<crate::component_integrity::v1_3_1::SPDMinfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TargetComponentURI")]
        pub target_component_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TPM")]
        pub tpm: Option<crate::component_integrity::v1_3_1::TPMinfo>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ComponentIntegrityDescription {
        V000001(crate::component_integrity::v1_3_1::ComponentIntegrityDescriptionN1),
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
        #[serde(rename = "TCM")]
        TCM,
        #[serde(rename = "TPCM")]
        TPCM,
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
        pub sessions: Option<Vec<crate::component_integrity::v1_3_1::SPDMcommunicationSessions>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMcommunicationSessions {
        V010301(crate::component_integrity::v1_3_1::SingleSessionInfo),
        V000001(crate::component_integrity::v1_3_1::SPDMcommunicationSessionsN1),
    }
    impl Default for SPDMcommunicationSessions {
        fn default() -> Self {
            Self::V010301(Default::default())
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
            Option<crate::component_integrity::v1_3_1::SPDMidentityRequesterAuthentication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResponderAuthentication"
        )]
        pub responder_authentication:
            Option<crate::component_integrity::v1_3_1::SPDMidentityResponderAuthentication>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMidentityRequesterAuthentication {
        V010301(crate::component_integrity::v1_3_1::SPDMrequesterAuth),
        V000001(crate::component_integrity::v1_3_1::SPDMidentityRequesterAuthenticationN1),
    }
    impl Default for SPDMidentityRequesterAuthentication {
        fn default() -> Self {
            Self::V010301(Default::default())
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
        V010301(crate::component_integrity::v1_3_1::SPDMresponderAuth),
        V000001(crate::component_integrity::v1_3_1::SPDMidentityResponderAuthenticationN1),
    }
    impl Default for SPDMidentityResponderAuthentication {
        fn default() -> Self {
            Self::V010301(Default::default())
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
            Option<crate::component_integrity::v1_3_1::SPDMinfoComponentCommunication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IdentityAuthentication"
        )]
        pub identity_authentication:
            Option<crate::component_integrity::v1_3_1::SPDMinfoIdentityAuthentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementSet")]
        pub measurement_set: Option<crate::component_integrity::v1_3_1::SPDMinfoMeasurementSet>,
        #[serde(rename = "Requester")]
        pub requester: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMinfoComponentCommunication {
        V010301(crate::component_integrity::v1_3_1::SPDMcommunication),
        V000001(crate::component_integrity::v1_3_1::SPDMinfoComponentCommunicationN1),
    }
    impl Default for SPDMinfoComponentCommunication {
        fn default() -> Self {
            Self::V010301(Default::default())
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
        V010301(crate::component_integrity::v1_3_1::SPDMidentity),
        V000001(crate::component_integrity::v1_3_1::SPDMinfoIdentityAuthenticationN1),
    }
    impl Default for SPDMinfoIdentityAuthentication {
        fn default() -> Self {
            Self::V010301(Default::default())
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
        V010301(crate::component_integrity::v1_3_1::SPDMmeasurementSet),
        V000001(crate::component_integrity::v1_3_1::SPDMinfoMeasurementSetN1),
    }
    impl Default for SPDMinfoMeasurementSet {
        fn default() -> Self {
            Self::V010301(Default::default())
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
            Option<crate::component_integrity::v1_3_1::SPDMmeasurementSetMeasurementSpecification>,
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
            Option<crate::component_integrity::v1_3_1::SPDMmeasurementSetMeasurementSummaryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements:
            Option<Vec<crate::component_integrity::v1_3_1::SPDMmeasurementSetMeasurements>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMmeasurementSetMeasurementSpecification {
        V010301(crate::component_integrity::v1_3_1::MeasurementSpecification),
        V000001(crate::component_integrity::v1_3_1::SPDMmeasurementSetMeasurementSpecificationN1),
    }
    impl Default for SPDMmeasurementSetMeasurementSpecification {
        fn default() -> Self {
            Self::V010301(Default::default())
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
        V010301(crate::component_integrity::v1_3_1::SPDMmeasurementSummaryType),
        V000001(crate::component_integrity::v1_3_1::SPDMmeasurementSetMeasurementSummaryTypeN1),
    }
    impl Default for SPDMmeasurementSetMeasurementSummaryType {
        fn default() -> Self {
            Self::V010301(Default::default())
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
        V010301(crate::component_integrity::v1_3_1::SPDMsingleMeasurement),
        V000001(crate::component_integrity::v1_3_1::SPDMmeasurementSetMeasurementsN1),
    }
    impl Default for SPDMmeasurementSetMeasurements {
        fn default() -> Self {
            Self::V010301(Default::default())
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
            Option<crate::component_integrity::v1_3_1::SPDMresponderAuthVerificationStatus>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMresponderAuthVerificationStatus {
        V010301(crate::component_integrity::v1_3_1::VerificationStatus),
        V000001(crate::component_integrity::v1_3_1::SPDMresponderAuthVerificationStatusN1),
    }
    impl Default for SPDMresponderAuthVerificationStatus {
        fn default() -> Self {
            Self::V010301(Default::default())
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
            Option<crate::component_integrity::v1_3_1::SPDMsingleMeasurementMeasurementType>,
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
        V010301(crate::component_integrity::v1_3_1::DMTFmeasurementTypes),
        V000001(crate::component_integrity::v1_3_1::SPDMsingleMeasurementMeasurementTypeN1),
    }
    impl Default for SPDMsingleMeasurementMeasurementType {
        fn default() -> Self {
            Self::V010301(Default::default())
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
        pub session_type: Option<crate::component_integrity::v1_3_1::SingleSessionInfoSessionType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SingleSessionInfoSessionType {
        V010301(crate::component_integrity::v1_3_1::SecureSessionType),
        V000001(crate::component_integrity::v1_3_1::SingleSessionInfoSessionTypeN1),
    }
    impl Default for SingleSessionInfoSessionType {
        fn default() -> Self {
            Self::V010301(Default::default())
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
            Option<crate::component_integrity::v1_3_1::TPMauthVerificationStatus>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TPMauthVerificationStatus {
        V010301(crate::component_integrity::v1_3_1::VerificationStatus),
        V000001(crate::component_integrity::v1_3_1::TPMauthVerificationStatusN1),
    }
    impl Default for TPMauthVerificationStatus {
        fn default() -> Self {
            Self::V010301(Default::default())
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
        pub sessions: Option<Vec<crate::component_integrity::v1_3_1::TPMcommunicationSessions>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TPMcommunicationSessions {
        V010301(crate::component_integrity::v1_3_1::SingleSessionInfo),
        V000001(crate::component_integrity::v1_3_1::TPMcommunicationSessionsN1),
    }
    impl Default for TPMcommunicationSessions {
        fn default() -> Self {
            Self::V010301(Default::default())
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
            Option<crate::component_integrity::v1_3_1::TPMinfoComponentCommunication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IdentityAuthentication"
        )]
        pub identity_authentication:
            Option<crate::component_integrity::v1_3_1::TPMinfoIdentityAuthentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementSet")]
        pub measurement_set: Option<crate::component_integrity::v1_3_1::TPMinfoMeasurementSet>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NonceSizeBytesMaximum"
        )]
        pub nonce_size_bytes_maximum: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TPMinfoComponentCommunication {
        V010301(crate::component_integrity::v1_3_1::TPMcommunication),
        V000001(crate::component_integrity::v1_3_1::TPMinfoComponentCommunicationN1),
    }
    impl Default for TPMinfoComponentCommunication {
        fn default() -> Self {
            Self::V010301(Default::default())
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
        V010301(crate::component_integrity::v1_3_1::TPMauth),
        V000001(crate::component_integrity::v1_3_1::TPMinfoIdentityAuthenticationN1),
    }
    impl Default for TPMinfoIdentityAuthentication {
        fn default() -> Self {
            Self::V010301(Default::default())
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
        V010301(crate::component_integrity::v1_3_1::TPMmeasurementSet),
        V000001(crate::component_integrity::v1_3_1::TPMinfoMeasurementSetN1),
    }
    impl Default for TPMinfoMeasurementSet {
        fn default() -> Self {
            Self::V010301(Default::default())
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
            Option<Vec<crate::component_integrity::v1_3_1::TPMmeasurementSetMeasurements>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TPMmeasurementSetMeasurements {
        V010301(crate::component_integrity::v1_3_1::TPMsingleMeasurement),
        V000001(crate::component_integrity::v1_3_1::TPMmeasurementSetMeasurementsN1),
    }
    impl Default for TPMmeasurementSetMeasurements {
        fn default() -> Self {
            Self::V010301(Default::default())
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
pub mod v1_3_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComponentIntegrity.SPDMGetSignedMeasurements"
        )]
        pub component_integrity_spdm_get_signed_measurements:
            Option<crate::component_integrity::v1_3_2::SPDMGetSignedMeasurements>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#ComponentIntegrity.TPMGetSignedMeasurements"
        )]
        pub component_integrity_tpm_get_signed_measurements:
            Option<crate::component_integrity::v1_3_2::TPMGetSignedMeasurements>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::component_integrity::v1_3_2::OemActions>,
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
            Option<crate::component_integrity::v1_3_2::CommonAuthInfoVerificationStatus>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CommonAuthInfoVerificationStatus {
        V010302(crate::component_integrity::v1_3_2::VerificationStatus),
        V000001(crate::component_integrity::v1_3_2::CommonAuthInfoVerificationStatusN1),
    }
    impl Default for CommonAuthInfoVerificationStatus {
        fn default() -> Self {
            Self::V010302(Default::default())
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
        pub sessions: Option<Vec<crate::component_integrity::v1_3_2::CommunicationInfoSessions>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CommunicationInfoSessions {
        V010302(crate::component_integrity::v1_3_2::SingleSessionInfo),
        V000001(crate::component_integrity::v1_3_2::CommunicationInfoSessionsN1),
    }
    impl Default for CommunicationInfoSessions {
        fn default() -> Self {
            Self::V010302(Default::default())
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
        pub actions: Option<crate::component_integrity::v1_3_2::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ComponentIntegrityEnabled"
        )]
        pub component_integrity_enabled: Option<bool>,
        #[serde(rename = "ComponentIntegrityType")]
        pub component_integrity_type: crate::component_integrity::v1_3_2::ComponentIntegrityType,
        #[serde(rename = "ComponentIntegrityTypeVersion")]
        pub component_integrity_type_version: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::component_integrity::v1_3_2::ComponentIntegrityDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LastUpdated")]
        pub last_updated: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::component_integrity::v1_3_2::Links>,
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
        pub spdm: Option<crate::component_integrity::v1_3_2::SPDMinfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TargetComponentURI")]
        pub target_component_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TPM")]
        pub tpm: Option<crate::component_integrity::v1_3_2::TPMinfo>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ComponentIntegrityDescription {
        V000001(crate::component_integrity::v1_3_2::ComponentIntegrityDescriptionN1),
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
        #[serde(rename = "TCM")]
        TCM,
        #[serde(rename = "TPCM")]
        TPCM,
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
        pub sessions: Option<Vec<crate::component_integrity::v1_3_2::SPDMcommunicationSessions>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMcommunicationSessions {
        V010302(crate::component_integrity::v1_3_2::SingleSessionInfo),
        V000001(crate::component_integrity::v1_3_2::SPDMcommunicationSessionsN1),
    }
    impl Default for SPDMcommunicationSessions {
        fn default() -> Self {
            Self::V010302(Default::default())
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
            Option<crate::component_integrity::v1_3_2::SPDMidentityRequesterAuthentication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResponderAuthentication"
        )]
        pub responder_authentication:
            Option<crate::component_integrity::v1_3_2::SPDMidentityResponderAuthentication>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMidentityRequesterAuthentication {
        V010302(crate::component_integrity::v1_3_2::SPDMrequesterAuth),
        V000001(crate::component_integrity::v1_3_2::SPDMidentityRequesterAuthenticationN1),
    }
    impl Default for SPDMidentityRequesterAuthentication {
        fn default() -> Self {
            Self::V010302(Default::default())
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
        V010302(crate::component_integrity::v1_3_2::SPDMresponderAuth),
        V000001(crate::component_integrity::v1_3_2::SPDMidentityResponderAuthenticationN1),
    }
    impl Default for SPDMidentityResponderAuthentication {
        fn default() -> Self {
            Self::V010302(Default::default())
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
            Option<crate::component_integrity::v1_3_2::SPDMinfoComponentCommunication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IdentityAuthentication"
        )]
        pub identity_authentication:
            Option<crate::component_integrity::v1_3_2::SPDMinfoIdentityAuthentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementSet")]
        pub measurement_set: Option<crate::component_integrity::v1_3_2::SPDMinfoMeasurementSet>,
        #[serde(rename = "Requester")]
        pub requester: crate::odata_v4::IdRef,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMinfoComponentCommunication {
        V010302(crate::component_integrity::v1_3_2::SPDMcommunication),
        V000001(crate::component_integrity::v1_3_2::SPDMinfoComponentCommunicationN1),
    }
    impl Default for SPDMinfoComponentCommunication {
        fn default() -> Self {
            Self::V010302(Default::default())
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
        V010302(crate::component_integrity::v1_3_2::SPDMidentity),
        V000001(crate::component_integrity::v1_3_2::SPDMinfoIdentityAuthenticationN1),
    }
    impl Default for SPDMinfoIdentityAuthentication {
        fn default() -> Self {
            Self::V010302(Default::default())
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
        V010302(crate::component_integrity::v1_3_2::SPDMmeasurementSet),
        V000001(crate::component_integrity::v1_3_2::SPDMinfoMeasurementSetN1),
    }
    impl Default for SPDMinfoMeasurementSet {
        fn default() -> Self {
            Self::V010302(Default::default())
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
            Option<crate::component_integrity::v1_3_2::SPDMmeasurementSetMeasurementSpecification>,
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
            Option<crate::component_integrity::v1_3_2::SPDMmeasurementSetMeasurementSummaryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements:
            Option<Vec<crate::component_integrity::v1_3_2::SPDMmeasurementSetMeasurements>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMmeasurementSetMeasurementSpecification {
        V010302(crate::component_integrity::v1_3_2::MeasurementSpecification),
        V000001(crate::component_integrity::v1_3_2::SPDMmeasurementSetMeasurementSpecificationN1),
    }
    impl Default for SPDMmeasurementSetMeasurementSpecification {
        fn default() -> Self {
            Self::V010302(Default::default())
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
        V010302(crate::component_integrity::v1_3_2::SPDMmeasurementSummaryType),
        V000001(crate::component_integrity::v1_3_2::SPDMmeasurementSetMeasurementSummaryTypeN1),
    }
    impl Default for SPDMmeasurementSetMeasurementSummaryType {
        fn default() -> Self {
            Self::V010302(Default::default())
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
        V010302(crate::component_integrity::v1_3_2::SPDMsingleMeasurement),
        V000001(crate::component_integrity::v1_3_2::SPDMmeasurementSetMeasurementsN1),
    }
    impl Default for SPDMmeasurementSetMeasurements {
        fn default() -> Self {
            Self::V010302(Default::default())
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
            Option<crate::component_integrity::v1_3_2::SPDMresponderAuthVerificationStatus>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SPDMresponderAuthVerificationStatus {
        V010302(crate::component_integrity::v1_3_2::VerificationStatus),
        V000001(crate::component_integrity::v1_3_2::SPDMresponderAuthVerificationStatusN1),
    }
    impl Default for SPDMresponderAuthVerificationStatus {
        fn default() -> Self {
            Self::V010302(Default::default())
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
            Option<crate::component_integrity::v1_3_2::SPDMsingleMeasurementMeasurementType>,
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
        V010302(crate::component_integrity::v1_3_2::DMTFmeasurementTypes),
        V000001(crate::component_integrity::v1_3_2::SPDMsingleMeasurementMeasurementTypeN1),
    }
    impl Default for SPDMsingleMeasurementMeasurementType {
        fn default() -> Self {
            Self::V010302(Default::default())
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
        pub session_type: Option<crate::component_integrity::v1_3_2::SingleSessionInfoSessionType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SingleSessionInfoSessionType {
        V010302(crate::component_integrity::v1_3_2::SecureSessionType),
        V000001(crate::component_integrity::v1_3_2::SingleSessionInfoSessionTypeN1),
    }
    impl Default for SingleSessionInfoSessionType {
        fn default() -> Self {
            Self::V010302(Default::default())
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
            Option<crate::component_integrity::v1_3_2::TPMauthVerificationStatus>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TPMauthVerificationStatus {
        V010302(crate::component_integrity::v1_3_2::VerificationStatus),
        V000001(crate::component_integrity::v1_3_2::TPMauthVerificationStatusN1),
    }
    impl Default for TPMauthVerificationStatus {
        fn default() -> Self {
            Self::V010302(Default::default())
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
        pub sessions: Option<Vec<crate::component_integrity::v1_3_2::TPMcommunicationSessions>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TPMcommunicationSessions {
        V010302(crate::component_integrity::v1_3_2::SingleSessionInfo),
        V000001(crate::component_integrity::v1_3_2::TPMcommunicationSessionsN1),
    }
    impl Default for TPMcommunicationSessions {
        fn default() -> Self {
            Self::V010302(Default::default())
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
            Option<crate::component_integrity::v1_3_2::TPMinfoComponentCommunication>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IdentityAuthentication"
        )]
        pub identity_authentication:
            Option<crate::component_integrity::v1_3_2::TPMinfoIdentityAuthentication>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MeasurementSet")]
        pub measurement_set: Option<crate::component_integrity::v1_3_2::TPMinfoMeasurementSet>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NonceSizeBytesMaximum"
        )]
        pub nonce_size_bytes_maximum: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TPMinfoComponentCommunication {
        V010302(crate::component_integrity::v1_3_2::TPMcommunication),
        V000001(crate::component_integrity::v1_3_2::TPMinfoComponentCommunicationN1),
    }
    impl Default for TPMinfoComponentCommunication {
        fn default() -> Self {
            Self::V010302(Default::default())
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
        V010302(crate::component_integrity::v1_3_2::TPMauth),
        V000001(crate::component_integrity::v1_3_2::TPMinfoIdentityAuthenticationN1),
    }
    impl Default for TPMinfoIdentityAuthentication {
        fn default() -> Self {
            Self::V010302(Default::default())
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
        V010302(crate::component_integrity::v1_3_2::TPMmeasurementSet),
        V000001(crate::component_integrity::v1_3_2::TPMinfoMeasurementSetN1),
    }
    impl Default for TPMinfoMeasurementSet {
        fn default() -> Self {
            Self::V010302(Default::default())
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
            Option<Vec<crate::component_integrity::v1_3_2::TPMmeasurementSetMeasurements>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TPMmeasurementSetMeasurements {
        V010302(crate::component_integrity::v1_3_2::TPMsingleMeasurement),
        V000001(crate::component_integrity::v1_3_2::TPMmeasurementSetMeasurementsN1),
    }
    impl Default for TPMmeasurementSetMeasurements {
        fn default() -> Self {
            Self::V010302(Default::default())
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
