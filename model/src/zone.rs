pub mod v1_6_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::zone::v1_6_1::OemActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Zone.AddEndpoint")]
        pub zone_add_endpoint: Option<crate::zone::v1_6_1::AddEndpoint>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Zone.RemoveEndpoint"
        )]
        pub zone_remove_endpoint: Option<crate::zone::v1_6_1::RemoveEndpoint>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddEndpoint {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddEndpointRequestBody {
        #[serde(rename = "Endpoint")]
        pub endpoint: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointETag")]
        pub endpoint_etag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ZoneETag")]
        pub zone_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ExternalAccessibility {
        #[default]
        #[serde(rename = "GloballyAccessible")]
        GloballyAccessible,
        #[serde(rename = "NoInternalRouting")]
        NoInternalRouting,
        #[serde(rename = "NonZonedAccessible")]
        NonZonedAccessible,
        #[serde(rename = "ZoneOnly")]
        ZoneOnly,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressPools")]
        pub address_pools: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AddressPools@odata.count"
        )]
        pub address_pools_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContainedByZones")]
        pub contained_by_zones: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ContainedByZones@odata.count"
        )]
        pub contained_by_zones_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContainsZones")]
        pub contains_zones: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ContainsZones@odata.count"
        )]
        pub contains_zones_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InvolvedSwitches")]
        pub involved_switches: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InvolvedSwitches@odata.count"
        )]
        pub involved_switches_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResourceBlocks")]
        pub resource_blocks: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ResourceBlocks@odata.count"
        )]
        pub resource_blocks_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveEndpoint {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveEndpointRequestBody {
        #[serde(rename = "Endpoint")]
        pub endpoint: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndpointETag")]
        pub endpoint_etag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ZoneETag")]
        pub zone_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Zone {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::zone::v1_6_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultRoutingEnabled"
        )]
        pub default_routing_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ExternalAccessibility"
        )]
        pub external_accessibility: Option<crate::zone::v1_6_1::ExternalAccessibility>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::zone::v1_6_1::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ZoneType")]
        pub zone_type: Option<crate::zone::v1_6_1::ZoneType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ZoneType {
        #[default]
        #[serde(rename = "Default")]
        Default,
        #[serde(rename = "ZoneOfEndpoints")]
        ZoneOfEndpoints,
        #[serde(rename = "ZoneOfResourceBlocks")]
        ZoneOfResourceBlocks,
        #[serde(rename = "ZoneOfZones")]
        ZoneOfZones,
    }
}
