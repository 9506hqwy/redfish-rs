use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum OperatingConfig {
    V010002(crate::operating_config::v1_0_2::OperatingConfig),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::operating_config::v1_0_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct BaseSpeedPrioritySettings {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseSpeedMHz")]
        pub base_speed_mhz: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoreCount")]
        pub core_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoreIDs")]
        pub core_ids: Option<Vec<i64>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OperatingConfig {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::operating_config::v1_0_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BaseSpeedMHz")]
        pub base_speed_mhz: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BaseSpeedPrioritySettings"
        )]
        pub base_speed_priority_settings:
            Option<Vec<crate::operating_config::v1_0_2::BaseSpeedPrioritySettings>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxJunctionTemperatureCelsius"
        )]
        pub max_junction_temperature_celsius: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSpeedMHz")]
        pub max_speed_mhz: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "TDPWatts")]
        pub tdp_watts: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalAvailableCoreCount"
        )]
        pub total_available_core_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TurboProfile")]
        pub turbo_profile: Option<Vec<crate::operating_config::v1_0_2::TurboProfileDatapoint>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TurboProfileDatapoint {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveCoreCount")]
        pub active_core_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxSpeedMHz")]
        pub max_speed_mhz: Option<i64>,
    }
}
