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
pub type Circuit = crate::circuit::v1_8_1::Circuit;
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
pub mod v1_8_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Circuit.BreakerControl"
        )]
        pub circuit_breaker_control: Option<crate::circuit::v1_8_1::BreakerControl>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Circuit.PowerControl"
        )]
        pub circuit_power_control: Option<crate::circuit::v1_8_1::PowerControl>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Circuit.ResetMetrics"
        )]
        pub circuit_reset_metrics: Option<crate::circuit::v1_8_1::ResetMetrics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::circuit::v1_8_1::OemActions>,
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
        pub power_state: Option<crate::circuit::PowerState>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Circuit {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::circuit::v1_8_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BreakerState")]
        pub breaker_state: Option<crate::circuit::v1_8_1::CircuitBreakerState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CircuitType")]
        pub circuit_type: Option<crate::circuit::v1_8_1::CircuitCircuitType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CriticalCircuit")]
        pub critical_circuit: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentAmps")]
        pub current_amps: Option<crate::circuit::v1_8_1::CircuitCurrentAmps>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::circuit::v1_8_1::CircuitDescription>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ElectricalConsumerNames"
        )]
        pub electrical_consumer_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElectricalContext")]
        pub electrical_context: Option<crate::circuit::v1_8_1::CircuitElectricalContext>,
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
        pub energyk_wh: Option<crate::circuit::v1_8_1::CircuitEnergykWh>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FrequencyHz")]
        pub frequency_hz: Option<crate::circuit::v1_8_1::CircuitFrequencyHz>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::circuit::v1_8_1::CircuitIndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::circuit::v1_8_1::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NominalFrequencyHz")]
        pub nominal_frequency_hz: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NominalVoltage")]
        pub nominal_voltage: Option<crate::circuit::v1_8_1::CircuitNominalVoltage>,
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
        pub phase_wiring_type: Option<crate::circuit::v1_8_1::CircuitPhaseWiringType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlugType")]
        pub plug_type: Option<crate::circuit::v1_8_1::CircuitPlugType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PolyPhaseCurrentAmps"
        )]
        pub poly_phase_current_amps: Option<crate::circuit::v1_8_1::CircuitPolyPhaseCurrentAmps>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolyPhaseEnergykWh")]
        pub poly_phase_energyk_wh: Option<crate::circuit::v1_8_1::CircuitPolyPhaseEnergykWh>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PolyPhasePowerWatts"
        )]
        pub poly_phase_power_watts: Option<crate::circuit::v1_8_1::CircuitPolyPhasePowerWatts>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolyPhaseVoltage")]
        pub poly_phase_voltage: Option<crate::circuit::v1_8_1::CircuitPolyPhaseVoltage>,
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
        pub power_load_percent: Option<crate::circuit::v1_8_1::CircuitPowerLoadPercent>,
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
        pub power_restore_policy: Option<crate::circuit::PowerRestorePolicyTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerState")]
        pub power_state: Option<crate::circuit::v1_8_1::CircuitPowerState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerStateInTransition"
        )]
        pub power_state_in_transition: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerWatts")]
        pub power_watts: Option<crate::circuit::v1_8_1::CircuitPowerWatts>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RatedCurrentAmps")]
        pub rated_current_amps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnbalancedCurrentPercent"
        )]
        pub unbalanced_current_percent:
            Option<crate::circuit::v1_8_1::CircuitUnbalancedCurrentPercent>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnbalancedVoltagePercent"
        )]
        pub unbalanced_voltage_percent:
            Option<crate::circuit::v1_8_1::CircuitUnbalancedVoltagePercent>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Voltage")]
        pub voltage: Option<crate::circuit::v1_8_1::CircuitVoltage>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VoltageType")]
        pub voltage_type: Option<crate::circuit::v1_8_1::CircuitVoltageType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitBreakerState {
        V000001(crate::circuit::v1_8_1::CircuitBreakerStateN1),
        CircuitBreakerStates(crate::circuit::BreakerStates),
    }
    impl Default for CircuitBreakerState {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitBreakerStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitCircuitType {
        V010801(crate::circuit::v1_8_1::CircuitType),
        V000001(crate::circuit::v1_8_1::CircuitCircuitTypeN1),
    }
    impl Default for CircuitCircuitType {
        fn default() -> Self {
            Self::V010801(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitCircuitTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitCurrentAmps {
        V000001(crate::circuit::v1_8_1::CircuitCurrentAmpsN1),
        SensorSensorCurrentExcerpt(crate::sensor::v1_9_2::SensorCurrentExcerpt),
    }
    impl Default for CircuitCurrentAmps {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitCurrentAmpsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitDescription {
        V000001(crate::circuit::v1_8_1::CircuitDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for CircuitDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitElectricalContext {
        V000001(crate::circuit::v1_8_1::CircuitElectricalContextN1),
        SensorElectricalContext(crate::sensor::ElectricalContext),
    }
    impl Default for CircuitElectricalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitElectricalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitEnergykWh {
        V000001(crate::circuit::v1_8_1::CircuitEnergykWhN1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_2::SensorEnergykWhExcerpt),
    }
    impl Default for CircuitEnergykWh {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitEnergykWhN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitFrequencyHz {
        V000001(crate::circuit::v1_8_1::CircuitFrequencyHzN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for CircuitFrequencyHz {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitFrequencyHzN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitIndicatorLED {
        V000001(crate::circuit::v1_8_1::CircuitIndicatorLEDN1),
        ResourceIndicatorLED(crate::resource::IndicatorLED),
    }
    impl Default for CircuitIndicatorLED {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitIndicatorLEDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitNominalVoltage {
        V000001(crate::circuit::v1_8_1::CircuitNominalVoltageN1),
        CircuitNominalVoltageType(crate::circuit::NominalVoltageType),
    }
    impl Default for CircuitNominalVoltage {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitNominalVoltageN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitPhaseWiringType {
        V000001(crate::circuit::v1_8_1::CircuitPhaseWiringTypeN1),
        CircuitPhaseWiringType(crate::circuit::PhaseWiringType),
    }
    impl Default for CircuitPhaseWiringType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitPhaseWiringTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitPlugType {
        V000001(crate::circuit::v1_8_1::CircuitPlugTypeN1),
        CircuitPlugType(crate::circuit::PlugType),
    }
    impl Default for CircuitPlugType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitPlugTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitPolyPhaseCurrentAmps {
        V010801(crate::circuit::v1_8_1::CurrentSensors),
        V000001(crate::circuit::v1_8_1::CircuitPolyPhaseCurrentAmpsN1),
    }
    impl Default for CircuitPolyPhaseCurrentAmps {
        fn default() -> Self {
            Self::V010801(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitPolyPhaseCurrentAmpsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitPolyPhaseEnergykWh {
        V010801(crate::circuit::v1_8_1::EnergySensors),
        V000001(crate::circuit::v1_8_1::CircuitPolyPhaseEnergykWhN1),
    }
    impl Default for CircuitPolyPhaseEnergykWh {
        fn default() -> Self {
            Self::V010801(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitPolyPhaseEnergykWhN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitPolyPhasePowerWatts {
        V010801(crate::circuit::v1_8_1::PowerSensors),
        V000001(crate::circuit::v1_8_1::CircuitPolyPhasePowerWattsN1),
    }
    impl Default for CircuitPolyPhasePowerWatts {
        fn default() -> Self {
            Self::V010801(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitPolyPhasePowerWattsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitPolyPhaseVoltage {
        V010801(crate::circuit::v1_8_1::VoltageSensors),
        V000001(crate::circuit::v1_8_1::CircuitPolyPhaseVoltageN1),
    }
    impl Default for CircuitPolyPhaseVoltage {
        fn default() -> Self {
            Self::V010801(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitPolyPhaseVoltageN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitPowerLoadPercent {
        V000001(crate::circuit::v1_8_1::CircuitPowerLoadPercentN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for CircuitPowerLoadPercent {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitPowerLoadPercentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitPowerState {
        V000001(crate::circuit::v1_8_1::CircuitPowerStateN1),
        ResourcePowerState(crate::resource::PowerState),
    }
    impl Default for CircuitPowerState {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitPowerStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitPowerWatts {
        V000001(crate::circuit::v1_8_1::CircuitPowerWattsN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_2::SensorPowerExcerpt),
    }
    impl Default for CircuitPowerWatts {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitPowerWattsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitUnbalancedCurrentPercent {
        V000001(crate::circuit::v1_8_1::CircuitUnbalancedCurrentPercentN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for CircuitUnbalancedCurrentPercent {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitUnbalancedCurrentPercentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitUnbalancedVoltagePercent {
        V000001(crate::circuit::v1_8_1::CircuitUnbalancedVoltagePercentN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for CircuitUnbalancedVoltagePercent {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitUnbalancedVoltagePercentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitVoltage {
        V000001(crate::circuit::v1_8_1::CircuitVoltageN1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_2::SensorVoltageExcerpt),
    }
    impl Default for CircuitVoltage {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitVoltageN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CircuitVoltageType {
        V010801(crate::circuit::v1_8_1::VoltageType),
        V000001(crate::circuit::v1_8_1::CircuitVoltageTypeN1),
    }
    impl Default for CircuitVoltageType {
        fn default() -> Self {
            Self::V010801(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CircuitVoltageTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CurrentSensors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1")]
        pub line1: Option<crate::circuit::v1_8_1::CurrentSensorsLine1>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2")]
        pub line2: Option<crate::circuit::v1_8_1::CurrentSensorsLine2>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3")]
        pub line3: Option<crate::circuit::v1_8_1::CurrentSensorsLine3>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neutral")]
        pub neutral: Option<crate::circuit::v1_8_1::CurrentSensorsNeutral>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CurrentSensorsLine1 {
        V000001(crate::circuit::v1_8_1::CurrentSensorsLine1N1),
        SensorSensorCurrentExcerpt(crate::sensor::v1_9_2::SensorCurrentExcerpt),
    }
    impl Default for CurrentSensorsLine1 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CurrentSensorsLine1N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CurrentSensorsLine2 {
        V000001(crate::circuit::v1_8_1::CurrentSensorsLine2N1),
        SensorSensorCurrentExcerpt(crate::sensor::v1_9_2::SensorCurrentExcerpt),
    }
    impl Default for CurrentSensorsLine2 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CurrentSensorsLine2N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CurrentSensorsLine3 {
        V000001(crate::circuit::v1_8_1::CurrentSensorsLine3N1),
        SensorSensorCurrentExcerpt(crate::sensor::v1_9_2::SensorCurrentExcerpt),
    }
    impl Default for CurrentSensorsLine3 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CurrentSensorsLine3N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CurrentSensorsNeutral {
        V000001(crate::circuit::v1_8_1::CurrentSensorsNeutralN1),
        SensorSensorCurrentExcerpt(crate::sensor::v1_9_2::SensorCurrentExcerpt),
    }
    impl Default for CurrentSensorsNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CurrentSensorsNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EnergySensors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToLine2")]
        pub line1_to_line2: Option<crate::circuit::v1_8_1::EnergySensorsLine1ToLine2>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToNeutral")]
        pub line1_to_neutral: Option<crate::circuit::v1_8_1::EnergySensorsLine1ToNeutral>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToLine3")]
        pub line2_to_line3: Option<crate::circuit::v1_8_1::EnergySensorsLine2ToLine3>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToNeutral")]
        pub line2_to_neutral: Option<crate::circuit::v1_8_1::EnergySensorsLine2ToNeutral>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToLine1")]
        pub line3_to_line1: Option<crate::circuit::v1_8_1::EnergySensorsLine3ToLine1>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToNeutral")]
        pub line3_to_neutral: Option<crate::circuit::v1_8_1::EnergySensorsLine3ToNeutral>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnergySensorsLine1ToLine2 {
        V000001(crate::circuit::v1_8_1::EnergySensorsLine1ToLine2N1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_2::SensorEnergykWhExcerpt),
    }
    impl Default for EnergySensorsLine1ToLine2 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnergySensorsLine1ToLine2N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnergySensorsLine1ToNeutral {
        V000001(crate::circuit::v1_8_1::EnergySensorsLine1ToNeutralN1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_2::SensorEnergykWhExcerpt),
    }
    impl Default for EnergySensorsLine1ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnergySensorsLine1ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnergySensorsLine2ToLine3 {
        V000001(crate::circuit::v1_8_1::EnergySensorsLine2ToLine3N1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_2::SensorEnergykWhExcerpt),
    }
    impl Default for EnergySensorsLine2ToLine3 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnergySensorsLine2ToLine3N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnergySensorsLine2ToNeutral {
        V000001(crate::circuit::v1_8_1::EnergySensorsLine2ToNeutralN1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_2::SensorEnergykWhExcerpt),
    }
    impl Default for EnergySensorsLine2ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnergySensorsLine2ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnergySensorsLine3ToLine1 {
        V000001(crate::circuit::v1_8_1::EnergySensorsLine3ToLine1N1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_2::SensorEnergykWhExcerpt),
    }
    impl Default for EnergySensorsLine3ToLine1 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnergySensorsLine3ToLine1N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum EnergySensorsLine3ToNeutral {
        V000001(crate::circuit::v1_8_1::EnergySensorsLine3ToNeutralN1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_2::SensorEnergykWhExcerpt),
    }
    impl Default for EnergySensorsLine3ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EnergySensorsLine3ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BranchCircuit")]
        pub branch_circuit: Option<crate::circuit::v1_8_1::LinksBranchCircuit>,
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
        pub power_outlet: Option<crate::circuit::v1_8_1::LinksPowerOutlet>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SourceCircuit")]
        pub source_circuit: Option<crate::circuit::v1_8_1::LinksSourceCircuit>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksBranchCircuit {
        V000001(crate::circuit::v1_8_1::LinksBranchCircuitN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksBranchCircuit {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksBranchCircuitN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksPowerOutlet {
        V000001(crate::circuit::v1_8_1::LinksPowerOutletN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksPowerOutlet {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksPowerOutletN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksSourceCircuit {
        V000001(crate::circuit::v1_8_1::LinksSourceCircuitN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksSourceCircuit {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksSourceCircuitN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub power_state: Option<crate::circuit::PowerState>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSensors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToLine2")]
        pub line1_to_line2: Option<crate::circuit::v1_8_1::PowerSensorsLine1ToLine2>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToNeutral")]
        pub line1_to_neutral: Option<crate::circuit::v1_8_1::PowerSensorsLine1ToNeutral>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToLine3")]
        pub line2_to_line3: Option<crate::circuit::v1_8_1::PowerSensorsLine2ToLine3>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToNeutral")]
        pub line2_to_neutral: Option<crate::circuit::v1_8_1::PowerSensorsLine2ToNeutral>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToLine1")]
        pub line3_to_line1: Option<crate::circuit::v1_8_1::PowerSensorsLine3ToLine1>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToNeutral")]
        pub line3_to_neutral: Option<crate::circuit::v1_8_1::PowerSensorsLine3ToNeutral>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSensorsLine1ToLine2 {
        V000001(crate::circuit::v1_8_1::PowerSensorsLine1ToLine2N1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_2::SensorPowerExcerpt),
    }
    impl Default for PowerSensorsLine1ToLine2 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSensorsLine1ToLine2N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSensorsLine1ToNeutral {
        V000001(crate::circuit::v1_8_1::PowerSensorsLine1ToNeutralN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_2::SensorPowerExcerpt),
    }
    impl Default for PowerSensorsLine1ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSensorsLine1ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSensorsLine2ToLine3 {
        V000001(crate::circuit::v1_8_1::PowerSensorsLine2ToLine3N1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_2::SensorPowerExcerpt),
    }
    impl Default for PowerSensorsLine2ToLine3 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSensorsLine2ToLine3N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSensorsLine2ToNeutral {
        V000001(crate::circuit::v1_8_1::PowerSensorsLine2ToNeutralN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_2::SensorPowerExcerpt),
    }
    impl Default for PowerSensorsLine2ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSensorsLine2ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSensorsLine3ToLine1 {
        V000001(crate::circuit::v1_8_1::PowerSensorsLine3ToLine1N1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_2::SensorPowerExcerpt),
    }
    impl Default for PowerSensorsLine3ToLine1 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSensorsLine3ToLine1N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerSensorsLine3ToNeutral {
        V000001(crate::circuit::v1_8_1::PowerSensorsLine3ToNeutralN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_2::SensorPowerExcerpt),
    }
    impl Default for PowerSensorsLine3ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSensorsLine3ToNeutralN1 {
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VoltageSensors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToLine2")]
        pub line1_to_line2: Option<crate::circuit::v1_8_1::VoltageSensorsLine1ToLine2>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToNeutral")]
        pub line1_to_neutral: Option<crate::circuit::v1_8_1::VoltageSensorsLine1ToNeutral>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToLine3")]
        pub line2_to_line3: Option<crate::circuit::v1_8_1::VoltageSensorsLine2ToLine3>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToNeutral")]
        pub line2_to_neutral: Option<crate::circuit::v1_8_1::VoltageSensorsLine2ToNeutral>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToLine1")]
        pub line3_to_line1: Option<crate::circuit::v1_8_1::VoltageSensorsLine3ToLine1>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToNeutral")]
        pub line3_to_neutral: Option<crate::circuit::v1_8_1::VoltageSensorsLine3ToNeutral>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VoltageSensorsLine1ToLine2 {
        V000001(crate::circuit::v1_8_1::VoltageSensorsLine1ToLine2N1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_2::SensorVoltageExcerpt),
    }
    impl Default for VoltageSensorsLine1ToLine2 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VoltageSensorsLine1ToLine2N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VoltageSensorsLine1ToNeutral {
        V000001(crate::circuit::v1_8_1::VoltageSensorsLine1ToNeutralN1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_2::SensorVoltageExcerpt),
    }
    impl Default for VoltageSensorsLine1ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VoltageSensorsLine1ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VoltageSensorsLine2ToLine3 {
        V000001(crate::circuit::v1_8_1::VoltageSensorsLine2ToLine3N1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_2::SensorVoltageExcerpt),
    }
    impl Default for VoltageSensorsLine2ToLine3 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VoltageSensorsLine2ToLine3N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VoltageSensorsLine2ToNeutral {
        V000001(crate::circuit::v1_8_1::VoltageSensorsLine2ToNeutralN1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_2::SensorVoltageExcerpt),
    }
    impl Default for VoltageSensorsLine2ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VoltageSensorsLine2ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VoltageSensorsLine3ToLine1 {
        V000001(crate::circuit::v1_8_1::VoltageSensorsLine3ToLine1N1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_2::SensorVoltageExcerpt),
    }
    impl Default for VoltageSensorsLine3ToLine1 {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VoltageSensorsLine3ToLine1N1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VoltageSensorsLine3ToNeutral {
        V000001(crate::circuit::v1_8_1::VoltageSensorsLine3ToNeutralN1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_2::SensorVoltageExcerpt),
    }
    impl Default for VoltageSensorsLine3ToNeutral {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum VoltageSensorsLine3ToNeutralN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
