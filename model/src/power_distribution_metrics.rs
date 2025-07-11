pub type PowerDistributionMetrics =
    crate::power_distribution_metrics::v1_4_0::PowerDistributionMetrics;
pub mod v1_4_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power_distribution_metrics::v1_4_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#PowerDistributionMetrics.ResetMetrics"
        )]
        pub power_distribution_metrics_reset_metrics:
            Option<crate::power_distribution_metrics::v1_4_0::ResetMetrics>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerDistributionMetrics { # [serde (skip_serializing_if = "Option::is_none" , rename = "AbsoluteHumidity")] pub absolute_humidity : Option < crate :: power_distribution_metrics :: v1_4_0 :: PowerDistributionMetricsAbsoluteHumidity > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Actions")] pub actions : Option < crate :: power_distribution_metrics :: v1_4_0 :: Actions > , # [serde (skip_serializing_if = "Option::is_none" , rename = "AmbientTemperatureCelsius")] pub ambient_temperature_celsius : Option < crate :: power_distribution_metrics :: v1_4_0 :: PowerDistributionMetricsAmbientTemperatureCelsius > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Description")] pub description : Option < crate :: power_distribution_metrics :: v1_4_0 :: PowerDistributionMetricsDescription > , # [serde (skip_serializing_if = "Option::is_none" , rename = "EnergykWh")] pub energyk_wh : Option < crate :: power_distribution_metrics :: v1_4_0 :: PowerDistributionMetricsEnergykWh > , # [serde (skip_serializing_if = "Option::is_none" , rename = "HumidityPercent")] pub humidity_percent : Option < crate :: power_distribution_metrics :: v1_4_0 :: PowerDistributionMetricsHumidityPercent > , # [serde (rename = "Id")] pub id : String , # [serde (rename = "Name")] pub name : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "@odata.context")] pub odata_context : Option < String > , # [serde (skip_serializing_if = "Option::is_none" , rename = "@odata.etag")] pub odata_etag : Option < String > , # [serde (rename = "@odata.id")] pub odata_id : String , # [serde (rename = "@odata.type")] pub odata_type : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "Oem")] pub oem : Option < crate :: resource :: Oem > , # [serde (skip_serializing_if = "Option::is_none" , rename = "PowerLoadPercent")] pub power_load_percent : Option < crate :: power_distribution_metrics :: v1_4_0 :: PowerDistributionMetricsPowerLoadPercent > , # [serde (skip_serializing_if = "Option::is_none" , rename = "PowerWatts")] pub power_watts : Option < crate :: power_distribution_metrics :: v1_4_0 :: PowerDistributionMetricsPowerWatts > , # [serde (skip_serializing_if = "Option::is_none" , rename = "TemperatureCelsius")] pub temperature_celsius : Option < crate :: power_distribution_metrics :: v1_4_0 :: PowerDistributionMetricsTemperatureCelsius > }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerDistributionMetricsAbsoluteHumidity {
        V000001(
            crate::power_distribution_metrics::v1_4_0::PowerDistributionMetricsAbsoluteHumidityN1,
        ),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for PowerDistributionMetricsAbsoluteHumidity {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerDistributionMetricsAbsoluteHumidityN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerDistributionMetricsAmbientTemperatureCelsius {
        V000001 (crate :: power_distribution_metrics :: v1_4_0 :: PowerDistributionMetricsAmbientTemperatureCelsiusN1) , SensorSensorExcerpt (crate :: sensor :: v1_9_2 :: SensorExcerpt) }
    impl Default for PowerDistributionMetricsAmbientTemperatureCelsius {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerDistributionMetricsAmbientTemperatureCelsiusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerDistributionMetricsDescription {
        V000001(crate::power_distribution_metrics::v1_4_0::PowerDistributionMetricsDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for PowerDistributionMetricsDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerDistributionMetricsDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerDistributionMetricsEnergykWh {
        V000001(crate::power_distribution_metrics::v1_4_0::PowerDistributionMetricsEnergykWhN1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_2::SensorEnergykWhExcerpt),
    }
    impl Default for PowerDistributionMetricsEnergykWh {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerDistributionMetricsEnergykWhN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerDistributionMetricsHumidityPercent {
        V000001(
            crate::power_distribution_metrics::v1_4_0::PowerDistributionMetricsHumidityPercentN1,
        ),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for PowerDistributionMetricsHumidityPercent {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerDistributionMetricsHumidityPercentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerDistributionMetricsPowerLoadPercent {
        V000001(
            crate::power_distribution_metrics::v1_4_0::PowerDistributionMetricsPowerLoadPercentN1,
        ),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for PowerDistributionMetricsPowerLoadPercent {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerDistributionMetricsPowerLoadPercentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerDistributionMetricsPowerWatts {
        V000001(crate::power_distribution_metrics::v1_4_0::PowerDistributionMetricsPowerWattsN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_2::SensorPowerExcerpt),
    }
    impl Default for PowerDistributionMetricsPowerWatts {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerDistributionMetricsPowerWattsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerDistributionMetricsTemperatureCelsius {
        V000001(
            crate::power_distribution_metrics::v1_4_0::PowerDistributionMetricsTemperatureCelsiusN1,
        ),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for PowerDistributionMetricsTemperatureCelsius {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerDistributionMetricsTemperatureCelsiusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetMetricsRequestBody {}
}
