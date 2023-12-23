use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum BreakerStates {
    #[default]
    #[serde(rename = "Normal")]
    Normal,
    #[serde(rename = "Off")]
    Off,
    #[serde(rename = "Tripped")]
    Tripped,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum NominalVoltageType {
    #[default]
    #[serde(rename = "AC100To127V")]
    AC100To127V,
    #[serde(rename = "AC100To240V")]
    AC100To240V,
    #[serde(rename = "AC100To277V")]
    AC100To277V,
    #[serde(rename = "AC120V")]
    AC120V,
    #[serde(rename = "AC200To240V")]
    AC200To240V,
    #[serde(rename = "AC200To277V")]
    AC200To277V,
    #[serde(rename = "AC208V")]
    AC208V,
    #[serde(rename = "AC230V")]
    AC230V,
    #[serde(rename = "AC240AndDC380V")]
    AC240AndDC380V,
    #[serde(rename = "AC240V")]
    AC240V,
    #[serde(rename = "AC277AndDC380V")]
    AC277AndDC380V,
    #[serde(rename = "AC277V")]
    AC277V,
    #[serde(rename = "AC400V")]
    AC400V,
    #[serde(rename = "AC480V")]
    AC480V,
    #[serde(rename = "DC12V")]
    DC12V,
    #[serde(rename = "DC16V")]
    DC16V,
    #[serde(rename = "DC1_8V")]
    DC18V,
    #[serde(rename = "DC240V")]
    DC240V,
    #[serde(rename = "DC380V")]
    DC380V,
    #[serde(rename = "DC3_3V")]
    DC33V,
    #[serde(rename = "DC48V")]
    DC48V,
    #[serde(rename = "DC5V")]
    DC5V,
    #[serde(rename = "DC9V")]
    DC9V,
    #[serde(rename = "DCNeg48V")]
    DCNeg48V,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum PhaseWiringType {
    #[default]
    #[serde(rename = "OneOrTwoPhase3Wire")]
    OneOrTwoPhase3Wire,
    #[serde(rename = "OnePhase3Wire")]
    OnePhase3Wire,
    #[serde(rename = "ThreePhase4Wire")]
    ThreePhase4Wire,
    #[serde(rename = "ThreePhase5Wire")]
    ThreePhase5Wire,
    #[serde(rename = "TwoPhase3Wire")]
    TwoPhase3Wire,
    #[serde(rename = "TwoPhase4Wire")]
    TwoPhase4Wire,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum PlugType {
    #[default]
    #[serde(rename = "California_CS8265")]
    CaliforniaCS8265,
    #[serde(rename = "California_CS8365")]
    CaliforniaCS8365,
    #[serde(rename = "Field_208V_3P4W_60A")]
    Field208V3P4W60A,
    #[serde(rename = "Field_400V_3P5W_32A")]
    Field400V3P5W32A,
    #[serde(rename = "IEC_60309_316P6")]
    IECN60309316P6,
    #[serde(rename = "IEC_60309_332P6")]
    IECN60309332P6,
    #[serde(rename = "IEC_60309_363P6")]
    IECN60309363P6,
    #[serde(rename = "IEC_60309_460P9")]
    IECN60309460P9,
    #[serde(rename = "IEC_60309_516P6")]
    IECN60309516P6,
    #[serde(rename = "IEC_60309_532P6")]
    IECN60309532P6,
    #[serde(rename = "IEC_60309_560P9")]
    IECN60309560P9,
    #[serde(rename = "IEC_60309_563P6")]
    IECN60309563P6,
    #[serde(rename = "IEC_60320_C14")]
    IECN60320C14,
    #[serde(rename = "IEC_60320_C20")]
    IECN60320C20,
    #[serde(rename = "NEMA_5_15P")]
    NEMAN515P,
    #[serde(rename = "NEMA_5_20P")]
    NEMAN520P,
    #[serde(rename = "NEMA_6_15P")]
    NEMAN615P,
    #[serde(rename = "NEMA_6_20P")]
    NEMAN620P,
    #[serde(rename = "NEMA_L14_20P")]
    NEMAL1420P,
    #[serde(rename = "NEMA_L14_30P")]
    NEMAL1430P,
    #[serde(rename = "NEMA_L15_20P")]
    NEMAL1520P,
    #[serde(rename = "NEMA_L15_30P")]
    NEMAL1530P,
    #[serde(rename = "NEMA_L21_20P")]
    NEMAL2120P,
    #[serde(rename = "NEMA_L21_30P")]
    NEMAL2130P,
    #[serde(rename = "NEMA_L22_20P")]
    NEMAL2220P,
    #[serde(rename = "NEMA_L22_30P")]
    NEMAL2230P,
    #[serde(rename = "NEMA_L5_15P")]
    NEMAL515P,
    #[serde(rename = "NEMA_L5_20P")]
    NEMAL520P,
    #[serde(rename = "NEMA_L5_30P")]
    NEMAL530P,
    #[serde(rename = "NEMA_L6_15P")]
    NEMAL615P,
    #[serde(rename = "NEMA_L6_20P")]
    NEMAL620P,
    #[serde(rename = "NEMA_L6_30P")]
    NEMAL630P,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum PowerRestorePolicyTypes {
    #[default]
    #[serde(rename = "AlwaysOff")]
    AlwaysOff,
    #[serde(rename = "AlwaysOn")]
    AlwaysOn,
    #[serde(rename = "LastState")]
    LastState,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum PowerState {
    #[default]
    #[serde(rename = "Off")]
    Off,
    #[serde(rename = "On")]
    On,
    #[serde(rename = "PowerCycle")]
    PowerCycle,
}
pub mod v1_7_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Circuit.BreakerControl"
        )]
        pub circuit_breaker_control: Option<crate::circuit::v1_7_0::BreakerControl>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Circuit.PowerControl"
        )]
        pub circuit_power_control: Option<crate::circuit::v1_7_0::PowerControl>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Circuit.ResetMetrics"
        )]
        pub circuit_reset_metrics: Option<crate::circuit::v1_7_0::ResetMetrics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::circuit::v1_7_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BreakerControl {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BreakerControlRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Circuit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::circuit::v1_7_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BreakerState")]
        pub breaker_state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CircuitType")]
        pub circuit_type: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CriticalCircuit")]
        pub critical_circuit: Option<bool>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ElectricalSourceManagerURI"
        )]
        pub electrical_source_manager_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ElectricalSourceName"
        )]
        pub electrical_source_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergykWh")]
        pub energyk_wh: Option<crate::sensor::SensorEnergykWhExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FrequencyHz")]
        pub frequency_hz: Option<crate::sensor::SensorExcerpt>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::circuit::v1_7_0::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhaseWiringType")]
        pub phase_wiring_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlugType")]
        pub plug_type: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PolyPhaseCurrentAmps"
        )]
        pub poly_phase_current_amps: Option<crate::circuit::v1_7_0::CurrentSensors>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolyPhaseEnergykWh")]
        pub poly_phase_energyk_wh: Option<crate::circuit::v1_7_0::EnergySensors>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PolyPhasePowerWatts"
        )]
        pub poly_phase_power_watts: Option<crate::circuit::v1_7_0::PowerSensors>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolyPhaseVoltage")]
        pub poly_phase_voltage: Option<crate::circuit::v1_7_0::VoltageSensors>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnbalancedCurrentPercent"
        )]
        pub unbalanced_current_percent: Option<crate::sensor::SensorExcerpt>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnbalancedVoltagePercent"
        )]
        pub unbalanced_voltage_percent: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Voltage")]
        pub voltage: Option<crate::sensor::SensorVoltageExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VoltageType")]
        pub voltage_type: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitType {
        #[default]
        #[serde(rename = "Branch")]
        Branch,
        #[serde(rename = "Bus")]
        Bus,
        #[serde(rename = "Feeder")]
        Feeder,
        #[serde(rename = "Mains")]
        Mains,
        #[serde(rename = "Subfeed")]
        Subfeed,
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
    pub struct EnergySensors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToLine2")]
        pub line1_to_line2: Option<crate::sensor::SensorEnergykWhExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToNeutral")]
        pub line1_to_neutral: Option<crate::sensor::SensorEnergykWhExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToLine3")]
        pub line2_to_line3: Option<crate::sensor::SensorEnergykWhExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToNeutral")]
        pub line2_to_neutral: Option<crate::sensor::SensorEnergykWhExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToLine1")]
        pub line3_to_line1: Option<crate::sensor::SensorEnergykWhExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToNeutral")]
        pub line3_to_neutral: Option<crate::sensor::SensorEnergykWhExcerpt>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BranchCircuit")]
        pub branch_circuit: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Outlets")]
        pub outlets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Outlets@odata.count"
        )]
        pub outlets_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerOutlet")]
        pub power_outlet: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourceCircuit")]
        pub source_circuit: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
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
    pub struct PowerSensors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToLine2")]
        pub line1_to_line2: Option<crate::sensor::SensorPowerExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToNeutral")]
        pub line1_to_neutral: Option<crate::sensor::SensorPowerExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToLine3")]
        pub line2_to_line3: Option<crate::sensor::SensorPowerExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToNeutral")]
        pub line2_to_neutral: Option<crate::sensor::SensorPowerExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToLine1")]
        pub line3_to_line1: Option<crate::sensor::SensorPowerExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToNeutral")]
        pub line3_to_neutral: Option<crate::sensor::SensorPowerExcerpt>,
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
