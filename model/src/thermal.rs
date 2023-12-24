use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Thermal {
    V010701(crate::thermal::v1_7_1::Thermal),
    V010603(crate::thermal::v1_6_3::Thermal),
    V010506(crate::thermal::v1_5_6::Thermal),
    V010407(crate::thermal::v1_4_7::Thermal),
    V010308(crate::thermal::v1_3_8::Thermal),
    V010209(crate::thermal::v1_2_9::Thermal),
    V010110(crate::thermal::v1_1_10::Thermal),
    V010012(crate::thermal::v1_0_12::Thermal),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_12 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Fan {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanName")]
        pub fan_name: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<i64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<i64>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<crate::thermal::v1_0_12::ReadingUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingUnits {
        #[default]
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "RPM")]
        RPM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Temperature {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxReadingRangeTemp"
        )]
        pub max_reading_range_temp: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinReadingRangeTemp"
        )]
        pub min_reading_range_temp: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingCelsius")]
        pub reading_celsius: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorNumber")]
        pub sensor_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thermal {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans")]
        pub fans: Option<Vec<crate::thermal::v1_0_12::Fan>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans@odata.count")]
        pub fans_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperatures")]
        pub temperatures: Option<Vec<crate::thermal::v1_0_12::Temperature>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Temperatures@odata.count"
        )]
        pub temperatures_odata_count: Option<i64>,
    }
}
pub mod v1_1_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Fan {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanName")]
        pub fan_name: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<i64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<crate::thermal::v1_1_10::ReadingUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingUnits {
        #[default]
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "RPM")]
        RPM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Temperature {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxReadingRangeTemp"
        )]
        pub max_reading_range_temp: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinReadingRangeTemp"
        )]
        pub min_reading_range_temp: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingCelsius")]
        pub reading_celsius: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorNumber")]
        pub sensor_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thermal {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans")]
        pub fans: Option<Vec<crate::thermal::v1_1_10::Fan>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans@odata.count")]
        pub fans_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperatures")]
        pub temperatures: Option<Vec<crate::thermal::v1_1_10::Temperature>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Temperatures@odata.count"
        )]
        pub temperatures_odata_count: Option<i64>,
    }
}
pub mod v1_2_9 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Fan {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanName")]
        pub fan_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<i64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<crate::thermal::v1_2_9::ReadingUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingUnits {
        #[default]
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "RPM")]
        RPM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Temperature {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxReadingRangeTemp"
        )]
        pub max_reading_range_temp: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinReadingRangeTemp"
        )]
        pub min_reading_range_temp: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingCelsius")]
        pub reading_celsius: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorNumber")]
        pub sensor_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thermal {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans")]
        pub fans: Option<Vec<crate::thermal::v1_2_9::Fan>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans@odata.count")]
        pub fans_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperatures")]
        pub temperatures: Option<Vec<crate::thermal::v1_2_9::Temperature>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Temperatures@odata.count"
        )]
        pub temperatures_odata_count: Option<i64>,
    }
}
pub mod v1_3_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Fan {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_3_8::FanActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanName")]
        pub fan_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<i64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<crate::thermal::v1_3_8::ReadingUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FanActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_3_8::FanOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FanOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingUnits {
        #[default]
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "RPM")]
        RPM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Temperature {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_3_8::TemperatureActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxReadingRangeTemp"
        )]
        pub max_reading_range_temp: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinReadingRangeTemp"
        )]
        pub min_reading_range_temp: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingCelsius")]
        pub reading_celsius: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorNumber")]
        pub sensor_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TemperatureActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_3_8::TemperatureOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TemperatureOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thermal {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_3_8::ThermalActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans")]
        pub fans: Option<Vec<crate::thermal::v1_3_8::Fan>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans@odata.count")]
        pub fans_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperatures")]
        pub temperatures: Option<Vec<crate::thermal::v1_3_8::Temperature>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Temperatures@odata.count"
        )]
        pub temperatures_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_3_8::ThermalOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalOemActions {}
}
pub mod v1_4_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Fan {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_4_7::FanActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanName")]
        pub fan_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotPluggable")]
        pub hot_pluggable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<i64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<crate::thermal::v1_4_7::ReadingUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FanActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_4_7::FanOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FanOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingUnits {
        #[default]
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "RPM")]
        RPM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Temperature {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_4_7::TemperatureActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdjustedMaxAllowableOperatingValue"
        )]
        pub adjusted_max_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdjustedMinAllowableOperatingValue"
        )]
        pub adjusted_min_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaPhysicalContext"
        )]
        pub delta_physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaReadingCelsius"
        )]
        pub delta_reading_celsius: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxAllowableOperatingValue"
        )]
        pub max_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxReadingRangeTemp"
        )]
        pub max_reading_range_temp: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinAllowableOperatingValue"
        )]
        pub min_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinReadingRangeTemp"
        )]
        pub min_reading_range_temp: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingCelsius")]
        pub reading_celsius: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorNumber")]
        pub sensor_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TemperatureActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_4_7::TemperatureOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TemperatureOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thermal {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_4_7::ThermalActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans")]
        pub fans: Option<Vec<crate::thermal::v1_4_7::Fan>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans@odata.count")]
        pub fans_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperatures")]
        pub temperatures: Option<Vec<crate::thermal::v1_4_7::Temperature>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Temperatures@odata.count"
        )]
        pub temperatures_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_4_7::ThermalOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalOemActions {}
}
pub mod v1_5_6 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Fan {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_5_6::FanActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanName")]
        pub fan_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotPluggable")]
        pub hot_pluggable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<i64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<crate::thermal::v1_5_6::ReadingUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorNumber")]
        pub sensor_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FanActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_5_6::FanOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FanOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingUnits {
        #[default]
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "RPM")]
        RPM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Temperature {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_5_6::TemperatureActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdjustedMaxAllowableOperatingValue"
        )]
        pub adjusted_max_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdjustedMinAllowableOperatingValue"
        )]
        pub adjusted_min_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaPhysicalContext"
        )]
        pub delta_physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaReadingCelsius"
        )]
        pub delta_reading_celsius: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxAllowableOperatingValue"
        )]
        pub max_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxReadingRangeTemp"
        )]
        pub max_reading_range_temp: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinAllowableOperatingValue"
        )]
        pub min_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinReadingRangeTemp"
        )]
        pub min_reading_range_temp: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingCelsius")]
        pub reading_celsius: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorNumber")]
        pub sensor_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TemperatureActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_5_6::TemperatureOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TemperatureOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thermal {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_5_6::ThermalActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans")]
        pub fans: Option<Vec<crate::thermal::v1_5_6::Fan>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans@odata.count")]
        pub fans_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperatures")]
        pub temperatures: Option<Vec<crate::thermal::v1_5_6::Temperature>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Temperatures@odata.count"
        )]
        pub temperatures_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_5_6::ThermalOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalOemActions {}
}
pub mod v1_6_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Fan {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_6_3::FanActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanName")]
        pub fan_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotPluggable")]
        pub hot_pluggable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<i64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<crate::thermal::v1_6_3::ReadingUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorNumber")]
        pub sensor_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FanActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_6_3::FanOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FanOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingUnits {
        #[default]
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "RPM")]
        RPM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Temperature {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_6_3::TemperatureActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdjustedMaxAllowableOperatingValue"
        )]
        pub adjusted_max_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdjustedMinAllowableOperatingValue"
        )]
        pub adjusted_min_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaPhysicalContext"
        )]
        pub delta_physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaReadingCelsius"
        )]
        pub delta_reading_celsius: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerThresholdUser")]
        pub lower_threshold_user: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxAllowableOperatingValue"
        )]
        pub max_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxReadingRangeTemp"
        )]
        pub max_reading_range_temp: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinAllowableOperatingValue"
        )]
        pub min_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinReadingRangeTemp"
        )]
        pub min_reading_range_temp: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingCelsius")]
        pub reading_celsius: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorNumber")]
        pub sensor_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperThresholdUser")]
        pub upper_threshold_user: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TemperatureActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_6_3::TemperatureOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TemperatureOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thermal {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_6_3::ThermalActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans")]
        pub fans: Option<Vec<crate::thermal::v1_6_3::Fan>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans@odata.count")]
        pub fans_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperatures")]
        pub temperatures: Option<Vec<crate::thermal::v1_6_3::Temperature>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Temperatures@odata.count"
        )]
        pub temperatures_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_6_3::ThermalOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalOemActions {}
}
pub mod v1_7_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Fan {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_7_1::FanActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanName")]
        pub fan_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotPluggable")]
        pub hot_pluggable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<i64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reading")]
        pub reading: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingUnits")]
        pub reading_units: Option<crate::thermal::v1_7_1::ReadingUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorNumber")]
        pub sensor_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FanActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_7_1::FanOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FanOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ReadingUnits {
        #[default]
        #[serde(rename = "Percent")]
        Percent,
        #[serde(rename = "RPM")]
        RPM,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Temperature {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_7_1::TemperatureActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdjustedMaxAllowableOperatingValue"
        )]
        pub adjusted_max_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdjustedMinAllowableOperatingValue"
        )]
        pub adjusted_min_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaPhysicalContext"
        )]
        pub delta_physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaReadingCelsius"
        )]
        pub delta_reading_celsius: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdCritical"
        )]
        pub lower_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdFatal"
        )]
        pub lower_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowerThresholdNonCritical"
        )]
        pub lower_threshold_non_critical: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowerThresholdUser")]
        pub lower_threshold_user: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxAllowableOperatingValue"
        )]
        pub max_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxReadingRangeTemp"
        )]
        pub max_reading_range_temp: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinAllowableOperatingValue"
        )]
        pub min_allowable_operating_value: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MinReadingRangeTemp"
        )]
        pub min_reading_range_temp: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingCelsius")]
        pub reading_celsius: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorNumber")]
        pub sensor_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdCritical"
        )]
        pub upper_threshold_critical: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdFatal"
        )]
        pub upper_threshold_fatal: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpperThresholdNonCritical"
        )]
        pub upper_threshold_non_critical: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpperThresholdUser")]
        pub upper_threshold_user: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TemperatureActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_7_1::TemperatureOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TemperatureOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Thermal {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal::v1_7_1::ThermalActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans")]
        pub fans: Option<Vec<crate::thermal::v1_7_1::Fan>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans@odata.count")]
        pub fans_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperatures")]
        pub temperatures: Option<Vec<crate::thermal::v1_7_1::Temperature>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Temperatures@odata.count"
        )]
        pub temperatures_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal::v1_7_1::ThermalOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalOemActions {}
}
