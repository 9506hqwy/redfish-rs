pub type Redundancy = crate::redundancy::v1_4_1::Redundancy;
pub type RedundantGroup = crate::redundancy::v1_4_1::RedundantGroup;
pub mod v1_4_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::redundancy::v1_4_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Redundancy {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::redundancy::v1_4_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxNumSupported")]
        pub max_num_supported: Option<i64>,
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "MinNumNeeded")]
        pub min_num_needed: Option<i64>,
        #[serde(rename = "Mode")]
        pub mode: Option<crate::redundancy::v1_4_1::RedundancyMode>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RedundancyEnabled")]
        pub redundancy_enabled: Option<bool>,
        #[serde(rename = "RedundancySet")]
        pub redundancy_set: Vec<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RedundancySet@odata.count"
        )]
        pub redundancy_set_odata_count: Option<i64>,
        #[serde(rename = "Status")]
        pub status: crate::resource::Status,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RedundancyMode {
        #[default]
        #[serde(rename = "Failover")]
        Failover,
        #[serde(rename = "N+m")]
        NM,
        #[serde(rename = "NotRedundant")]
        NotRedundant,
        #[serde(rename = "Sharing")]
        Sharing,
        #[serde(rename = "Sparing")]
        Sparing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RedundancyType {
        #[default]
        #[serde(rename = "Failover")]
        Failover,
        #[serde(rename = "NPlusM")]
        NPlusM,
        #[serde(rename = "NotRedundant")]
        NotRedundant,
        #[serde(rename = "Sharing")]
        Sharing,
        #[serde(rename = "Sparing")]
        Sparing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RedundantGroup {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxSupportedInGroup"
        )]
        pub max_supported_in_group: Option<i64>,
        #[serde(rename = "MinNeededInGroup")]
        pub min_needed_in_group: Option<i64>,
        #[serde(rename = "RedundancyGroup")]
        pub redundancy_group: Vec<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RedundancyGroup@odata.count"
        )]
        pub redundancy_group_odata_count: Option<i64>,
        #[serde(rename = "RedundancyType")]
        pub redundancy_type: Option<crate::redundancy::v1_4_1::RedundancyType>,
        #[serde(rename = "Status")]
        pub status: crate::resource::Status,
    }
}
