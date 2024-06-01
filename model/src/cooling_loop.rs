pub type Coolant = crate::cooling_loop::v1_0_2::Coolant;
pub type CoolingLoop = crate::cooling_loop::v1_0_2::CoolingLoop;
pub mod v1_0_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::cooling_loop::v1_0_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Coolant {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditiveName")]
        pub additive_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditivePercent")]
        pub additive_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolantType")]
        pub coolant_type: Option<crate::cooling_loop::v1_0_0::CoolantType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DensityKgPerCubicMeter"
        )]
        pub density_kg_per_cubic_meter: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RatedServiceHours")]
        pub rated_service_hours: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceHours")]
        pub service_hours: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServicedDate")]
        pub serviced_date: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SpecificHeatkJoulesPerKgK"
        )]
        pub specific_heatk_joules_per_kg_k: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantType {
        #[default]
        #[serde(rename = "Dielectric")]
        Dielectric,
        #[serde(rename = "Fluorocarbon")]
        Fluorocarbon,
        #[serde(rename = "Hydrocarbon")]
        Hydrocarbon,
        #[serde(rename = "Water")]
        Water,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CoolingLoop {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::cooling_loop::v1_0_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingEquipmentNames"
        )]
        pub consuming_equipment_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Coolant")]
        pub coolant: Option<crate::cooling_loop::v1_0_0::Coolant>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CoolantLevelPercent"
        )]
        pub coolant_level_percent: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolantLevelStatus")]
        pub coolant_level_status: Option<crate::resource::Health>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolantQuality")]
        pub coolant_quality: Option<crate::resource::Health>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolingManagerURI")]
        pub cooling_manager_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::cooling_loop::v1_0_0::Links>,
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
            rename = "PrimaryCoolantConnectors"
        )]
        pub primary_coolant_connectors: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RatedFlowLitersPerMinute"
        )]
        pub rated_flow_liters_per_minute: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RatedPressurekPa")]
        pub rated_pressurek_pa: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecondaryCoolantConnectors"
        )]
        pub secondary_coolant_connectors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyEquipmentNames"
        )]
        pub supply_equipment_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Facility")]
        pub facility: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::cooling_loop::v1_0_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Coolant {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditiveName")]
        pub additive_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditivePercent")]
        pub additive_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolantType")]
        pub coolant_type: Option<crate::cooling_loop::v1_0_2::CoolantType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DensityKgPerCubicMeter"
        )]
        pub density_kg_per_cubic_meter: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RatedServiceHours")]
        pub rated_service_hours: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceHours")]
        pub service_hours: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServicedDate")]
        pub serviced_date: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SpecificHeatkJoulesPerKgK"
        )]
        pub specific_heatk_joules_per_kg_k: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CoolantType {
        #[default]
        #[serde(rename = "Dielectric")]
        Dielectric,
        #[serde(rename = "Fluorocarbon")]
        Fluorocarbon,
        #[serde(rename = "Hydrocarbon")]
        Hydrocarbon,
        #[serde(rename = "Water")]
        Water,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CoolingLoop {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::cooling_loop::v1_0_2::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsumingEquipmentNames"
        )]
        pub consuming_equipment_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Coolant")]
        pub coolant: Option<crate::cooling_loop::v1_0_2::Coolant>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CoolantLevelPercent"
        )]
        pub coolant_level_percent: Option<crate::sensor::SensorExcerpt>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolantLevelStatus")]
        pub coolant_level_status: Option<crate::resource::Health>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolantQuality")]
        pub coolant_quality: Option<crate::resource::Health>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolingManagerURI")]
        pub cooling_manager_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::cooling_loop::v1_0_2::Links>,
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
            rename = "PrimaryCoolantConnectors"
        )]
        pub primary_coolant_connectors: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RatedFlowLitersPerMinute"
        )]
        pub rated_flow_liters_per_minute: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RatedPressurekPa")]
        pub rated_pressurek_pa: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SecondaryCoolantConnectors"
        )]
        pub secondary_coolant_connectors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupplyEquipmentNames"
        )]
        pub supply_equipment_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Facility")]
        pub facility: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
