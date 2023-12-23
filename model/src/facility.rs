pub mod v1_4_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::facility::v1_4_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Facility {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::facility::v1_4_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AmbientMetrics")]
        pub ambient_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "FacilityType")]
        pub facility_type: crate::facility::v1_4_0::FacilityType,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::facility::v1_4_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerDomains")]
        pub power_domains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FacilityType {
        #[default]
        #[serde(rename = "Building")]
        Building,
        #[serde(rename = "Floor")]
        Floor,
        #[serde(rename = "Room")]
        Room,
        #[serde(rename = "Site")]
        Site,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CDUs")]
        pub cd_us: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CDUs@odata.count")]
        pub cd_us_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ContainedByFacility"
        )]
        pub contained_by_facility: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContainsChassis")]
        pub contains_chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ContainsChassis@odata.count"
        )]
        pub contains_chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContainsFacilities")]
        pub contains_facilities: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ContainsFacilities@odata.count"
        )]
        pub contains_facilities_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CoolingLoops")]
        pub cooling_loops: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CoolingLoops@odata.count"
        )]
        pub cooling_loops_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElectricalBuses")]
        pub electrical_buses: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ElectricalBuses@odata.count"
        )]
        pub electrical_buses_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FloorPDUs")]
        pub floor_pd_us: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FloorPDUs@odata.count"
        )]
        pub floor_pd_us_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ImmersionUnits")]
        pub immersion_units: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ImmersionUnits@odata.count"
        )]
        pub immersion_units_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerShelves")]
        pub power_shelves: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerShelves@odata.count"
        )]
        pub power_shelves_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackPDUs")]
        pub rack_pd_us: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RackPDUs@odata.count"
        )]
        pub rack_pd_us_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Switchgear")]
        pub switchgear: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Switchgear@odata.count"
        )]
        pub switchgear_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferSwitches")]
        pub transfer_switches: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransferSwitches@odata.count"
        )]
        pub transfer_switches_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
