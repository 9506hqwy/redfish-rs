pub type ThermalEquipment = crate::thermal_equipment::v1_1_2::ThermalEquipment;
pub mod v1_1_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal_equipment::v1_1_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalEquipment {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal_equipment::v1_1_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CDUs")]
        pub cd_us: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CoolingLoopRedundancy"
        )]
        pub cooling_loop_redundancy: Option<Vec<crate::redundancy::RedundantGroup>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolingLoops")]
        pub cooling_loops: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HeatExchangers")]
        pub heat_exchangers: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ImmersionUnits")]
        pub immersion_units: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_1_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::thermal_equipment::v1_1_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ThermalEquipment {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::thermal_equipment::v1_1_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CDUs")]
        pub cd_us: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CoolingLoopRedundancy"
        )]
        pub cooling_loop_redundancy: Option<Vec<crate::redundancy::RedundantGroup>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolingLoops")]
        pub cooling_loops: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::thermal_equipment::v1_1_2::ThermalEquipmentDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HeatExchangers")]
        pub heat_exchangers: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ImmersionUnits")]
        pub immersion_units: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ThermalEquipmentDescription {
        V000001(crate::thermal_equipment::v1_1_2::ThermalEquipmentDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ThermalEquipmentDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ThermalEquipmentDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
