use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MetricDefinition {
    V010302(crate::metric_definition::v1_3_2::MetricDefinition),
    V010203(crate::metric_definition::v1_2_3::MetricDefinition),
    V010105(crate::metric_definition::v1_1_5::MetricDefinition),
    V010010(crate::metric_definition::v1_0_10::MetricDefinition),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::metric_definition::v1_0_10::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Calculable {
        #[default]
        #[serde(rename = "NonCalculatable")]
        NonCalculatable,
        #[serde(rename = "NonSummable")]
        NonSummable,
        #[serde(rename = "Summable")]
        Summable,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CalculationAlgorithmEnum {
        #[default]
        #[serde(rename = "Average")]
        Average,
        #[serde(rename = "Maximum")]
        Maximum,
        #[serde(rename = "Minimum")]
        Minimum,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CalculationParamsType {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResultMetric")]
        pub result_metric: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourceMetric")]
        pub source_metric: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ImplementationType {
        #[default]
        #[serde(rename = "Calculated")]
        Calculated,
        #[serde(rename = "DigitalMeter")]
        DigitalMeter,
        #[serde(rename = "PhysicalSensor")]
        PhysicalSensor,
        #[serde(rename = "Synthesized")]
        Synthesized,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricDataType {
        #[default]
        #[serde(rename = "Boolean")]
        Boolean,
        #[serde(rename = "DateTime")]
        DateTime,
        #[serde(rename = "Decimal")]
        Decimal,
        #[serde(rename = "Enumeration")]
        Enumeration,
        #[serde(rename = "Integer")]
        Integer,
        #[serde(rename = "String")]
        String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MetricDefinition {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::metric_definition::v1_0_10::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Calculable")]
        pub calculable: Option<crate::metric_definition::v1_0_10::Calculable>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationAlgorithm"
        )]
        pub calculation_algorithm:
            Option<crate::metric_definition::v1_0_10::CalculationAlgorithmEnum>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationParameters"
        )]
        pub calculation_parameters:
            Option<Vec<crate::metric_definition::v1_0_10::CalculationParamsType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationTimeInterval"
        )]
        pub calculation_time_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Calibration")]
        pub calibration: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiscreteValues")]
        pub discrete_values: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::metric_definition::v1_0_10::ImplementationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsLinear")]
        pub is_linear: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricDataType")]
        pub metric_data_type: Option<crate::metric_definition::v1_0_10::MetricDataType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricType")]
        pub metric_type: Option<crate::metric_definition::v1_0_10::MetricType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<f64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Precision")]
        pub precision: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensingInterval")]
        pub sensing_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimestampAccuracy")]
        pub timestamp_accuracy: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Units")]
        pub units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Wildcards")]
        pub wildcards: Option<Vec<crate::metric_definition::v1_0_10::Wildcard>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricType {
        #[default]
        #[serde(rename = "Countdown")]
        Countdown,
        #[serde(rename = "Counter")]
        Counter,
        #[serde(rename = "Discrete")]
        Discrete,
        #[serde(rename = "Gauge")]
        Gauge,
        #[serde(rename = "Numeric")]
        Numeric,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Wildcard {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Values")]
        pub values: Option<Vec<String>>,
    }
}
pub mod v1_1_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::metric_definition::v1_1_5::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Calculable {
        #[default]
        #[serde(rename = "NonCalculatable")]
        NonCalculatable,
        #[serde(rename = "NonSummable")]
        NonSummable,
        #[serde(rename = "Summable")]
        Summable,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CalculationAlgorithmEnum {
        #[default]
        #[serde(rename = "Average")]
        Average,
        #[serde(rename = "Maximum")]
        Maximum,
        #[serde(rename = "Minimum")]
        Minimum,
        #[serde(rename = "OEM")]
        OEM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CalculationParamsType {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResultMetric")]
        pub result_metric: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourceMetric")]
        pub source_metric: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ImplementationType {
        #[default]
        #[serde(rename = "Calculated")]
        Calculated,
        #[serde(rename = "DigitalMeter")]
        DigitalMeter,
        #[serde(rename = "PhysicalSensor")]
        PhysicalSensor,
        #[serde(rename = "Synthesized")]
        Synthesized,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricDataType {
        #[default]
        #[serde(rename = "Boolean")]
        Boolean,
        #[serde(rename = "DateTime")]
        DateTime,
        #[serde(rename = "Decimal")]
        Decimal,
        #[serde(rename = "Enumeration")]
        Enumeration,
        #[serde(rename = "Integer")]
        Integer,
        #[serde(rename = "String")]
        String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MetricDefinition {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::metric_definition::v1_1_5::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Calculable")]
        pub calculable: Option<crate::metric_definition::v1_1_5::Calculable>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationAlgorithm"
        )]
        pub calculation_algorithm:
            Option<crate::metric_definition::v1_1_5::CalculationAlgorithmEnum>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationParameters"
        )]
        pub calculation_parameters:
            Option<Vec<crate::metric_definition::v1_1_5::CalculationParamsType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationTimeInterval"
        )]
        pub calculation_time_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Calibration")]
        pub calibration: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiscreteValues")]
        pub discrete_values: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::metric_definition::v1_1_5::ImplementationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsLinear")]
        pub is_linear: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricDataType")]
        pub metric_data_type: Option<crate::metric_definition::v1_1_5::MetricDataType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricType")]
        pub metric_type: Option<crate::metric_definition::v1_1_5::MetricType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<f64>,
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
            rename = "OEMCalculationAlgorithm"
        )]
        pub oem_calculation_algorithm: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Precision")]
        pub precision: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensingInterval")]
        pub sensing_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimestampAccuracy")]
        pub timestamp_accuracy: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Units")]
        pub units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Wildcards")]
        pub wildcards: Option<Vec<crate::metric_definition::v1_1_5::Wildcard>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricType {
        #[default]
        #[serde(rename = "Countdown")]
        Countdown,
        #[serde(rename = "Counter")]
        Counter,
        #[serde(rename = "Discrete")]
        Discrete,
        #[serde(rename = "Gauge")]
        Gauge,
        #[serde(rename = "Numeric")]
        Numeric,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Wildcard {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Values")]
        pub values: Option<Vec<String>>,
    }
}
pub mod v1_2_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::metric_definition::v1_2_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Calculable {
        #[default]
        #[serde(rename = "NonCalculatable")]
        NonCalculatable,
        #[serde(rename = "NonSummable")]
        NonSummable,
        #[serde(rename = "Summable")]
        Summable,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CalculationAlgorithmEnum {
        #[default]
        #[serde(rename = "Average")]
        Average,
        #[serde(rename = "Maximum")]
        Maximum,
        #[serde(rename = "Minimum")]
        Minimum,
        #[serde(rename = "OEM")]
        OEM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CalculationParamsType {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResultMetric")]
        pub result_metric: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourceMetric")]
        pub source_metric: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ImplementationType {
        #[default]
        #[serde(rename = "Calculated")]
        Calculated,
        #[serde(rename = "DigitalMeter")]
        DigitalMeter,
        #[serde(rename = "PhysicalSensor")]
        PhysicalSensor,
        #[serde(rename = "Synthesized")]
        Synthesized,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricDataType {
        #[default]
        #[serde(rename = "Boolean")]
        Boolean,
        #[serde(rename = "DateTime")]
        DateTime,
        #[serde(rename = "Decimal")]
        Decimal,
        #[serde(rename = "Enumeration")]
        Enumeration,
        #[serde(rename = "Integer")]
        Integer,
        #[serde(rename = "String")]
        String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MetricDefinition {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::metric_definition::v1_2_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Calculable")]
        pub calculable: Option<crate::metric_definition::v1_2_3::Calculable>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationAlgorithm"
        )]
        pub calculation_algorithm:
            Option<crate::metric_definition::v1_2_3::CalculationAlgorithmEnum>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationParameters"
        )]
        pub calculation_parameters:
            Option<Vec<crate::metric_definition::v1_2_3::CalculationParamsType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationTimeInterval"
        )]
        pub calculation_time_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Calibration")]
        pub calibration: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiscreteValues")]
        pub discrete_values: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::metric_definition::v1_2_3::ImplementationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsLinear")]
        pub is_linear: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricDataType")]
        pub metric_data_type: Option<crate::metric_definition::v1_2_3::MetricDataType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricType")]
        pub metric_type: Option<crate::metric_definition::v1_2_3::MetricType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<f64>,
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
            rename = "OEMCalculationAlgorithm"
        )]
        pub oem_calculation_algorithm: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Precision")]
        pub precision: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensingInterval")]
        pub sensing_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimestampAccuracy")]
        pub timestamp_accuracy: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Units")]
        pub units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Wildcards")]
        pub wildcards: Option<Vec<crate::metric_definition::v1_2_3::Wildcard>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricType {
        #[default]
        #[serde(rename = "Countdown")]
        Countdown,
        #[serde(rename = "Counter")]
        Counter,
        #[serde(rename = "Discrete")]
        Discrete,
        #[serde(rename = "Gauge")]
        Gauge,
        #[serde(rename = "Numeric")]
        Numeric,
        #[serde(rename = "String")]
        String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Wildcard {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Values")]
        pub values: Option<Vec<String>>,
    }
}
pub mod v1_3_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::metric_definition::v1_3_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Calculable {
        #[default]
        #[serde(rename = "NonCalculatable")]
        NonCalculatable,
        #[serde(rename = "NonSummable")]
        NonSummable,
        #[serde(rename = "Summable")]
        Summable,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CalculationAlgorithmEnum {
        #[default]
        #[serde(rename = "Average")]
        Average,
        #[serde(rename = "Maximum")]
        Maximum,
        #[serde(rename = "Minimum")]
        Minimum,
        #[serde(rename = "OEM")]
        OEM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CalculationParamsType {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResultMetric")]
        pub result_metric: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourceMetric")]
        pub source_metric: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ImplementationType {
        #[default]
        #[serde(rename = "Calculated")]
        Calculated,
        #[serde(rename = "DigitalMeter")]
        DigitalMeter,
        #[serde(rename = "PhysicalSensor")]
        PhysicalSensor,
        #[serde(rename = "Synthesized")]
        Synthesized,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricDataType {
        #[default]
        #[serde(rename = "Boolean")]
        Boolean,
        #[serde(rename = "DateTime")]
        DateTime,
        #[serde(rename = "Decimal")]
        Decimal,
        #[serde(rename = "Enumeration")]
        Enumeration,
        #[serde(rename = "Integer")]
        Integer,
        #[serde(rename = "String")]
        String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MetricDefinition {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Accuracy")]
        pub accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::metric_definition::v1_3_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Calculable")]
        pub calculable: Option<crate::metric_definition::v1_3_2::Calculable>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationAlgorithm"
        )]
        pub calculation_algorithm:
            Option<crate::metric_definition::v1_3_2::CalculationAlgorithmEnum>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationParameters"
        )]
        pub calculation_parameters:
            Option<Vec<crate::metric_definition::v1_3_2::CalculationParamsType>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationTimeInterval"
        )]
        pub calculation_time_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Calibration")]
        pub calibration: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiscreteValues")]
        pub discrete_values: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation: Option<crate::metric_definition::v1_3_2::ImplementationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsLinear")]
        pub is_linear: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalContexts")]
        pub logical_contexts: Option<Vec<crate::physical_context::LogicalContext>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricDataType")]
        pub metric_data_type: Option<crate::metric_definition::v1_3_2::MetricDataType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricType")]
        pub metric_type: Option<crate::metric_definition::v1_3_2::MetricType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<f64>,
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
            rename = "OEMCalculationAlgorithm"
        )]
        pub oem_calculation_algorithm: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Precision")]
        pub precision: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensingInterval")]
        pub sensing_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimestampAccuracy")]
        pub timestamp_accuracy: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Units")]
        pub units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Wildcards")]
        pub wildcards: Option<Vec<crate::metric_definition::v1_3_2::Wildcard>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricType {
        #[default]
        #[serde(rename = "Countdown")]
        Countdown,
        #[serde(rename = "Counter")]
        Counter,
        #[serde(rename = "Discrete")]
        Discrete,
        #[serde(rename = "Gauge")]
        Gauge,
        #[serde(rename = "Numeric")]
        Numeric,
        #[serde(rename = "String")]
        String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Wildcard {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Values")]
        pub values: Option<Vec<String>>,
    }
}
