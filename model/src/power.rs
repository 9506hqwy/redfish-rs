use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Power {
    V010701(crate::power::v1_7_1::Power),
    V010603(crate::power::v1_6_3::Power),
    V010507(crate::power::v1_5_7::Power),
    V010408(crate::power::v1_4_8::Power),
    V010309(crate::power::v1_3_9::Power),
    V010211(crate::power::v1_2_11::Power),
    V010111(crate::power::v1_1_11::Power),
    V010013(crate::power::v1_0_13::Power),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_13 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LineInputVoltageType {
        #[default]
        #[serde(rename = "ACHighLine")]
        ACHighLine,
        #[serde(rename = "ACLowLine")]
        ACLowLine,
        #[serde(rename = "ACMidLine")]
        ACMidLine,
        #[serde(rename = "DC380V")]
        DC380V,
        #[serde(rename = "DCNeg48V")]
        DCNeg48V,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Power {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerControl")]
        pub power_control: Option<Vec<crate::power::v1_0_13::PowerControl>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerControl@odata.count"
        )]
        pub power_control_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplies")]
        pub power_supplies: Option<Vec<crate::power::v1_0_13::PowerSupply>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerSupplies@odata.count"
        )]
        pub power_supplies_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Voltages")]
        pub voltages: Option<Vec<crate::power::v1_0_13::Voltage>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Voltages@odata.count"
        )]
        pub voltages_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControl {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAllocatedWatts"
        )]
        pub power_allocated_watts: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAvailableWatts"
        )]
        pub power_available_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerConsumedWatts")]
        pub power_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerLimit")]
        pub power_limit: Option<crate::power::v1_0_13::PowerLimit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMetrics")]
        pub power_metrics: Option<crate::power::v1_0_13::PowerMetric>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerRequestedWatts"
        )]
        pub power_requested_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerLimit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CorrectionInMs")]
        pub correction_in_ms: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitException")]
        pub limit_exception: Option<crate::power::v1_0_13::PowerLimitException>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitInWatts")]
        pub limit_in_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerLimitException {
        #[default]
        #[serde(rename = "HardPowerOff")]
        HardPowerOff,
        #[serde(rename = "LogEventOnly")]
        LogEventOnly,
        #[serde(rename = "NoAction")]
        NoAction,
        #[serde(rename = "Oem")]
        Oem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerMetric {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AverageConsumedWatts"
        )]
        pub average_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IntervalInMin")]
        pub interval_in_min: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxConsumedWatts")]
        pub max_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinConsumedWatts")]
        pub min_consumed_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupply {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastPowerOutputWatts"
        )]
        pub last_power_output_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LineInputVoltage")]
        pub line_input_voltage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LineInputVoltageType"
        )]
        pub line_input_voltage_type: Option<crate::power::v1_0_13::LineInputVoltageType>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplyType")]
        pub power_supply_type: Option<crate::power::v1_0_13::PowerSupplyType>,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "ACorDC")]
        ACorDC,
        #[serde(rename = "DC")]
        DC,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Voltage {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingVolts")]
        pub reading_volts: Option<f64>,
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
}
pub mod v1_1_11 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InputRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputType")]
        pub input_type: Option<crate::power::v1_1_11::InputType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumFrequencyHz")]
        pub maximum_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumVoltage")]
        pub maximum_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumFrequencyHz")]
        pub minimum_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumVoltage")]
        pub minimum_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputWattage")]
        pub output_wattage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InputType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "DC")]
        DC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LineInputVoltageType {
        #[default]
        #[serde(rename = "AC120V")]
        AC120V,
        #[serde(rename = "AC240V")]
        AC240V,
        #[serde(rename = "AC277V")]
        AC277V,
        #[serde(rename = "ACHighLine")]
        ACHighLine,
        #[serde(rename = "ACLowLine")]
        ACLowLine,
        #[serde(rename = "ACMidLine")]
        ACMidLine,
        #[serde(rename = "ACWideRange")]
        ACWideRange,
        #[serde(rename = "ACandDCWideRange")]
        ACandDCWideRange,
        #[serde(rename = "DC240V")]
        DC240V,
        #[serde(rename = "DC380V")]
        DC380V,
        #[serde(rename = "DCNeg48V")]
        DCNeg48V,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Power {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerControl")]
        pub power_control: Option<Vec<crate::power::v1_1_11::PowerControl>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerControl@odata.count"
        )]
        pub power_control_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplies")]
        pub power_supplies: Option<Vec<crate::power::v1_1_11::PowerSupply>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerSupplies@odata.count"
        )]
        pub power_supplies_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Voltages")]
        pub voltages: Option<Vec<crate::power::v1_1_11::Voltage>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Voltages@odata.count"
        )]
        pub voltages_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControl {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAllocatedWatts"
        )]
        pub power_allocated_watts: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAvailableWatts"
        )]
        pub power_available_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerConsumedWatts")]
        pub power_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerLimit")]
        pub power_limit: Option<crate::power::v1_1_11::PowerLimit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMetrics")]
        pub power_metrics: Option<crate::power::v1_1_11::PowerMetric>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerRequestedWatts"
        )]
        pub power_requested_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerLimit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CorrectionInMs")]
        pub correction_in_ms: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitException")]
        pub limit_exception: Option<crate::power::v1_1_11::PowerLimitException>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitInWatts")]
        pub limit_in_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerLimitException {
        #[default]
        #[serde(rename = "HardPowerOff")]
        HardPowerOff,
        #[serde(rename = "LogEventOnly")]
        LogEventOnly,
        #[serde(rename = "NoAction")]
        NoAction,
        #[serde(rename = "Oem")]
        Oem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerMetric {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AverageConsumedWatts"
        )]
        pub average_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IntervalInMin")]
        pub interval_in_min: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxConsumedWatts")]
        pub max_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinConsumedWatts")]
        pub min_consumed_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupply {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputRanges")]
        pub input_ranges: Option<Vec<crate::power::v1_1_11::InputRange>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastPowerOutputWatts"
        )]
        pub last_power_output_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LineInputVoltage")]
        pub line_input_voltage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LineInputVoltageType"
        )]
        pub line_input_voltage_type: Option<crate::power::v1_1_11::LineInputVoltageType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplyType")]
        pub power_supply_type: Option<crate::power::v1_1_11::PowerSupplyType>,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "ACorDC")]
        ACorDC,
        #[serde(rename = "DC")]
        DC,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Voltage {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingVolts")]
        pub reading_volts: Option<f64>,
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
}
pub mod v1_2_11 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InputRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputType")]
        pub input_type: Option<crate::power::v1_2_11::InputType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumFrequencyHz")]
        pub maximum_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumVoltage")]
        pub maximum_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumFrequencyHz")]
        pub minimum_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumVoltage")]
        pub minimum_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputWattage")]
        pub output_wattage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InputType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "DC")]
        DC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LineInputVoltageType {
        #[default]
        #[serde(rename = "AC120V")]
        AC120V,
        #[serde(rename = "AC240V")]
        AC240V,
        #[serde(rename = "AC277V")]
        AC277V,
        #[serde(rename = "ACHighLine")]
        ACHighLine,
        #[serde(rename = "ACLowLine")]
        ACLowLine,
        #[serde(rename = "ACMidLine")]
        ACMidLine,
        #[serde(rename = "ACWideRange")]
        ACWideRange,
        #[serde(rename = "ACandDCWideRange")]
        ACandDCWideRange,
        #[serde(rename = "DC240V")]
        DC240V,
        #[serde(rename = "DC380V")]
        DC380V,
        #[serde(rename = "DCNeg48V")]
        DCNeg48V,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Power {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerControl")]
        pub power_control: Option<Vec<crate::power::v1_2_11::PowerControl>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerControl@odata.count"
        )]
        pub power_control_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplies")]
        pub power_supplies: Option<Vec<crate::power::v1_2_11::PowerSupply>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerSupplies@odata.count"
        )]
        pub power_supplies_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Voltages")]
        pub voltages: Option<Vec<crate::power::v1_2_11::Voltage>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Voltages@odata.count"
        )]
        pub voltages_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControl {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAllocatedWatts"
        )]
        pub power_allocated_watts: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAvailableWatts"
        )]
        pub power_available_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerConsumedWatts")]
        pub power_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerLimit")]
        pub power_limit: Option<crate::power::v1_2_11::PowerLimit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMetrics")]
        pub power_metrics: Option<crate::power::v1_2_11::PowerMetric>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerRequestedWatts"
        )]
        pub power_requested_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerLimit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CorrectionInMs")]
        pub correction_in_ms: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitException")]
        pub limit_exception: Option<crate::power::v1_2_11::PowerLimitException>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitInWatts")]
        pub limit_in_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerLimitException {
        #[default]
        #[serde(rename = "HardPowerOff")]
        HardPowerOff,
        #[serde(rename = "LogEventOnly")]
        LogEventOnly,
        #[serde(rename = "NoAction")]
        NoAction,
        #[serde(rename = "Oem")]
        Oem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerMetric {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AverageConsumedWatts"
        )]
        pub average_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IntervalInMin")]
        pub interval_in_min: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxConsumedWatts")]
        pub max_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinConsumedWatts")]
        pub min_consumed_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupply {
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputRanges")]
        pub input_ranges: Option<Vec<crate::power::v1_2_11::InputRange>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastPowerOutputWatts"
        )]
        pub last_power_output_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LineInputVoltage")]
        pub line_input_voltage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LineInputVoltageType"
        )]
        pub line_input_voltage_type: Option<crate::power::v1_2_11::LineInputVoltageType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplyType")]
        pub power_supply_type: Option<crate::power::v1_2_11::PowerSupplyType>,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "ACorDC")]
        ACorDC,
        #[serde(rename = "DC")]
        DC,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Voltage {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingVolts")]
        pub reading_volts: Option<f64>,
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
}
pub mod v1_3_9 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_3_9::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InputRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputType")]
        pub input_type: Option<crate::power::v1_3_9::InputType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumFrequencyHz")]
        pub maximum_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumVoltage")]
        pub maximum_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumFrequencyHz")]
        pub minimum_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumVoltage")]
        pub minimum_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputWattage")]
        pub output_wattage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InputType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "DC")]
        DC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LineInputVoltageType {
        #[default]
        #[serde(rename = "AC120V")]
        AC120V,
        #[serde(rename = "AC240V")]
        AC240V,
        #[serde(rename = "AC277V")]
        AC277V,
        #[serde(rename = "ACHighLine")]
        ACHighLine,
        #[serde(rename = "ACLowLine")]
        ACLowLine,
        #[serde(rename = "ACMidLine")]
        ACMidLine,
        #[serde(rename = "ACWideRange")]
        ACWideRange,
        #[serde(rename = "ACandDCWideRange")]
        ACandDCWideRange,
        #[serde(rename = "DC240V")]
        DC240V,
        #[serde(rename = "DC380V")]
        DC380V,
        #[serde(rename = "DCNeg48V")]
        DCNeg48V,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Power {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_3_9::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerControl")]
        pub power_control: Option<Vec<crate::power::v1_3_9::PowerControl>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerControl@odata.count"
        )]
        pub power_control_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplies")]
        pub power_supplies: Option<Vec<crate::power::v1_3_9::PowerSupply>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerSupplies@odata.count"
        )]
        pub power_supplies_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Voltages")]
        pub voltages: Option<Vec<crate::power::v1_3_9::Voltage>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Voltages@odata.count"
        )]
        pub voltages_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControl {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_3_9::PowerControlActions>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAllocatedWatts"
        )]
        pub power_allocated_watts: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAvailableWatts"
        )]
        pub power_available_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerConsumedWatts")]
        pub power_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerLimit")]
        pub power_limit: Option<crate::power::v1_3_9::PowerLimit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMetrics")]
        pub power_metrics: Option<crate::power::v1_3_9::PowerMetric>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerRequestedWatts"
        )]
        pub power_requested_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControlActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_3_9::PowerControlOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControlOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerLimit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CorrectionInMs")]
        pub correction_in_ms: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitException")]
        pub limit_exception: Option<crate::power::v1_3_9::PowerLimitException>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitInWatts")]
        pub limit_in_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerLimitException {
        #[default]
        #[serde(rename = "HardPowerOff")]
        HardPowerOff,
        #[serde(rename = "LogEventOnly")]
        LogEventOnly,
        #[serde(rename = "NoAction")]
        NoAction,
        #[serde(rename = "Oem")]
        Oem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerMetric {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AverageConsumedWatts"
        )]
        pub average_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IntervalInMin")]
        pub interval_in_min: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxConsumedWatts")]
        pub max_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinConsumedWatts")]
        pub min_consumed_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupply {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_3_9::PowerSupplyActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputRanges")]
        pub input_ranges: Option<Vec<crate::power::v1_3_9::InputRange>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastPowerOutputWatts"
        )]
        pub last_power_output_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LineInputVoltage")]
        pub line_input_voltage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LineInputVoltageType"
        )]
        pub line_input_voltage_type: Option<crate::power::v1_3_9::LineInputVoltageType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplyType")]
        pub power_supply_type: Option<crate::power::v1_3_9::PowerSupplyType>,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_3_9::PowerSupplyOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "ACorDC")]
        ACorDC,
        #[serde(rename = "DC")]
        DC,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Voltage {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_3_9::VoltageActions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingVolts")]
        pub reading_volts: Option<f64>,
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
    pub struct VoltageActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_3_9::VoltageOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VoltageOemActions {}
}
pub mod v1_4_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_4_8::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InputRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputType")]
        pub input_type: Option<crate::power::v1_4_8::InputType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumFrequencyHz")]
        pub maximum_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumVoltage")]
        pub maximum_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumFrequencyHz")]
        pub minimum_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumVoltage")]
        pub minimum_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputWattage")]
        pub output_wattage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InputType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "DC")]
        DC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LineInputVoltageType {
        #[default]
        #[serde(rename = "AC120V")]
        AC120V,
        #[serde(rename = "AC240V")]
        AC240V,
        #[serde(rename = "AC277V")]
        AC277V,
        #[serde(rename = "ACHighLine")]
        ACHighLine,
        #[serde(rename = "ACLowLine")]
        ACLowLine,
        #[serde(rename = "ACMidLine")]
        ACMidLine,
        #[serde(rename = "ACWideRange")]
        ACWideRange,
        #[serde(rename = "ACandDCWideRange")]
        ACandDCWideRange,
        #[serde(rename = "DC240V")]
        DC240V,
        #[serde(rename = "DC380V")]
        DC380V,
        #[serde(rename = "DCNeg48V")]
        DCNeg48V,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Power {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_4_8::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerControl")]
        pub power_control: Option<Vec<crate::power::v1_4_8::PowerControl>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerControl@odata.count"
        )]
        pub power_control_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplies")]
        pub power_supplies: Option<Vec<crate::power::v1_4_8::PowerSupply>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerSupplies@odata.count"
        )]
        pub power_supplies_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Voltages")]
        pub voltages: Option<Vec<crate::power::v1_4_8::Voltage>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Voltages@odata.count"
        )]
        pub voltages_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControl {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_4_8::PowerControlActions>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAllocatedWatts"
        )]
        pub power_allocated_watts: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAvailableWatts"
        )]
        pub power_available_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerConsumedWatts")]
        pub power_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerLimit")]
        pub power_limit: Option<crate::power::v1_4_8::PowerLimit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMetrics")]
        pub power_metrics: Option<crate::power::v1_4_8::PowerMetric>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerRequestedWatts"
        )]
        pub power_requested_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControlActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_4_8::PowerControlOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControlOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerLimit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CorrectionInMs")]
        pub correction_in_ms: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitException")]
        pub limit_exception: Option<crate::power::v1_4_8::PowerLimitException>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitInWatts")]
        pub limit_in_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerLimitException {
        #[default]
        #[serde(rename = "HardPowerOff")]
        HardPowerOff,
        #[serde(rename = "LogEventOnly")]
        LogEventOnly,
        #[serde(rename = "NoAction")]
        NoAction,
        #[serde(rename = "Oem")]
        Oem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerMetric {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AverageConsumedWatts"
        )]
        pub average_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IntervalInMin")]
        pub interval_in_min: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxConsumedWatts")]
        pub max_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinConsumedWatts")]
        pub min_consumed_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupply {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_4_8::PowerSupplyActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputRanges")]
        pub input_ranges: Option<Vec<crate::power::v1_4_8::InputRange>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastPowerOutputWatts"
        )]
        pub last_power_output_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LineInputVoltage")]
        pub line_input_voltage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LineInputVoltageType"
        )]
        pub line_input_voltage_type: Option<crate::power::v1_4_8::LineInputVoltageType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplyType")]
        pub power_supply_type: Option<crate::power::v1_4_8::PowerSupplyType>,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_4_8::PowerSupplyOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "ACorDC")]
        ACorDC,
        #[serde(rename = "DC")]
        DC,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Voltage {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_4_8::VoltageActions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingVolts")]
        pub reading_volts: Option<f64>,
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
    pub struct VoltageActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_4_8::VoltageOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VoltageOemActions {}
}
pub mod v1_5_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_5_7::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InputRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputType")]
        pub input_type: Option<crate::power::v1_5_7::InputType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumFrequencyHz")]
        pub maximum_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumVoltage")]
        pub maximum_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumFrequencyHz")]
        pub minimum_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumVoltage")]
        pub minimum_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputWattage")]
        pub output_wattage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InputType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "DC")]
        DC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LineInputVoltageType {
        #[default]
        #[serde(rename = "AC120V")]
        AC120V,
        #[serde(rename = "AC240V")]
        AC240V,
        #[serde(rename = "AC277V")]
        AC277V,
        #[serde(rename = "ACHighLine")]
        ACHighLine,
        #[serde(rename = "ACLowLine")]
        ACLowLine,
        #[serde(rename = "ACMidLine")]
        ACMidLine,
        #[serde(rename = "ACWideRange")]
        ACWideRange,
        #[serde(rename = "ACandDCWideRange")]
        ACandDCWideRange,
        #[serde(rename = "DC240V")]
        DC240V,
        #[serde(rename = "DC380V")]
        DC380V,
        #[serde(rename = "DCNeg48V")]
        DCNeg48V,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Power {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_5_7::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerControl")]
        pub power_control: Option<Vec<crate::power::v1_5_7::PowerControl>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerControl@odata.count"
        )]
        pub power_control_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplies")]
        pub power_supplies: Option<Vec<crate::power::v1_5_7::PowerSupply>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerSupplies@odata.count"
        )]
        pub power_supplies_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Voltages")]
        pub voltages: Option<Vec<crate::power::v1_5_7::Voltage>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Voltages@odata.count"
        )]
        pub voltages_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControl {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_5_7::PowerControlActions>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAllocatedWatts"
        )]
        pub power_allocated_watts: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAvailableWatts"
        )]
        pub power_available_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerConsumedWatts")]
        pub power_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerLimit")]
        pub power_limit: Option<crate::power::v1_5_7::PowerLimit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMetrics")]
        pub power_metrics: Option<crate::power::v1_5_7::PowerMetric>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerRequestedWatts"
        )]
        pub power_requested_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControlActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_5_7::PowerControlOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControlOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerLimit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CorrectionInMs")]
        pub correction_in_ms: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitException")]
        pub limit_exception: Option<crate::power::v1_5_7::PowerLimitException>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitInWatts")]
        pub limit_in_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerLimitException {
        #[default]
        #[serde(rename = "HardPowerOff")]
        HardPowerOff,
        #[serde(rename = "LogEventOnly")]
        LogEventOnly,
        #[serde(rename = "NoAction")]
        NoAction,
        #[serde(rename = "Oem")]
        Oem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerMetric {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AverageConsumedWatts"
        )]
        pub average_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IntervalInMin")]
        pub interval_in_min: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxConsumedWatts")]
        pub max_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinConsumedWatts")]
        pub min_consumed_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupply {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_5_7::PowerSupplyActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EfficiencyPercent")]
        pub efficiency_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotPluggable")]
        pub hot_pluggable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputRanges")]
        pub input_ranges: Option<Vec<crate::power::v1_5_7::InputRange>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastPowerOutputWatts"
        )]
        pub last_power_output_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LineInputVoltage")]
        pub line_input_voltage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LineInputVoltageType"
        )]
        pub line_input_voltage_type: Option<crate::power::v1_5_7::LineInputVoltageType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerInputWatts")]
        pub power_input_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerOutputWatts")]
        pub power_output_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplyType")]
        pub power_supply_type: Option<crate::power::v1_5_7::PowerSupplyType>,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_5_7::PowerSupplyOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "ACorDC")]
        ACorDC,
        #[serde(rename = "DC")]
        DC,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Voltage {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_5_7::VoltageActions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingVolts")]
        pub reading_volts: Option<f64>,
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
    pub struct VoltageActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_5_7::VoltageOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VoltageOemActions {}
}
pub mod v1_6_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_6_3::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Power.PowerSupplyReset"
        )]
        pub power_power_supply_reset: Option<crate::power::v1_6_3::PowerSupplyReset>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InputRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputType")]
        pub input_type: Option<crate::power::v1_6_3::InputType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumFrequencyHz")]
        pub maximum_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumVoltage")]
        pub maximum_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumFrequencyHz")]
        pub minimum_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumVoltage")]
        pub minimum_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputWattage")]
        pub output_wattage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InputType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "DC")]
        DC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LineInputVoltageType {
        #[default]
        #[serde(rename = "AC120V")]
        AC120V,
        #[serde(rename = "AC240V")]
        AC240V,
        #[serde(rename = "AC277V")]
        AC277V,
        #[serde(rename = "ACHighLine")]
        ACHighLine,
        #[serde(rename = "ACLowLine")]
        ACLowLine,
        #[serde(rename = "ACMidLine")]
        ACMidLine,
        #[serde(rename = "ACWideRange")]
        ACWideRange,
        #[serde(rename = "ACandDCWideRange")]
        ACandDCWideRange,
        #[serde(rename = "DC240V")]
        DC240V,
        #[serde(rename = "DC380V")]
        DC380V,
        #[serde(rename = "DCNeg48V")]
        DCNeg48V,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Power {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_6_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerControl")]
        pub power_control: Option<Vec<crate::power::v1_6_3::PowerControl>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerControl@odata.count"
        )]
        pub power_control_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplies")]
        pub power_supplies: Option<Vec<crate::power::v1_6_3::PowerSupply>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerSupplies@odata.count"
        )]
        pub power_supplies_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Voltages")]
        pub voltages: Option<Vec<crate::power::v1_6_3::Voltage>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Voltages@odata.count"
        )]
        pub voltages_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControl {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_6_3::PowerControlActions>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAllocatedWatts"
        )]
        pub power_allocated_watts: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAvailableWatts"
        )]
        pub power_available_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerConsumedWatts")]
        pub power_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerLimit")]
        pub power_limit: Option<crate::power::v1_6_3::PowerLimit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMetrics")]
        pub power_metrics: Option<crate::power::v1_6_3::PowerMetric>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerRequestedWatts"
        )]
        pub power_requested_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControlActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_6_3::PowerControlOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControlOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerLimit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CorrectionInMs")]
        pub correction_in_ms: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitException")]
        pub limit_exception: Option<crate::power::v1_6_3::PowerLimitException>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitInWatts")]
        pub limit_in_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerLimitException {
        #[default]
        #[serde(rename = "HardPowerOff")]
        HardPowerOff,
        #[serde(rename = "LogEventOnly")]
        LogEventOnly,
        #[serde(rename = "NoAction")]
        NoAction,
        #[serde(rename = "Oem")]
        Oem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerMetric {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AverageConsumedWatts"
        )]
        pub average_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IntervalInMin")]
        pub interval_in_min: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxConsumedWatts")]
        pub max_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinConsumedWatts")]
        pub min_consumed_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupply {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_6_3::PowerSupplyActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EfficiencyPercent")]
        pub efficiency_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotPluggable")]
        pub hot_pluggable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputRanges")]
        pub input_ranges: Option<Vec<crate::power::v1_6_3::InputRange>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastPowerOutputWatts"
        )]
        pub last_power_output_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LineInputVoltage")]
        pub line_input_voltage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LineInputVoltageType"
        )]
        pub line_input_voltage_type: Option<crate::power::v1_6_3::LineInputVoltageType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerInputWatts")]
        pub power_input_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerOutputWatts")]
        pub power_output_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplyType")]
        pub power_supply_type: Option<crate::power::v1_6_3::PowerSupplyType>,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_6_3::PowerSupplyOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyReset {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyResetRequestBody {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResetType")]
        pub reset_type: Option<crate::resource::ResetType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "ACorDC")]
        ACorDC,
        #[serde(rename = "DC")]
        DC,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Voltage {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_6_3::VoltageActions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingVolts")]
        pub reading_volts: Option<f64>,
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
    pub struct VoltageActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_6_3::VoltageOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VoltageOemActions {}
}
pub mod v1_7_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_7_1::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Power.PowerSupplyReset"
        )]
        pub power_power_supply_reset: Option<crate::power::v1_7_1::PowerSupplyReset>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InputRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputType")]
        pub input_type: Option<crate::power::v1_7_1::InputType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumFrequencyHz")]
        pub maximum_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumVoltage")]
        pub maximum_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumFrequencyHz")]
        pub minimum_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumVoltage")]
        pub minimum_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputWattage")]
        pub output_wattage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum InputType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "DC")]
        DC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LineInputVoltageType {
        #[default]
        #[serde(rename = "AC120V")]
        AC120V,
        #[serde(rename = "AC240V")]
        AC240V,
        #[serde(rename = "AC277V")]
        AC277V,
        #[serde(rename = "ACHighLine")]
        ACHighLine,
        #[serde(rename = "ACLowLine")]
        ACLowLine,
        #[serde(rename = "ACMidLine")]
        ACMidLine,
        #[serde(rename = "ACWideRange")]
        ACWideRange,
        #[serde(rename = "ACandDCWideRange")]
        ACandDCWideRange,
        #[serde(rename = "DC240V")]
        DC240V,
        #[serde(rename = "DC380V")]
        DC380V,
        #[serde(rename = "DCNeg48V")]
        DCNeg48V,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Power {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_7_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerControl")]
        pub power_control: Option<Vec<crate::power::v1_7_1::PowerControl>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerControl@odata.count"
        )]
        pub power_control_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplies")]
        pub power_supplies: Option<Vec<crate::power::v1_7_1::PowerSupply>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerSupplies@odata.count"
        )]
        pub power_supplies_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Redundancy")]
        pub redundancy: Option<Vec<crate::redundancy::Redundancy>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Redundancy@odata.count"
        )]
        pub redundancy_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Voltages")]
        pub voltages: Option<Vec<crate::power::v1_7_1::Voltage>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Voltages@odata.count"
        )]
        pub voltages_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControl {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_7_1::PowerControlActions>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAllocatedWatts"
        )]
        pub power_allocated_watts: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerAvailableWatts"
        )]
        pub power_available_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerConsumedWatts")]
        pub power_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerLimit")]
        pub power_limit: Option<crate::power::v1_7_1::PowerLimit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerMetrics")]
        pub power_metrics: Option<crate::power::v1_7_1::PowerMetric>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerRequestedWatts"
        )]
        pub power_requested_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControlActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_7_1::PowerControlOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControlOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerLimit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CorrectionInMs")]
        pub correction_in_ms: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitException")]
        pub limit_exception: Option<crate::power::v1_7_1::PowerLimitException>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LimitInWatts")]
        pub limit_in_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerLimitException {
        #[default]
        #[serde(rename = "HardPowerOff")]
        HardPowerOff,
        #[serde(rename = "LogEventOnly")]
        LogEventOnly,
        #[serde(rename = "NoAction")]
        NoAction,
        #[serde(rename = "Oem")]
        Oem,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerMetric {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AverageConsumedWatts"
        )]
        pub average_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IntervalInMin")]
        pub interval_in_min: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxConsumedWatts")]
        pub max_consumed_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinConsumedWatts")]
        pub min_consumed_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupply {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_7_1::PowerSupplyActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EfficiencyPercent")]
        pub efficiency_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotPluggable")]
        pub hot_pluggable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::resource::IndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputRanges")]
        pub input_ranges: Option<Vec<crate::power::v1_7_1::InputRange>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastPowerOutputWatts"
        )]
        pub last_power_output_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LineInputVoltage")]
        pub line_input_voltage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LineInputVoltageType"
        )]
        pub line_input_voltage_type: Option<crate::power::v1_7_1::LineInputVoltageType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerInputWatts")]
        pub power_input_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerOutputWatts")]
        pub power_output_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplyType")]
        pub power_supply_type: Option<crate::power::v1_7_1::PowerSupplyType>,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_7_1::PowerSupplyOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyOemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyReset {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupplyResetRequestBody {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResetType")]
        pub reset_type: Option<crate::resource::ResetType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "ACorDC")]
        ACorDC,
        #[serde(rename = "DC")]
        DC,
        #[serde(rename = "Unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Voltage {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power::v1_7_1::VoltageActions>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxReadingRange")]
        pub max_reading_range: Option<f64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinReadingRange")]
        pub min_reading_range: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadingVolts")]
        pub reading_volts: Option<f64>,
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
    pub struct VoltageActions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power::v1_7_1::VoltageOemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VoltageOemActions {}
}
