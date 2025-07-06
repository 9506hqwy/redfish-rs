pub type CoolantConnector = crate::coolant_connector::v1_2_0::CoolantConnector;
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::coolant_connector::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CoolantConnector {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::coolant_connector::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Coolant")]
        pub coolant: Option<crate::cooling_loop::Coolant>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CoolantConnectorType"
        )]
        pub coolant_connector_type:
            Option<crate::coolant_connector::v1_2_0::CoolantConnectorCoolantConnectorType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolingLoopName")]
        pub cooling_loop_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolingManagerURI")]
        pub cooling_manager_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaPressureControlkPa"
        )]
        pub delta_pressure_controlk_pa:
            Option<crate::coolant_connector::v1_2_0::CoolantConnectorDeltaPressureControlkPa>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeltaPressurekPa")]
        pub delta_pressurek_pa:
            Option<crate::coolant_connector::v1_2_0::CoolantConnectorDeltaPressurekPa>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaTemperatureCelsius"
        )]
        pub delta_temperature_celsius:
            Option<crate::coolant_connector::v1_2_0::CoolantConnectorDeltaTemperatureCelsius>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaTemperatureControlCelsius"
        )]
        pub delta_temperature_control_celsius: Option<
            crate::coolant_connector::v1_2_0::CoolantConnectorDeltaTemperatureControlCelsius,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::coolant_connector::v1_2_0::CoolantConnectorDescription>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FlowControlLitersPerMinute"
        )]
        pub flow_control_liters_per_minute:
            Option<crate::coolant_connector::v1_2_0::CoolantConnectorFlowControlLitersPerMinute>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FlowLitersPerMinute"
        )]
        pub flow_liters_per_minute:
            Option<crate::coolant_connector::v1_2_0::CoolantConnectorFlowLitersPerMinute>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HeatRemovedkW")]
        pub heat_removedk_w:
            Option<crate::coolant_connector::v1_2_0::CoolantConnectorHeatRemovedkW>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::coolant_connector::v1_2_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
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
            rename = "RatedFlowLitersPerMinute"
        )]
        pub rated_flow_liters_per_minute: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RatedFlowPressurekPa"
        )]
        pub rated_flow_pressurek_pa: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RatedPressurekPa")]
        pub rated_pressurek_pa: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReturnPressurekPa")]
        pub return_pressurek_pa:
            Option<crate::coolant_connector::v1_2_0::CoolantConnectorReturnPressurekPa>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReturnTemperatureCelsius"
        )]
        pub return_temperature_celsius:
            Option<crate::coolant_connector::v1_2_0::CoolantConnectorReturnTemperatureCelsius>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReturnTemperatureControlCelsius"
        )]
        pub return_temperature_control_celsius: Option<
            crate::coolant_connector::v1_2_0::CoolantConnectorReturnTemperatureControlCelsius,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupplyPressurekPa")]
        pub supply_pressurek_pa:
            Option<crate::coolant_connector::v1_2_0::CoolantConnectorSupplyPressurekPa>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyTemperatureCelsius"
        )]
        pub supply_temperature_celsius:
            Option<crate::coolant_connector::v1_2_0::CoolantConnectorSupplyTemperatureCelsius>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyTemperatureControlCelsius"
        )]
        pub supply_temperature_control_celsius: Option<
            crate::coolant_connector::v1_2_0::CoolantConnectorSupplyTemperatureControlCelsius,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ValvePositionControlPercent"
        )]
        pub valve_position_control_percent:
            Option<crate::coolant_connector::v1_2_0::CoolantConnectorValvePositionControlPercent>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ValvePositionPercent"
        )]
        pub valve_position_percent:
            Option<crate::coolant_connector::v1_2_0::CoolantConnectorValvePositionPercent>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorCoolantConnectorType {
        V010200(crate::coolant_connector::v1_2_0::CoolantConnectorType),
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorCoolantConnectorTypeN1),
    }
    impl Default for CoolantConnectorCoolantConnectorType {
        fn default() -> Self {
            Self::V010200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorCoolantConnectorTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorDeltaPressureControlkPa {
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorDeltaPressureControlkPaN1),
        ControlControlSingleLoopExcerpt(crate::control::v1_7_0::ControlSingleLoopExcerpt),
    }
    impl Default for CoolantConnectorDeltaPressureControlkPa {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorDeltaPressureControlkPaN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorDeltaPressurekPa {
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorDeltaPressurekPaN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for CoolantConnectorDeltaPressurekPa {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorDeltaPressurekPaN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorDeltaTemperatureCelsius {
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorDeltaTemperatureCelsiusN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for CoolantConnectorDeltaTemperatureCelsius {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorDeltaTemperatureCelsiusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorDeltaTemperatureControlCelsius {
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorDeltaTemperatureControlCelsiusN1),
        ControlControlSingleLoopExcerpt(crate::control::v1_7_0::ControlSingleLoopExcerpt),
    }
    impl Default for CoolantConnectorDeltaTemperatureControlCelsius {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorDeltaTemperatureControlCelsiusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorDescription {
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for CoolantConnectorDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorFlowControlLitersPerMinute {
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorFlowControlLitersPerMinuteN1),
        ControlControlSingleLoopExcerpt(crate::control::v1_7_0::ControlSingleLoopExcerpt),
    }
    impl Default for CoolantConnectorFlowControlLitersPerMinute {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorFlowControlLitersPerMinuteN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorFlowLitersPerMinute {
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorFlowLitersPerMinuteN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for CoolantConnectorFlowLitersPerMinute {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorFlowLitersPerMinuteN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorHeatRemovedkW {
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorHeatRemovedkWN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for CoolantConnectorHeatRemovedkW {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorHeatRemovedkWN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorReturnPressurekPa {
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorReturnPressurekPaN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for CoolantConnectorReturnPressurekPa {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorReturnPressurekPaN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorReturnTemperatureCelsius {
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorReturnTemperatureCelsiusN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for CoolantConnectorReturnTemperatureCelsius {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorReturnTemperatureCelsiusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorReturnTemperatureControlCelsius {
        V000001(
            crate::coolant_connector::v1_2_0::CoolantConnectorReturnTemperatureControlCelsiusN1,
        ),
        ControlControlSingleLoopExcerpt(crate::control::v1_7_0::ControlSingleLoopExcerpt),
    }
    impl Default for CoolantConnectorReturnTemperatureControlCelsius {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorReturnTemperatureControlCelsiusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorSupplyPressurekPa {
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorSupplyPressurekPaN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for CoolantConnectorSupplyPressurekPa {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorSupplyPressurekPaN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorSupplyTemperatureCelsius {
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorSupplyTemperatureCelsiusN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for CoolantConnectorSupplyTemperatureCelsius {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorSupplyTemperatureCelsiusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorSupplyTemperatureControlCelsius {
        V000001(
            crate::coolant_connector::v1_2_0::CoolantConnectorSupplyTemperatureControlCelsiusN1,
        ),
        ControlControlSingleLoopExcerpt(crate::control::v1_7_0::ControlSingleLoopExcerpt),
    }
    impl Default for CoolantConnectorSupplyTemperatureControlCelsius {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorSupplyTemperatureControlCelsiusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorType {
        #[default]
        #[serde(rename = "Closed")]
        Closed,
        #[serde(rename = "Inline")]
        Inline,
        #[serde(rename = "Pair")]
        Pair,
        #[serde(rename = "Return")]
        Return,
        #[serde(rename = "Supply")]
        Supply,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorValvePositionControlPercent {
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorValvePositionControlPercentN1),
        ControlControlSingleLoopExcerpt(crate::control::v1_7_0::ControlSingleLoopExcerpt),
    }
    impl Default for CoolantConnectorValvePositionControlPercent {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorValvePositionControlPercentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorValvePositionPercent {
        V000001(crate::coolant_connector::v1_2_0::CoolantConnectorValvePositionPercentN1),
        SensorSensorExcerpt(crate::sensor::v1_9_2::SensorExcerpt),
    }
    impl Default for CoolantConnectorValvePositionPercent {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorValvePositionPercentN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedChassis")]
        pub connected_chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedChassis@odata.count"
        )]
        pub connected_chassis_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedCoolingLoop"
        )]
        pub connected_cooling_loop:
            Option<crate::coolant_connector::v1_2_0::LinksConnectedCoolingLoop>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedCoolingUnit"
        )]
        pub connected_cooling_unit:
            Option<crate::coolant_connector::v1_2_0::LinksConnectedCoolingUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksConnectedCoolingLoop {
        V000001(crate::coolant_connector::v1_2_0::LinksConnectedCoolingLoopN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksConnectedCoolingLoop {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksConnectedCoolingLoopN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksConnectedCoolingUnit {
        V000001(crate::coolant_connector::v1_2_0::LinksConnectedCoolingUnitN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for LinksConnectedCoolingUnit {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LinksConnectedCoolingUnitN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
