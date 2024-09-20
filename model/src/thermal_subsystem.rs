pub type ThermalSubsystem = crate::thermal_subsystem::v1_3_2::ThermalSubsystem;
pub mod v1_3_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal_subsystem::v1_3_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalSubsystem {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal_subsystem::v1_3_2::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CoolantConnectorRedundancy"
        )]
        pub coolant_connector_redundancy: Option<Vec<crate::redundancy::RedundantGroup>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolantConnectors")]
        pub coolant_connectors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::thermal_subsystem::v1_3_2::ThermalSubsystemDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FanRedundancy")]
        pub fan_redundancy: Option<Vec<crate::redundancy::RedundantGroup>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Fans")]
        pub fans: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Heaters")]
        pub heaters: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LeakDetection")]
        pub leak_detection: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Pumps")]
        pub pumps: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ThermalMetrics")]
        pub thermal_metrics: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ThermalSubsystemDescription {
        V000001(crate::thermal_subsystem::v1_3_2::ThermalSubsystemDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ThermalSubsystemDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThermalSubsystemDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
