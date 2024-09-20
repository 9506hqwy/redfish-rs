pub type SwitchMetrics = crate::switch_metrics::v1_0_2::SwitchMetrics;
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::switch_metrics::v1_0_2::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#SwitchMetrics.ClearCurrentPeriod"
        )]
        pub switch_metrics_clear_current_period:
            Option<crate::switch_metrics::v1_0_2::ClearCurrentPeriod>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriodRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CurrentPeriod {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCErrorCount"
        )]
        pub correctable_ecc_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCErrorCount"
        )]
        pub uncorrectable_ecc_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InternalMemoryMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentPeriod")]
        pub current_period: Option<crate::switch_metrics::v1_0_2::CurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifeTime")]
        pub life_time: Option<crate::switch_metrics::v1_0_2::LifeTime>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LifeTime {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCErrorCount"
        )]
        pub correctable_ecc_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCErrorCount"
        )]
        pub uncorrectable_ecc_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SwitchMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::switch_metrics::v1_0_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::switch_metrics::v1_0_2::SwitchMetricsDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InternalMemoryMetrics"
        )]
        pub internal_memory_metrics: Option<crate::switch_metrics::v1_0_2::InternalMemoryMetrics>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeErrors")]
        pub pcie_errors: Option<crate::pcie_device::PCIeErrors>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SwitchMetricsDescription {
        V000001(crate::switch_metrics::v1_0_2::SwitchMetricsDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for SwitchMetricsDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SwitchMetricsDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
