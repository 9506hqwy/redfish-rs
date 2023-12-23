use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ReceptacleType {
    #[default]
    #[serde(rename = "BS_1363_Type_G")]
    BSN1363TypeG,
    #[serde(rename = "BusConnection")]
    BusConnection,
    #[serde(rename = "CEE_7_Type_E")]
    CEEN7TypeE,
    #[serde(rename = "CEE_7_Type_F")]
    CEEN7TypeF,
    #[serde(rename = "IEC_60320_C13")]
    IECN60320C13,
    #[serde(rename = "IEC_60320_C19")]
    IECN60320C19,
    #[serde(rename = "NEMA_5_15R")]
    NEMAN515R,
    #[serde(rename = "NEMA_5_20R")]
    NEMAN520R,
    #[serde(rename = "NEMA_L5_20R")]
    NEMAL520R,
    #[serde(rename = "NEMA_L5_30R")]
    NEMAL530R,
    #[serde(rename = "NEMA_L6_20R")]
    NEMAL620R,
    #[serde(rename = "NEMA_L6_30R")]
    NEMAL630R,
    #[serde(rename = "SEV_1011_TYPE_12")]
    SEVN1011TYPEN12,
    #[serde(rename = "SEV_1011_TYPE_23")]
    SEVN1011TYPEN23,
}
pub mod v1_4_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::outlet::v1_4_1::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Outlet.PowerControl"
        )]
        pub outlet_power_control: Option<crate::outlet::v1_4_1::PowerControl>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Outlet.ResetMetrics"
        )]
        pub outlet_reset_metrics: Option<crate::outlet::v1_4_1::ResetMetrics>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CurrentSensors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1")]
        pub line1: Option<crate::sensor::SensorCurrentExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2")]
        pub line2: Option<crate::sensor::SensorCurrentExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3")]
        pub line3: Option<crate::sensor::SensorCurrentExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neutral")]
        pub neutral: Option<crate::sensor::SensorCurrentExcerpt>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BranchCircuit")]
        pub branch_circuit: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DistributionCircuits"
        )]
        pub distribution_circuits: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DistributionCircuits@odata.count"
        )]
        pub distribution_circuits_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplies")]
        pub power_supplies: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerSupplies@odata.count"
        )]
        pub power_supplies_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Outlet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::outlet::v1_4_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentAmps")]
        pub current_amps: Option<crate::sensor::SensorCurrentExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ElectricalConsumerNames"
        )]
        pub electrical_consumer_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElectricalContext")]
        pub electrical_context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergykWh")]
        pub energyk_wh: Option<crate::sensor::SensorEnergykWhExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FrequencyHz")]
        pub frequency_hz: Option<crate::sensor::SensorExcerpt>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::outlet::v1_4_1::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NominalVoltage")]
        pub nominal_voltage: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutletType")]
        pub outlet_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhaseWiringType")]
        pub phase_wiring_type: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PolyPhaseCurrentAmps"
        )]
        pub poly_phase_current_amps: Option<crate::outlet::v1_4_1::CurrentSensors>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolyPhaseVoltage")]
        pub poly_phase_voltage: Option<crate::outlet::v1_4_1::VoltageSensors>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerControlLocked")]
        pub power_control_locked: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerCycleDelaySeconds"
        )]
        pub power_cycle_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerEnabled")]
        pub power_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerLoadPercent")]
        pub power_load_percent: Option<crate::sensor::SensorExcerpt>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOffDelaySeconds"
        )]
        pub power_off_delay_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOnDelaySeconds"
        )]
        pub power_on_delay_seconds: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerRestoreDelaySeconds"
        )]
        pub power_restore_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerRestorePolicy")]
        pub power_restore_policy: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerStateInTransition"
        )]
        pub power_state_in_transition: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerWatts")]
        pub power_watts: Option<crate::sensor::SensorPowerExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RatedCurrentAmps")]
        pub rated_current_amps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Voltage")]
        pub voltage: Option<crate::sensor::SensorVoltageExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VoltageType")]
        pub voltage_type: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControl {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerControlRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<String>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VoltageSensors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToLine2")]
        pub line1_to_line2: Option<crate::sensor::SensorVoltageExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToNeutral")]
        pub line1_to_neutral: Option<crate::sensor::SensorVoltageExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToLine3")]
        pub line2_to_line3: Option<crate::sensor::SensorVoltageExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToNeutral")]
        pub line2_to_neutral: Option<crate::sensor::SensorVoltageExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToLine1")]
        pub line3_to_line1: Option<crate::sensor::SensorVoltageExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToNeutral")]
        pub line3_to_neutral: Option<crate::sensor::SensorVoltageExcerpt>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VoltageType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "DC")]
        DC,
    }
}
