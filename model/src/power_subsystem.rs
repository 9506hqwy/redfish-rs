pub type PowerSubsystem = crate::power_subsystem::v1_2_0::PowerSubsystem;
pub mod v1_1_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power_subsystem::v1_1_4::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerAllocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedWatts")]
        pub allocated_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RequestedWatts")]
        pub requested_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSubsystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power_subsystem::v1_1_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Allocation")]
        pub allocation: Option<crate::power_subsystem::v1_1_4::PowerAllocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Batteries")]
        pub batteries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityWatts")]
        pub capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::power_subsystem::v1_1_4::PowerSubsystemDescription>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplies")]
        pub power_supplies: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerSupplyRedundancy"
        )]
        pub power_supply_redundancy: Option<Vec<crate::redundancy::RedundantGroup>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSubsystemDescription {
        V000001(crate::power_subsystem::v1_1_4::PowerSubsystemDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for PowerSubsystemDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSubsystemDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power_subsystem::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerBrakeLoadPercentSensor"
        )]
        pub power_brake_load_percent_sensor: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerBrakeTemperatureSensor"
        )]
        pub power_brake_temperature_sensor: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerAllocation {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllocatedWatts")]
        pub allocated_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RequestedWatts")]
        pub requested_watts: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerBrake {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Applied")]
        pub applied: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LoadPercentThreshold"
        )]
        pub load_percent_threshold:
            Option<crate::power_subsystem::v1_2_0::PowerBrakeLoadPercentThreshold>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TemperatureThreshold"
        )]
        pub temperature_threshold:
            Option<crate::power_subsystem::v1_2_0::PowerBrakeTemperatureThreshold>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerBrakeLoadPercentThreshold {
        V010200(crate::power_subsystem::v1_2_0::ThresholdLevel),
        V000001(crate::power_subsystem::v1_2_0::PowerBrakeLoadPercentThresholdN1),
    }
    impl Default for PowerBrakeLoadPercentThreshold {
        fn default() -> Self {
            Self::V010200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerBrakeLoadPercentThresholdN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerBrakeReason {
        #[default]
        #[serde(rename = "ExternalSignal")]
        ExternalSignal,
        #[serde(rename = "LoadPercent")]
        LoadPercent,
        #[serde(rename = "LossOfComms")]
        LossOfComms,
        #[serde(rename = "Temperature")]
        Temperature,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerBrakeTemperatureThreshold {
        V010200(crate::power_subsystem::v1_2_0::ThresholdLevel),
        V000001(crate::power_subsystem::v1_2_0::PowerBrakeTemperatureThresholdN1),
    }
    impl Default for PowerBrakeTemperatureThreshold {
        fn default() -> Self {
            Self::V010200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerBrakeTemperatureThresholdN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSubsystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power_subsystem::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Allocation")]
        pub allocation: Option<crate::power_subsystem::v1_2_0::PowerAllocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Batteries")]
        pub batteries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityWatts")]
        pub capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::power_subsystem::v1_2_0::PowerSubsystemDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::power_subsystem::v1_2_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxRampRateWattsPerSecond"
        )]
        pub max_ramp_rate_watts_per_second: Option<crate::control::ControlSingleExcerpt>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerBrake")]
        pub power_brake: Option<crate::power_subsystem::v1_2_0::PowerBrake>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplies")]
        pub power_supplies: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerSupplyRedundancy"
        )]
        pub power_supply_redundancy: Option<Vec<crate::redundancy::RedundantGroup>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSubsystemDescription {
        V000001(crate::power_subsystem::v1_2_0::PowerSubsystemDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for PowerSubsystemDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSubsystemDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThresholdLevel {
        #[default]
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "UpperCaution")]
        UpperCaution,
        #[serde(rename = "UpperCritical")]
        UpperCritical,
    }
}
