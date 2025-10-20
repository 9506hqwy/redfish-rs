pub type LeakDetector = crate::leak_detector::v1_5_0::LeakDetector;
pub type LeakDetectorArrayExcerpt = crate::leak_detector::v1_5_0::LeakDetectorArrayExcerpt;
pub type LeakDetectorExcerpt = crate::leak_detector::v1_5_0::LeakDetectorExcerpt;
pub mod v1_4_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::leak_detector::v1_4_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LeakDetector {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::leak_detector::v1_4_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CriticalReactionType"
        )]
        pub critical_reaction_type: Option<crate::leak_detector::v1_4_0::ReactionType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::leak_detector::v1_4_0::LeakDetectorDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DetectorState")]
        pub detector_state: Option<crate::leak_detector::v1_4_0::LeakDetectorDetectorState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LeakDetectorType")]
        pub leak_detector_type: Option<crate::leak_detector::v1_4_0::LeakDetectorLeakDetectorType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::leak_detector::v1_4_0::LeakDetectorPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context:
            Option<crate::leak_detector::v1_4_0::LeakDetectorPhysicalSubContext>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReactionDelaySeconds"
        )]
        pub reaction_delay_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensingFrequency")]
        pub sensing_frequency: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "WarningReactionType"
        )]
        pub warning_reaction_type: Option<crate::leak_detector::v1_4_0::ReactionType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LeakDetectorArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DetectorState")]
        pub detector_state:
            Option<crate::leak_detector::v1_4_0::LeakDetectorArrayExcerptDetectorState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceName")]
        pub device_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context:
            Option<crate::leak_detector::v1_4_0::LeakDetectorArrayExcerptPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context:
            Option<crate::leak_detector::v1_4_0::LeakDetectorArrayExcerptPhysicalSubContext>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorArrayExcerptDetectorState {
        V000001(crate::leak_detector::v1_4_0::LeakDetectorArrayExcerptDetectorStateN1),
        ResourceHealth(crate::resource::Health),
    }
    impl Default for LeakDetectorArrayExcerptDetectorState {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorArrayExcerptDetectorStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorArrayExcerptPhysicalContext {
        V000001(crate::leak_detector::v1_4_0::LeakDetectorArrayExcerptPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for LeakDetectorArrayExcerptPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorArrayExcerptPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorArrayExcerptPhysicalSubContext {
        V000001(crate::leak_detector::v1_4_0::LeakDetectorArrayExcerptPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for LeakDetectorArrayExcerptPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorArrayExcerptPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorDescription {
        V000001(crate::leak_detector::v1_4_0::LeakDetectorDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for LeakDetectorDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorDetectorState {
        V000001(crate::leak_detector::v1_4_0::LeakDetectorDetectorStateN1),
        ResourceHealth(crate::resource::Health),
    }
    impl Default for LeakDetectorDetectorState {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorDetectorStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LeakDetectorExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DetectorState")]
        pub detector_state: Option<crate::leak_detector::v1_4_0::LeakDetectorExcerptDetectorState>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorExcerptDetectorState {
        V000001(crate::leak_detector::v1_4_0::LeakDetectorExcerptDetectorStateN1),
        ResourceHealth(crate::resource::Health),
    }
    impl Default for LeakDetectorExcerptDetectorState {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorExcerptDetectorStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorLeakDetectorType {
        V010400(crate::leak_detector::v1_4_0::LeakDetectorType),
        V000001(crate::leak_detector::v1_4_0::LeakDetectorLeakDetectorTypeN1),
    }
    impl Default for LeakDetectorLeakDetectorType {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorLeakDetectorTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorPhysicalContext {
        V000001(crate::leak_detector::v1_4_0::LeakDetectorPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for LeakDetectorPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorPhysicalSubContext {
        V000001(crate::leak_detector::v1_4_0::LeakDetectorPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for LeakDetectorPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorType {
        #[default]
        #[serde(rename = "FloatSwitch")]
        FloatSwitch,
        #[serde(rename = "Moisture")]
        Moisture,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReactionType {
        #[default]
        #[serde(rename = "ForceOff")]
        ForceOff,
        #[serde(rename = "GracefulShutdown")]
        GracefulShutdown,
        #[serde(rename = "None")]
        None,
    }
}
pub mod v1_5_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::leak_detector::v1_5_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DetectorState {
        #[default]
        #[serde(rename = "Absent")]
        Absent,
        #[serde(rename = "Critical")]
        Critical,
        #[serde(rename = "OK")]
        OK,
        #[serde(rename = "Unavailable")]
        Unavailable,
        #[serde(rename = "Warning")]
        Warning,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LeakDetector {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::leak_detector::v1_5_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CriticalReactionType"
        )]
        pub critical_reaction_type: Option<crate::leak_detector::v1_5_0::ReactionType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::leak_detector::v1_5_0::LeakDetectorDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DetectorState")]
        pub detector_state: Option<crate::leak_detector::v1_5_0::LeakDetectorDetectorState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LeakDetectorType")]
        pub leak_detector_type: Option<crate::leak_detector::v1_5_0::LeakDetectorLeakDetectorType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::leak_detector::v1_5_0::LeakDetectorPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context:
            Option<crate::leak_detector::v1_5_0::LeakDetectorPhysicalSubContext>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReactionDelaySeconds"
        )]
        pub reaction_delay_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensingFrequency")]
        pub sensing_frequency: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "WarningReactionType"
        )]
        pub warning_reaction_type: Option<crate::leak_detector::v1_5_0::ReactionType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LeakDetectorArrayExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DetectorState")]
        pub detector_state:
            Option<crate::leak_detector::v1_5_0::LeakDetectorArrayExcerptDetectorState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeviceName")]
        pub device_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context:
            Option<crate::leak_detector::v1_5_0::LeakDetectorArrayExcerptPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalSubContext")]
        pub physical_sub_context:
            Option<crate::leak_detector::v1_5_0::LeakDetectorArrayExcerptPhysicalSubContext>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorArrayExcerptDetectorState {
        V010500(crate::leak_detector::v1_5_0::DetectorState),
        V000001(crate::leak_detector::v1_5_0::LeakDetectorArrayExcerptDetectorStateN1),
    }
    impl Default for LeakDetectorArrayExcerptDetectorState {
        fn default() -> Self {
            Self::V010500(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorArrayExcerptDetectorStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorArrayExcerptPhysicalContext {
        V000001(crate::leak_detector::v1_5_0::LeakDetectorArrayExcerptPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for LeakDetectorArrayExcerptPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorArrayExcerptPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorArrayExcerptPhysicalSubContext {
        V000001(crate::leak_detector::v1_5_0::LeakDetectorArrayExcerptPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for LeakDetectorArrayExcerptPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorArrayExcerptPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorDescription {
        V000001(crate::leak_detector::v1_5_0::LeakDetectorDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for LeakDetectorDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorDetectorState {
        V010500(crate::leak_detector::v1_5_0::DetectorState),
        V000001(crate::leak_detector::v1_5_0::LeakDetectorDetectorStateN1),
    }
    impl Default for LeakDetectorDetectorState {
        fn default() -> Self {
            Self::V010500(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorDetectorStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LeakDetectorExcerpt {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataSourceUri")]
        pub data_source_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DetectorState")]
        pub detector_state: Option<crate::leak_detector::v1_5_0::LeakDetectorExcerptDetectorState>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorExcerptDetectorState {
        V010500(crate::leak_detector::v1_5_0::DetectorState),
        V000001(crate::leak_detector::v1_5_0::LeakDetectorExcerptDetectorStateN1),
    }
    impl Default for LeakDetectorExcerptDetectorState {
        fn default() -> Self {
            Self::V010500(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorExcerptDetectorStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorLeakDetectorType {
        V010500(crate::leak_detector::v1_5_0::LeakDetectorType),
        V000001(crate::leak_detector::v1_5_0::LeakDetectorLeakDetectorTypeN1),
    }
    impl Default for LeakDetectorLeakDetectorType {
        fn default() -> Self {
            Self::V010500(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorLeakDetectorTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorPhysicalContext {
        V000001(crate::leak_detector::v1_5_0::LeakDetectorPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for LeakDetectorPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorPhysicalSubContext {
        V000001(crate::leak_detector::v1_5_0::LeakDetectorPhysicalSubContextN1),
        PhysicalContextPhysicalSubContext(crate::physical_context::PhysicalSubContext),
    }
    impl Default for LeakDetectorPhysicalSubContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorPhysicalSubContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorType {
        #[default]
        #[serde(rename = "FloatSwitch")]
        FloatSwitch,
        #[serde(rename = "Moisture")]
        Moisture,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReactionType {
        #[default]
        #[serde(rename = "ForceOff")]
        ForceOff,
        #[serde(rename = "GracefulShutdown")]
        GracefulShutdown,
        #[serde(rename = "None")]
        None,
    }
}
