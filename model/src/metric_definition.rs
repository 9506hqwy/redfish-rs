pub type MetricDefinition = crate::metric_definition::v1_3_4::MetricDefinition;
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
pub mod v1_3_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::metric_definition::v1_3_4::OemActions>,
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
        pub actions: Option<crate::metric_definition::v1_3_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Calculable")]
        pub calculable: Option<crate::metric_definition::v1_3_4::MetricDefinitionCalculable>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationAlgorithm"
        )]
        pub calculation_algorithm:
            Option<crate::metric_definition::v1_3_4::MetricDefinitionCalculationAlgorithm>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationParameters"
        )]
        pub calculation_parameters:
            Option<Vec<crate::metric_definition::v1_3_4::MetricDefinitionCalculationParameters>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CalculationTimeInterval"
        )]
        pub calculation_time_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Calibration")]
        pub calibration: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::metric_definition::v1_3_4::MetricDefinitionDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiscreteValues")]
        pub discrete_values: Option<Vec<String>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Implementation")]
        pub implementation:
            Option<crate::metric_definition::v1_3_4::MetricDefinitionImplementation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsLinear")]
        pub is_linear: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogicalContexts")]
        pub logical_contexts: Option<Vec<crate::physical_context::LogicalContext>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricDataType")]
        pub metric_data_type:
            Option<crate::metric_definition::v1_3_4::MetricDefinitionMetricDataType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricProperties")]
        pub metric_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MetricType")]
        pub metric_type: Option<crate::metric_definition::v1_3_4::MetricDefinitionMetricType>,
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
        pub physical_context:
            Option<crate::metric_definition::v1_3_4::MetricDefinitionPhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Precision")]
        pub precision: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensingInterval")]
        pub sensing_interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TimestampAccuracy")]
        pub timestamp_accuracy: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Units")]
        pub units: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Wildcards")]
        pub wildcards: Option<Vec<crate::metric_definition::v1_3_4::Wildcard>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MetricDefinitionCalculable {
        V010304(crate::metric_definition::v1_3_4::Calculable),
        V000001(crate::metric_definition::v1_3_4::MetricDefinitionCalculableN1),
    }
    impl Default for MetricDefinitionCalculable {
        fn default() -> Self {
            Self::V010304(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricDefinitionCalculableN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MetricDefinitionCalculationAlgorithm {
        V010304(crate::metric_definition::v1_3_4::CalculationAlgorithmEnum),
        V000001(crate::metric_definition::v1_3_4::MetricDefinitionCalculationAlgorithmN1),
    }
    impl Default for MetricDefinitionCalculationAlgorithm {
        fn default() -> Self {
            Self::V010304(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricDefinitionCalculationAlgorithmN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MetricDefinitionCalculationParameters {
        V010304(crate::metric_definition::v1_3_4::CalculationParamsType),
        V000001(crate::metric_definition::v1_3_4::MetricDefinitionCalculationParametersN1),
    }
    impl Default for MetricDefinitionCalculationParameters {
        fn default() -> Self {
            Self::V010304(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricDefinitionCalculationParametersN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MetricDefinitionDescription {
        V000001(crate::metric_definition::v1_3_4::MetricDefinitionDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for MetricDefinitionDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricDefinitionDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MetricDefinitionImplementation {
        V010304(crate::metric_definition::v1_3_4::ImplementationType),
        V000001(crate::metric_definition::v1_3_4::MetricDefinitionImplementationN1),
    }
    impl Default for MetricDefinitionImplementation {
        fn default() -> Self {
            Self::V010304(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricDefinitionImplementationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MetricDefinitionMetricDataType {
        V010304(crate::metric_definition::v1_3_4::MetricDataType),
        V000001(crate::metric_definition::v1_3_4::MetricDefinitionMetricDataTypeN1),
    }
    impl Default for MetricDefinitionMetricDataType {
        fn default() -> Self {
            Self::V010304(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricDefinitionMetricDataTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MetricDefinitionMetricType {
        V010304(crate::metric_definition::v1_3_4::MetricType),
        V000001(crate::metric_definition::v1_3_4::MetricDefinitionMetricTypeN1),
    }
    impl Default for MetricDefinitionMetricType {
        fn default() -> Self {
            Self::V010304(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricDefinitionMetricTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum MetricDefinitionPhysicalContext {
        V000001(crate::metric_definition::v1_3_4::MetricDefinitionPhysicalContextN1),
        PhysicalContextPhysicalContext(crate::physical_context::PhysicalContext),
    }
    impl Default for MetricDefinitionPhysicalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MetricDefinitionPhysicalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
