pub type CoolantConnector = crate::coolant_connector::v1_5_0::CoolantConnector;
pub mod v1_4_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#CoolantConnector.ValveControl"
        )]
        pub coolant_connector_valve_control: Option<crate::coolant_connector::v1_4_0::ValveControl>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::coolant_connector::v1_4_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CoolantConnector {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::coolant_connector::v1_4_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectorGroup")]
        pub connector_group: Option<crate::redundancy::RedundantGroup>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Coolant")]
        pub coolant: Option<crate::cooling_loop::Coolant>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CoolantConnectorType"
        )]
        pub coolant_connector_type:
            Option<crate::coolant_connector::v1_4_0::CoolantConnectorCoolantConnectorType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolingLoopName")]
        pub cooling_loop_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolingManagerURI")]
        pub cooling_manager_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaPressureControlkPa"
        )]
        pub delta_pressure_controlk_pa:
            Option<crate::coolant_connector::v1_4_0::CoolantConnectorDeltaPressureControlkPa>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeltaPressurekPa")]
        pub delta_pressurek_pa:
            Option<crate::coolant_connector::v1_4_0::CoolantConnectorDeltaPressurekPa>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaTemperatureCelsius"
        )]
        pub delta_temperature_celsius:
            Option<crate::coolant_connector::v1_4_0::CoolantConnectorDeltaTemperatureCelsius>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaTemperatureControlCelsius"
        )]
        pub delta_temperature_control_celsius: Option<
            crate::coolant_connector::v1_4_0::CoolantConnectorDeltaTemperatureControlCelsius,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::coolant_connector::v1_4_0::CoolantConnectorDescription>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DewPointProximityCelsius"
        )]
        pub dew_point_proximity_celsius:
            Option<crate::coolant_connector::v1_4_0::CoolantConnectorDewPointProximityCelsius>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FlowControlLitersPerMinute"
        )]
        pub flow_control_liters_per_minute:
            Option<crate::coolant_connector::v1_4_0::CoolantConnectorFlowControlLitersPerMinute>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FlowLitersPerMinute"
        )]
        pub flow_liters_per_minute:
            Option<crate::coolant_connector::v1_4_0::CoolantConnectorFlowLitersPerMinute>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HeatRemovedkW")]
        pub heat_removedk_w:
            Option<crate::coolant_connector::v1_4_0::CoolantConnectorHeatRemovedkW>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::coolant_connector::v1_4_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
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
            Option<crate::coolant_connector::v1_4_0::CoolantConnectorReturnPressurekPa>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReturnTemperatureCelsius"
        )]
        pub return_temperature_celsius:
            Option<crate::coolant_connector::v1_4_0::CoolantConnectorReturnTemperatureCelsius>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReturnTemperatureControlCelsius"
        )]
        pub return_temperature_control_celsius: Option<
            crate::coolant_connector::v1_4_0::CoolantConnectorReturnTemperatureControlCelsius,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupplyPressurekPa")]
        pub supply_pressurek_pa:
            Option<crate::coolant_connector::v1_4_0::CoolantConnectorSupplyPressurekPa>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyTemperatureCelsius"
        )]
        pub supply_temperature_celsius:
            Option<crate::coolant_connector::v1_4_0::CoolantConnectorSupplyTemperatureCelsius>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyTemperatureControlCelsius"
        )]
        pub supply_temperature_control_celsius: Option<
            crate::coolant_connector::v1_4_0::CoolantConnectorSupplyTemperatureControlCelsius,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ValvePositionControlPercent"
        )]
        pub valve_position_control_percent:
            Option<crate::coolant_connector::v1_4_0::CoolantConnectorValvePositionControlPercent>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ValvePositionPercent"
        )]
        pub valve_position_percent:
            Option<crate::coolant_connector::v1_4_0::CoolantConnectorValvePositionPercent>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorCoolantConnectorType {
        V010400(crate::coolant_connector::v1_4_0::CoolantConnectorType),
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorCoolantConnectorTypeN1),
    }
    impl Default for CoolantConnectorCoolantConnectorType {
        fn default() -> Self {
            Self::V010400(Default::default())
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
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorDeltaPressureControlkPaN1),
        ControlControlSingleLoopExcerpt(crate::control::v1_8_1::ControlSingleLoopExcerpt),
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
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorDeltaPressurekPaN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorDeltaTemperatureCelsiusN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorDeltaTemperatureControlCelsiusN1),
        ControlControlSingleLoopExcerpt(crate::control::v1_8_1::ControlSingleLoopExcerpt),
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
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorDescriptionN1),
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
    pub enum CoolantConnectorDewPointProximityCelsius {
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorDewPointProximityCelsiusN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
    }
    impl Default for CoolantConnectorDewPointProximityCelsius {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorDewPointProximityCelsiusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorFlowControlLitersPerMinute {
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorFlowControlLitersPerMinuteN1),
        ControlControlSingleLoopExcerpt(crate::control::v1_8_1::ControlSingleLoopExcerpt),
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
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorFlowLitersPerMinuteN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorHeatRemovedkWN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorReturnPressurekPaN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorReturnTemperatureCelsiusN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
            crate::coolant_connector::v1_4_0::CoolantConnectorReturnTemperatureControlCelsiusN1,
        ),
        ControlControlSingleLoopExcerpt(crate::control::v1_8_1::ControlSingleLoopExcerpt),
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
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorSupplyPressurekPaN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorSupplyTemperatureCelsiusN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
            crate::coolant_connector::v1_4_0::CoolantConnectorSupplyTemperatureControlCelsiusN1,
        ),
        ControlControlSingleLoopExcerpt(crate::control::v1_8_1::ControlSingleLoopExcerpt),
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
        #[serde(rename = "PairGroupMember")]
        PairGroupMember,
        #[serde(rename = "Return")]
        Return,
        #[serde(rename = "ReturnGroupMember")]
        ReturnGroupMember,
        #[serde(rename = "Supply")]
        Supply,
        #[serde(rename = "SupplyGroupMember")]
        SupplyGroupMember,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorValvePositionControlPercent {
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorValvePositionControlPercentN1),
        ControlControlSingleLoopExcerpt(crate::control::v1_8_1::ControlSingleLoopExcerpt),
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
        V000001(crate::coolant_connector::v1_4_0::CoolantConnectorValvePositionPercentN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
            Option<crate::coolant_connector::v1_4_0::LinksConnectedCoolingLoop>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedCoolingUnit"
        )]
        pub connected_cooling_unit:
            Option<crate::coolant_connector::v1_4_0::LinksConnectedCoolingUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksConnectedCoolingLoop {
        V000001(crate::coolant_connector::v1_4_0::LinksConnectedCoolingLoopN1),
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
        V000001(crate::coolant_connector::v1_4_0::LinksConnectedCoolingUnitN1),
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ValveControl {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ValveControlRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ValveState")]
        pub valve_state: Option<crate::coolant_connector::v1_4_0::ValveState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ValveStateReason")]
        pub valve_state_reason: Option<crate::coolant_connector::v1_4_0::ValveStateReason>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ValveState {
        #[default]
        #[serde(rename = "Closed")]
        Closed,
        #[serde(rename = "Open")]
        Open,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ValveStateReason {
        #[default]
        #[serde(rename = "LeakDetected")]
        LeakDetected,
        #[serde(rename = "Normal")]
        Normal,
        #[serde(rename = "NotInUse")]
        NotInUse,
    }
}
pub mod v1_5_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#CoolantConnector.ValveControl"
        )]
        pub coolant_connector_valve_control: Option<crate::coolant_connector::v1_5_0::ValveControl>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::coolant_connector::v1_5_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CoolantConnector {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::coolant_connector::v1_5_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConductivityMSPerCm"
        )]
        pub conductivity_ms_per_cm:
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorConductivityMSPerCm>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectorGroup")]
        pub connector_group: Option<crate::redundancy::RedundantGroup>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Coolant")]
        pub coolant: Option<crate::cooling_loop::Coolant>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CoolantConnectorType"
        )]
        pub coolant_connector_type:
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorCoolantConnectorType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolingLoopName")]
        pub cooling_loop_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolingManagerURI")]
        pub cooling_manager_uri: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaPressureControlkPa"
        )]
        pub delta_pressure_controlk_pa:
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorDeltaPressureControlkPa>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DeltaPressurekPa")]
        pub delta_pressurek_pa:
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorDeltaPressurekPa>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaTemperatureCelsius"
        )]
        pub delta_temperature_celsius:
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorDeltaTemperatureCelsius>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DeltaTemperatureControlCelsius"
        )]
        pub delta_temperature_control_celsius: Option<
            crate::coolant_connector::v1_5_0::CoolantConnectorDeltaTemperatureControlCelsius,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::coolant_connector::v1_5_0::CoolantConnectorDescription>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DewPointProximityCelsius"
        )]
        pub dew_point_proximity_celsius:
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorDewPointProximityCelsius>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FlowControlLitersPerMinute"
        )]
        pub flow_control_liters_per_minute:
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorFlowControlLitersPerMinute>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FlowLitersPerMinute"
        )]
        pub flow_liters_per_minute:
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorFlowLitersPerMinute>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HeatRemovedkW")]
        pub heat_removedk_w:
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorHeatRemovedkW>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::coolant_connector::v1_5_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "pH")]
        pub ph: Option<crate::coolant_connector::v1_5_0::CoolantConnectorPH>,
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
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorReturnPressurekPa>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReturnTemperatureCelsius"
        )]
        pub return_temperature_celsius:
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorReturnTemperatureCelsius>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReturnTemperatureControlCelsius"
        )]
        pub return_temperature_control_celsius: Option<
            crate::coolant_connector::v1_5_0::CoolantConnectorReturnTemperatureControlCelsius,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupplyPressurekPa")]
        pub supply_pressurek_pa:
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorSupplyPressurekPa>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyTemperatureCelsius"
        )]
        pub supply_temperature_celsius:
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorSupplyTemperatureCelsius>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyTemperatureControlCelsius"
        )]
        pub supply_temperature_control_celsius: Option<
            crate::coolant_connector::v1_5_0::CoolantConnectorSupplyTemperatureControlCelsius,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TurbidityNTU")]
        pub turbidity_ntu: Option<crate::coolant_connector::v1_5_0::CoolantConnectorTurbidityNTU>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ValvePositionControlPercent"
        )]
        pub valve_position_control_percent:
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorValvePositionControlPercent>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ValvePositionPercent"
        )]
        pub valve_position_percent:
            Option<crate::coolant_connector::v1_5_0::CoolantConnectorValvePositionPercent>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorConductivityMSPerCm {
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorConductivityMSPerCmN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
    }
    impl Default for CoolantConnectorConductivityMSPerCm {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorConductivityMSPerCmN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorCoolantConnectorType {
        V010500(crate::coolant_connector::v1_5_0::CoolantConnectorType),
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorCoolantConnectorTypeN1),
    }
    impl Default for CoolantConnectorCoolantConnectorType {
        fn default() -> Self {
            Self::V010500(Default::default())
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
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorDeltaPressureControlkPaN1),
        ControlControlSingleLoopExcerpt(crate::control::v1_8_1::ControlSingleLoopExcerpt),
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
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorDeltaPressurekPaN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorDeltaTemperatureCelsiusN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorDeltaTemperatureControlCelsiusN1),
        ControlControlSingleLoopExcerpt(crate::control::v1_8_1::ControlSingleLoopExcerpt),
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
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorDescriptionN1),
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
    pub enum CoolantConnectorDewPointProximityCelsius {
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorDewPointProximityCelsiusN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
    }
    impl Default for CoolantConnectorDewPointProximityCelsius {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorDewPointProximityCelsiusN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorFlowControlLitersPerMinute {
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorFlowControlLitersPerMinuteN1),
        ControlControlSingleLoopExcerpt(crate::control::v1_8_1::ControlSingleLoopExcerpt),
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
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorFlowLitersPerMinuteN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorHeatRemovedkWN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
    pub enum CoolantConnectorPH {
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorPHN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
    }
    impl Default for CoolantConnectorPH {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorPHN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorReturnPressurekPa {
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorReturnPressurekPaN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorReturnTemperatureCelsiusN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
            crate::coolant_connector::v1_5_0::CoolantConnectorReturnTemperatureControlCelsiusN1,
        ),
        ControlControlSingleLoopExcerpt(crate::control::v1_8_1::ControlSingleLoopExcerpt),
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
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorSupplyPressurekPaN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorSupplyTemperatureCelsiusN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
            crate::coolant_connector::v1_5_0::CoolantConnectorSupplyTemperatureControlCelsiusN1,
        ),
        ControlControlSingleLoopExcerpt(crate::control::v1_8_1::ControlSingleLoopExcerpt),
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorTurbidityNTU {
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorTurbidityNTUN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
    }
    impl Default for CoolantConnectorTurbidityNTU {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantConnectorTurbidityNTUN1 {
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
        #[serde(rename = "PairGroupMember")]
        PairGroupMember,
        #[serde(rename = "Return")]
        Return,
        #[serde(rename = "ReturnGroupMember")]
        ReturnGroupMember,
        #[serde(rename = "Supply")]
        Supply,
        #[serde(rename = "SupplyGroupMember")]
        SupplyGroupMember,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CoolantConnectorValvePositionControlPercent {
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorValvePositionControlPercentN1),
        ControlControlSingleLoopExcerpt(crate::control::v1_8_1::ControlSingleLoopExcerpt),
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
        V000001(crate::coolant_connector::v1_5_0::CoolantConnectorValvePositionPercentN1),
        SensorSensorExcerpt(crate::sensor::v1_14_0::SensorExcerpt),
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
            Option<crate::coolant_connector::v1_5_0::LinksConnectedCoolingLoop>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConnectedCoolingUnit"
        )]
        pub connected_cooling_unit:
            Option<crate::coolant_connector::v1_5_0::LinksConnectedCoolingUnit>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LinksConnectedCoolingLoop {
        V000001(crate::coolant_connector::v1_5_0::LinksConnectedCoolingLoopN1),
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
        V000001(crate::coolant_connector::v1_5_0::LinksConnectedCoolingUnitN1),
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ValveControl {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ValveControlRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ValveState")]
        pub valve_state: Option<crate::coolant_connector::v1_5_0::ValveState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ValveStateReason")]
        pub valve_state_reason: Option<crate::coolant_connector::v1_5_0::ValveStateReason>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ValveState {
        #[default]
        #[serde(rename = "Closed")]
        Closed,
        #[serde(rename = "Open")]
        Open,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ValveStateReason {
        #[default]
        #[serde(rename = "LeakDetected")]
        LeakDetected,
        #[serde(rename = "Normal")]
        Normal,
        #[serde(rename = "NotInUse")]
        NotInUse,
    }
}
