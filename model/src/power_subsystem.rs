pub type PowerSubsystem = crate::power_subsystem::v1_1_2::PowerSubsystem;
pub mod v1_1_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power_subsystem::v1_1_2::OemActions>,
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
        pub actions: Option<crate::power_subsystem::v1_1_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Allocation")]
        pub allocation: Option<crate::power_subsystem::v1_1_2::PowerAllocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Batteries")]
        pub batteries: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityWatts")]
        pub capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::power_subsystem::v1_1_2::PowerSubsystemDescription>,
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
        V000001(crate::power_subsystem::v1_1_2::PowerSubsystemDescriptionN1),
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
