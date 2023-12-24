use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum FileShare {
    V010300(crate::swordfish::file_share::v1_3_0::FileShare),
    V010200(crate::swordfish::file_share::v1_2_0::FileShare),
    V010103(crate::swordfish::file_share::v1_1_3::FileShare),
    V010002(crate::swordfish::file_share::v1_0_2::FileShare),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FileShare {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CASupported")]
        pub ca_supported: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultAccessCapabilities"
        )]
        pub default_access_capabilities:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::StorageAccessCapability>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExecuteSupport")]
        pub execute_support: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileSharePath")]
        pub file_share_path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileShareQuotaType")]
        pub file_share_quota_type: Option<crate::swordfish::file_share::v1_0_2::QuotaType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FileShareRemainingQuotaBytes"
        )]
        pub file_share_remaining_quota_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FileShareTotalQuotaBytes"
        )]
        pub file_share_total_quota_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FileSharingProtocols"
        )]
        pub file_sharing_protocols: Option<Vec<crate::swordfish::file_system::FileProtocol>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::file_share::v1_0_2::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RootAccess")]
        pub root_access: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WritePolicy")]
        pub write_policy: Option<crate::swordfish::storage_replica_info::ReplicaUpdateMode>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassOfService")]
        pub class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileSystem")]
        pub file_system: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum QuotaType {
        #[default]
        #[serde(rename = "Hard")]
        Hard,
        #[serde(rename = "Soft")]
        Soft,
    }
}
pub mod v1_1_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::file_share::v1_1_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FileShare {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::file_share::v1_1_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CASupported")]
        pub ca_supported: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultAccessCapabilities"
        )]
        pub default_access_capabilities:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::StorageAccessCapability>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExecuteSupport")]
        pub execute_support: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileSharePath")]
        pub file_share_path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileShareQuotaType")]
        pub file_share_quota_type: Option<crate::swordfish::file_share::v1_1_3::QuotaType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FileShareRemainingQuotaBytes"
        )]
        pub file_share_remaining_quota_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FileShareTotalQuotaBytes"
        )]
        pub file_share_total_quota_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FileSharingProtocols"
        )]
        pub file_sharing_protocols: Option<Vec<crate::swordfish::file_system::FileProtocol>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::file_share::v1_1_3::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
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
            rename = "RemainingCapacityPercent"
        )]
        pub remaining_capacity_percent: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RootAccess")]
        pub root_access: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WritePolicy")]
        pub write_policy: Option<crate::swordfish::storage_replica_info::ReplicaUpdateMode>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassOfService")]
        pub class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileSystem")]
        pub file_system: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum QuotaType {
        #[default]
        #[serde(rename = "Hard")]
        Hard,
        #[serde(rename = "Soft")]
        Soft,
    }
}
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::file_share::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FileShare {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::file_share::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CASupported")]
        pub ca_supported: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultAccessCapabilities"
        )]
        pub default_access_capabilities:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::StorageAccessCapability>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExecuteSupport")]
        pub execute_support: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileSharePath")]
        pub file_share_path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileShareQuotaType")]
        pub file_share_quota_type: Option<crate::swordfish::file_share::v1_2_0::QuotaType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FileShareRemainingQuotaBytes"
        )]
        pub file_share_remaining_quota_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FileShareTotalQuotaBytes"
        )]
        pub file_share_total_quota_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FileSharingProtocols"
        )]
        pub file_sharing_protocols: Option<Vec<crate::swordfish::file_system::FileProtocol>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::file_share::v1_2_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
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
            rename = "RemainingCapacityPercent"
        )]
        pub remaining_capacity_percent: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RootAccess")]
        pub root_access: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WritePolicy")]
        pub write_policy: Option<crate::swordfish::storage_replica_info::ReplicaUpdateMode>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassOfService")]
        pub class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileSystem")]
        pub file_system: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum QuotaType {
        #[default]
        #[serde(rename = "Hard")]
        Hard,
        #[serde(rename = "Soft")]
        Soft,
    }
}
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::file_share::v1_3_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FileShare {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::file_share::v1_3_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CASupported")]
        pub ca_supported: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DefaultAccessCapabilities"
        )]
        pub default_access_capabilities:
            Option<Vec<crate::swordfish::data_storage_los_capabilities::StorageAccessCapability>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EthernetInterfaces")]
        pub ethernet_interfaces: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExecuteSupport")]
        pub execute_support: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileSharePath")]
        pub file_share_path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileShareQuotaType")]
        pub file_share_quota_type: Option<crate::swordfish::file_share::v1_3_0::QuotaType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FileShareRemainingQuotaBytes"
        )]
        pub file_share_remaining_quota_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FileShareTotalQuotaBytes"
        )]
        pub file_share_total_quota_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FileSharingProtocols"
        )]
        pub file_sharing_protocols: Option<Vec<crate::swordfish::file_system::FileProtocol>>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::file_share::v1_3_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LowSpaceWarningThresholdPercents"
        )]
        pub low_space_warning_threshold_percents: Option<Vec<i64>>,
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
            rename = "RemainingCapacityPercent"
        )]
        pub remaining_capacity_percent: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReplicationEnabled")]
        pub replication_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RootAccess")]
        pub root_access: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WritePolicy")]
        pub write_policy: Option<crate::swordfish::storage_replica_info::ReplicaUpdateMode>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClassOfService")]
        pub class_of_service: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileSystem")]
        pub file_system: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum QuotaType {
        #[default]
        #[serde(rename = "Hard")]
        Hard,
        #[serde(rename = "Soft")]
        Soft,
    }
}
