use serde::{Deserialize, Serialize};
pub type OutletGroup = crate::outlet_group::v1_2_0::OutletGroup;
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
pub mod v1_1_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::outlet_group::v1_1_2::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#OutletGroup.PowerControl"
        )]
        pub outlet_group_power_control: Option<crate::outlet_group::v1_1_2::PowerControl>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#OutletGroup.ResetMetrics"
        )]
        pub outlet_group_reset_metrics: Option<crate::outlet_group::v1_1_2::ResetMetrics>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Outlets")]
        pub outlets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Outlets@odata.count"
        )]
        pub outlets_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OutletGroup {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::outlet_group::v1_1_2::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CreatedBy")]
        pub created_by: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergykWh")]
        pub energyk_wh: Option<crate::sensor::SensorEnergykWhExcerpt>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::outlet_group::v1_1_2::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerControlLocked")]
        pub power_control_locked: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerCycleDelaySeconds"
        )]
        pub power_cycle_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerEnabled")]
        pub power_enabled: Option<bool>,
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
        pub power_state: Option<crate::resource::PowerState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerStateInTransition"
        )]
        pub power_state_in_transition: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerWatts")]
        pub power_watts: Option<crate::sensor::SensorPowerExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
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
}
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::outlet_group::v1_2_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#OutletGroup.PowerControl"
        )]
        pub outlet_group_power_control: Option<crate::outlet_group::v1_2_0::PowerControl>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#OutletGroup.ResetMetrics"
        )]
        pub outlet_group_reset_metrics: Option<crate::outlet_group::v1_2_0::ResetMetrics>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutletGroups")]
        pub outlet_groups: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OutletGroups@odata.count"
        )]
        pub outlet_groups_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Outlets")]
        pub outlets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Outlets@odata.count"
        )]
        pub outlets_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OutletGroup {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::outlet_group::v1_2_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConfigurationLocked"
        )]
        pub configuration_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CreatedBy")]
        pub created_by: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::outlet_group::v1_2_0::OutletGroupDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnergykWh")]
        pub energyk_wh: Option<crate::outlet_group::v1_2_0::OutletGroupEnergykWh>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::outlet_group::v1_2_0::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutletGroupType")]
        pub outlet_group_type: Option<crate::outlet_group::v1_2_0::OutletGroupType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerControlLocked")]
        pub power_control_locked: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerCycleDelaySeconds"
        )]
        pub power_cycle_delay_seconds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerEnabled")]
        pub power_enabled: Option<bool>,
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
        pub power_state: Option<crate::outlet_group::v1_2_0::OutletGroupPowerState>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerStateInTransition"
        )]
        pub power_state_in_transition: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerWatts")]
        pub power_watts: Option<crate::outlet_group::v1_2_0::OutletGroupPowerWatts>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletGroupDescription {
        V000001(crate::outlet_group::v1_2_0::OutletGroupDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for OutletGroupDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletGroupDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletGroupEnergykWh {
        V000001(crate::outlet_group::v1_2_0::OutletGroupEnergykWhN1),
        SensorSensorEnergykWhExcerpt(crate::sensor::v1_9_0::SensorEnergykWhExcerpt),
    }
    impl Default for OutletGroupEnergykWh {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletGroupEnergykWhN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletGroupPowerState {
        V000001(crate::outlet_group::v1_2_0::OutletGroupPowerStateN1),
        ResourcePowerState(crate::resource::PowerState),
    }
    impl Default for OutletGroupPowerState {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletGroupPowerStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum OutletGroupPowerWatts {
        V000001(crate::outlet_group::v1_2_0::OutletGroupPowerWattsN1),
        SensorSensorPowerExcerpt(crate::sensor::v1_9_0::SensorPowerExcerpt),
    }
    impl Default for OutletGroupPowerWatts {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletGroupPowerWattsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OutletGroupType {
        #[default]
        #[serde(rename = "HardwareDefined")]
        HardwareDefined,
        #[serde(rename = "UserDefined")]
        UserDefined,
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
}
