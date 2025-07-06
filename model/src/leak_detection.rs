pub type LeakDetection = crate::leak_detection::v1_1_0::LeakDetection;
pub mod v1_1_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::leak_detection::v1_1_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LeakDetection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::leak_detection::v1_1_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::leak_detection::v1_1_0::LeakDetectionDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LeakDetectorGroups")]
        pub leak_detector_groups: Option<Vec<crate::leak_detection::v1_1_0::LeakDetectorGroup>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LeakDetectors")]
        pub leak_detectors: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectionDescription {
        V000001(crate::leak_detection::v1_1_0::LeakDetectionDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for LeakDetectionDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectionDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LeakDetectorGroup {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Detectors")]
        pub detectors: Option<Vec<crate::leak_detector::LeakDetectorArrayExcerpt>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Detectors@odata.count"
        )]
        pub detectors_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GroupName")]
        pub group_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HumidityPercent")]
        pub humidity_percent:
            Option<crate::leak_detection::v1_1_0::LeakDetectorGroupHumidityPercent>,
        #[serde(rename = "Status")]
        pub status: crate::resource::Status,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LeakDetectorGroupHumidityPercent {
        V000001(crate::leak_detection::v1_1_0::LeakDetectorGroupHumidityPercentN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for LeakDetectorGroupHumidityPercent {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LeakDetectorGroupHumidityPercentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
