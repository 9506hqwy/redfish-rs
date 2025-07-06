use serde::{Deserialize, Serialize};
pub type Outlet = crate::outlet::v1_4_4::Outlet;
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
pub mod v1_4_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::outlet::v1_4_4::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Outlet.PowerControl"
        )]
        pub outlet_power_control: Option<crate::outlet::v1_4_4::PowerControl>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Outlet.ResetMetrics"
        )]
        pub outlet_reset_metrics: Option<crate::outlet::v1_4_4::ResetMetrics>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CurrentSensors {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1")]
        pub line1: Option<crate::outlet::v1_4_4::CurrentSensorsLine1>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2")]
        pub line2: Option<crate::outlet::v1_4_4::CurrentSensorsLine2>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3")]
        pub line3: Option<crate::outlet::v1_4_4::CurrentSensorsLine3>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neutral")]
        pub neutral: Option<crate::outlet::v1_4_4::CurrentSensorsNeutral>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CurrentSensorsLine1 {
        V000001(crate::outlet::v1_4_4::CurrentSensorsLine1N1),
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
        V000001(crate::outlet::v1_4_4::CurrentSensorsLine2N1),
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
        V000001(crate::outlet::v1_4_4::CurrentSensorsLine3N1),
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
        V000001(crate::outlet::v1_4_4::CurrentSensorsNeutralN1),
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
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BranchCircuit")]
        pub branch_circuit: Option<crate::outlet::v1_4_4::LinksBranchCircuit>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksBranchCircuit {
        V000001(crate::outlet::v1_4_4::LinksBranchCircuitN1),
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Outlet {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::outlet::v1_4_4::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentAmps")]
        pub current_amps: Option<crate::outlet::v1_4_4::OutletCurrentAmps>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::outlet::v1_4_4::OutletDescription>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ElectricalConsumerNames"
        )]
        pub electrical_consumer_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElectricalContext")]
        pub electrical_context: Option<crate::outlet::v1_4_4::OutletElectricalContext>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergykWh")]
        pub energyk_wh: Option<crate::outlet::v1_4_4::OutletEnergykWh>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FrequencyHz")]
        pub frequency_hz: Option<crate::outlet::v1_4_4::OutletFrequencyHz>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IndicatorLED")]
        pub indicator_led: Option<crate::outlet::v1_4_4::OutletIndicatorLED>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::outlet::v1_4_4::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NominalVoltage")]
        pub nominal_voltage: Option<crate::outlet::v1_4_4::OutletNominalVoltage>,
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
        pub outlet_type: Option<crate::outlet::v1_4_4::OutletOutletType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhaseWiringType")]
        pub phase_wiring_type: Option<crate::outlet::v1_4_4::OutletPhaseWiringType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PolyPhaseCurrentAmps"
        )]
        pub poly_phase_current_amps: Option<crate::outlet::v1_4_4::OutletPolyPhaseCurrentAmps>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PolyPhaseVoltage")]
        pub poly_phase_voltage: Option<crate::outlet::v1_4_4::OutletPolyPhaseVoltage>,
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
        pub power_load_percent: Option<crate::outlet::v1_4_4::OutletPowerLoadPercent>,
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
        pub power_state: Option<crate::outlet::v1_4_4::OutletPowerState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerStateInTransition"
        )]
        pub power_state_in_transition: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerWatts")]
        pub power_watts: Option<crate::outlet::v1_4_4::OutletPowerWatts>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RatedCurrentAmps")]
        pub rated_current_amps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Voltage")]
        pub voltage: Option<crate::outlet::v1_4_4::OutletVoltage>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VoltageType")]
        pub voltage_type: Option<crate::outlet::v1_4_4::OutletVoltageType>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletCurrentAmps {
        V000001(crate::outlet::v1_4_4::OutletCurrentAmpsN1),
        SensorSensorCurrentExcerpt(crate::sensor::v1_9_2::SensorCurrentExcerpt),
    }
    impl Default for OutletCurrentAmps {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletCurrentAmpsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletDescription {
        V000001(crate::outlet::v1_4_4::OutletDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for OutletDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletElectricalContext {
        V000001(crate::outlet::v1_4_4::OutletElectricalContextN1),
        SensorElectricalContext(crate::sensor::ElectricalContext),
    }
    impl Default for OutletElectricalContext {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletElectricalContextN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletEnergykWh {
        V000001(crate::outlet::v1_4_4::OutletEnergykWhN1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_2::SensorEnergykWhExcerpt),
    }
    impl Default for OutletEnergykWh {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletEnergykWhN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletFrequencyHz {
        V000001(crate::outlet::v1_4_4::OutletFrequencyHzN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for OutletFrequencyHz {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletFrequencyHzN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletIndicatorLED {
        V000001(crate::outlet::v1_4_4::OutletIndicatorLEDN1),
        ResourceIndicatorLED(crate::resource::IndicatorLED),
    }
    impl Default for OutletIndicatorLED {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletIndicatorLEDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletNominalVoltage {
        V000001(crate::outlet::v1_4_4::OutletNominalVoltageN1),
        CircuitNominalVoltageType(crate::circuit::NominalVoltageType),
    }
    impl Default for OutletNominalVoltage {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletNominalVoltageN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletOutletType {
        V000001(crate::outlet::v1_4_4::OutletOutletTypeN1),
        OutletReceptacleType(crate::outlet::ReceptacleType),
    }
    impl Default for OutletOutletType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletOutletTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletPhaseWiringType {
        V000001(crate::outlet::v1_4_4::OutletPhaseWiringTypeN1),
        CircuitPhaseWiringType(crate::circuit::PhaseWiringType),
    }
    impl Default for OutletPhaseWiringType {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletPhaseWiringTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletPolyPhaseCurrentAmps {
        V010404(crate::outlet::v1_4_4::CurrentSensors),
        V000001(crate::outlet::v1_4_4::OutletPolyPhaseCurrentAmpsN1),
    }
    impl Default for OutletPolyPhaseCurrentAmps {
        fn default() -> Self {
            Self::V010404(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletPolyPhaseCurrentAmpsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletPolyPhaseVoltage {
        V010404(crate::outlet::v1_4_4::VoltageSensors),
        V000001(crate::outlet::v1_4_4::OutletPolyPhaseVoltageN1),
    }
    impl Default for OutletPolyPhaseVoltage {
        fn default() -> Self {
            Self::V010404(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletPolyPhaseVoltageN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletPowerLoadPercent {
        V000001(crate::outlet::v1_4_4::OutletPowerLoadPercentN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for OutletPowerLoadPercent {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletPowerLoadPercentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletPowerState {
        V000001(crate::outlet::v1_4_4::OutletPowerStateN1),
        ResourcePowerState(crate::resource::PowerState),
    }
    impl Default for OutletPowerState {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletPowerStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletPowerWatts {
        V000001(crate::outlet::v1_4_4::OutletPowerWattsN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_2::SensorPowerExcerpt),
    }
    impl Default for OutletPowerWatts {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletPowerWattsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletVoltage {
        V000001(crate::outlet::v1_4_4::OutletVoltageN1),
        SensorSensorVoltageExcerpt(crate::sensor::v1_9_2::SensorVoltageExcerpt),
    }
    impl Default for OutletVoltage {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletVoltageN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletVoltageType {
        V010404(crate::outlet::v1_4_4::VoltageType),
        V000001(crate::outlet::v1_4_4::OutletVoltageTypeN1),
    }
    impl Default for OutletVoltageType {
        fn default() -> Self {
            Self::V010404(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletVoltageTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub power_state: Option<crate::circuit::PowerState>,
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
        pub line1_to_line2: Option<crate::outlet::v1_4_4::VoltageSensorsLine1ToLine2>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line1ToNeutral")]
        pub line1_to_neutral: Option<crate::outlet::v1_4_4::VoltageSensorsLine1ToNeutral>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToLine3")]
        pub line2_to_line3: Option<crate::outlet::v1_4_4::VoltageSensorsLine2ToLine3>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line2ToNeutral")]
        pub line2_to_neutral: Option<crate::outlet::v1_4_4::VoltageSensorsLine2ToNeutral>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToLine1")]
        pub line3_to_line1: Option<crate::outlet::v1_4_4::VoltageSensorsLine3ToLine1>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Line3ToNeutral")]
        pub line3_to_neutral: Option<crate::outlet::v1_4_4::VoltageSensorsLine3ToNeutral>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum VoltageSensorsLine1ToLine2 {
        V000001(crate::outlet::v1_4_4::VoltageSensorsLine1ToLine2N1),
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
        V000001(crate::outlet::v1_4_4::VoltageSensorsLine1ToNeutralN1),
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
        V000001(crate::outlet::v1_4_4::VoltageSensorsLine2ToLine3N1),
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
        V000001(crate::outlet::v1_4_4::VoltageSensorsLine2ToNeutralN1),
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
        V000001(crate::outlet::v1_4_4::VoltageSensorsLine3ToLine1N1),
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
        V000001(crate::outlet::v1_4_4::VoltageSensorsLine3ToNeutralN1),
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
