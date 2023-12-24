use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum UpdateParameters {
    V011103(crate::update_service::v1_11_3::UpdateParameters),
    V011004(crate::update_service::v1_10_4::UpdateParameters),
    V010904(crate::update_service::v1_9_4::UpdateParameters),
    V010807(crate::update_service::v1_8_7::UpdateParameters),
    V010707(crate::update_service::v1_7_7::UpdateParameters),
    V010609(crate::update_service::v1_6_9::UpdateParameters),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum UpdateService {
    V011103(crate::update_service::v1_11_3::UpdateService),
    V011004(crate::update_service::v1_10_4::UpdateService),
    V010904(crate::update_service::v1_9_4::UpdateService),
    V010807(crate::update_service::v1_8_7::UpdateService),
    V010707(crate::update_service::v1_7_7::UpdateService),
    V010609(crate::update_service::v1_6_9::UpdateService),
    V010508(crate::update_service::v1_5_8::UpdateService),
    V010409(crate::update_service::v1_4_9::UpdateService),
    V010309(crate::update_service::v1_3_9::UpdateService),
    V010211(crate::update_service::v1_2_11::UpdateService),
    V010111(crate::update_service::v1_1_11::UpdateService),
    V010010(crate::update_service::v1_0_10::UpdateService),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::update_service::v1_0_10::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.SimpleUpdate"
        )]
        pub update_service_simple_update: Option<crate::update_service::v1_0_10::SimpleUpdate>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdateRequestBody {
        #[serde(rename = "ImageURI")]
        pub image_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::update_service::v1_0_10::TransferProtocolType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferProtocolType {
        #[default]
        #[serde(rename = "CIFS")]
        CIFS,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "HTTPS")]
        HTTPS,
        #[serde(rename = "NSF")]
        NSF,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::update_service::v1_0_10::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareInventory")]
        pub firmware_inventory: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareInventory")]
        pub software_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_1_11 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::update_service::v1_1_11::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.SimpleUpdate"
        )]
        pub update_service_simple_update: Option<crate::update_service::v1_1_11::SimpleUpdate>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdateRequestBody {
        #[serde(rename = "ImageURI")]
        pub image_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::update_service::v1_1_11::TransferProtocolType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferProtocolType {
        #[default]
        #[serde(rename = "CIFS")]
        CIFS,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "HTTPS")]
        HTTPS,
        #[serde(rename = "NSF")]
        NSF,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "SFTP")]
        SFTP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::update_service::v1_1_11::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareInventory")]
        pub firmware_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUri")]
        pub http_push_uri: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareInventory")]
        pub software_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_2_11 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::update_service::v1_2_11::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.SimpleUpdate"
        )]
        pub update_service_simple_update: Option<crate::update_service::v1_2_11::SimpleUpdate>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdateRequestBody {
        #[serde(rename = "ImageURI")]
        pub image_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::update_service::v1_2_11::TransferProtocolType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferProtocolType {
        #[default]
        #[serde(rename = "CIFS")]
        CIFS,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "HTTPS")]
        HTTPS,
        #[serde(rename = "NSF")]
        NSF,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "SFTP")]
        SFTP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::update_service::v1_2_11::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareInventory")]
        pub firmware_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUri")]
        pub http_push_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriTargets")]
        pub http_push_uri_targets: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriTargetsBusy"
        )]
        pub http_push_uri_targets_busy: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareInventory")]
        pub software_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_3_9 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::update_service::v1_3_9::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.SimpleUpdate"
        )]
        pub update_service_simple_update: Option<crate::update_service::v1_3_9::SimpleUpdate>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdateRequestBody {
        #[serde(rename = "ImageURI")]
        pub image_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::update_service::v1_3_9::TransferProtocolType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferProtocolType {
        #[default]
        #[serde(rename = "CIFS")]
        CIFS,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "HTTPS")]
        HTTPS,
        #[serde(rename = "NFS")]
        NFS,
        #[serde(rename = "NSF")]
        NSF,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "SFTP")]
        SFTP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::update_service::v1_3_9::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareInventory")]
        pub firmware_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUri")]
        pub http_push_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriTargets")]
        pub http_push_uri_targets: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriTargetsBusy"
        )]
        pub http_push_uri_targets_busy: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareInventory")]
        pub software_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_4_9 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::update_service::v1_4_9::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.SimpleUpdate"
        )]
        pub update_service_simple_update: Option<crate::update_service::v1_4_9::SimpleUpdate>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ApplyTime {
        #[default]
        #[serde(rename = "AtMaintenanceWindowStart")]
        AtMaintenanceWindowStart,
        #[serde(rename = "Immediate")]
        Immediate,
        #[serde(rename = "InMaintenanceWindowOnReset")]
        InMaintenanceWindowOnReset,
        #[serde(rename = "OnReset")]
        OnReset,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriApplyTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApplyTime")]
        pub apply_time: Option<crate::update_service::v1_4_9::ApplyTime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowDurationInSeconds"
        )]
        pub maintenance_window_duration_in_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowStartTime"
        )]
        pub maintenance_window_start_time: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriOptions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriApplyTime"
        )]
        pub http_push_uri_apply_time: Option<crate::update_service::v1_4_9::HttpPushUriApplyTime>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdateRequestBody {
        #[serde(rename = "ImageURI")]
        pub image_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::update_service::v1_4_9::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferProtocolType {
        #[default]
        #[serde(rename = "CIFS")]
        CIFS,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "HTTPS")]
        HTTPS,
        #[serde(rename = "NFS")]
        NFS,
        #[serde(rename = "NSF")]
        NSF,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "SFTP")]
        SFTP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::update_service::v1_4_9::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareInventory")]
        pub firmware_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUri")]
        pub http_push_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriOptions")]
        pub http_push_uri_options: Option<crate::update_service::v1_4_9::HttpPushUriOptions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriOptionsBusy"
        )]
        pub http_push_uri_options_busy: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriTargets")]
        pub http_push_uri_targets: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriTargetsBusy"
        )]
        pub http_push_uri_targets_busy: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareInventory")]
        pub software_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_5_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::update_service::v1_5_8::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.SimpleUpdate"
        )]
        pub update_service_simple_update: Option<crate::update_service::v1_5_8::SimpleUpdate>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ApplyTime {
        #[default]
        #[serde(rename = "AtMaintenanceWindowStart")]
        AtMaintenanceWindowStart,
        #[serde(rename = "Immediate")]
        Immediate,
        #[serde(rename = "InMaintenanceWindowOnReset")]
        InMaintenanceWindowOnReset,
        #[serde(rename = "OnReset")]
        OnReset,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriApplyTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApplyTime")]
        pub apply_time: Option<crate::update_service::v1_5_8::ApplyTime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowDurationInSeconds"
        )]
        pub maintenance_window_duration_in_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowStartTime"
        )]
        pub maintenance_window_start_time: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriOptions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriApplyTime"
        )]
        pub http_push_uri_apply_time: Option<crate::update_service::v1_5_8::HttpPushUriApplyTime>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdateRequestBody {
        #[serde(rename = "ImageURI")]
        pub image_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::update_service::v1_5_8::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferProtocolType {
        #[default]
        #[serde(rename = "CIFS")]
        CIFS,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "HTTPS")]
        HTTPS,
        #[serde(rename = "NFS")]
        NFS,
        #[serde(rename = "NSF")]
        NSF,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "SFTP")]
        SFTP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::update_service::v1_5_8::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareInventory")]
        pub firmware_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUri")]
        pub http_push_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriOptions")]
        pub http_push_uri_options: Option<crate::update_service::v1_5_8::HttpPushUriOptions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriOptionsBusy"
        )]
        pub http_push_uri_options_busy: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriTargets")]
        pub http_push_uri_targets: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriTargetsBusy"
        )]
        pub http_push_uri_targets_busy: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxImageSizeBytes")]
        pub max_image_size_bytes: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareInventory")]
        pub software_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_6_9 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::update_service::v1_6_9::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.SimpleUpdate"
        )]
        pub update_service_simple_update: Option<crate::update_service::v1_6_9::SimpleUpdate>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ApplyTime {
        #[default]
        #[serde(rename = "AtMaintenanceWindowStart")]
        AtMaintenanceWindowStart,
        #[serde(rename = "Immediate")]
        Immediate,
        #[serde(rename = "InMaintenanceWindowOnReset")]
        InMaintenanceWindowOnReset,
        #[serde(rename = "OnReset")]
        OnReset,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriApplyTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApplyTime")]
        pub apply_time: Option<crate::update_service::v1_6_9::ApplyTime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowDurationInSeconds"
        )]
        pub maintenance_window_duration_in_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowStartTime"
        )]
        pub maintenance_window_start_time: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriOptions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriApplyTime"
        )]
        pub http_push_uri_apply_time: Option<crate::update_service::v1_6_9::HttpPushUriApplyTime>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdateRequestBody {
        #[serde(rename = "ImageURI")]
        pub image_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::update_service::v1_6_9::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferProtocolType {
        #[default]
        #[serde(rename = "CIFS")]
        CIFS,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "HTTPS")]
        HTTPS,
        #[serde(rename = "NFS")]
        NFS,
        #[serde(rename = "NSF")]
        NSF,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "SFTP")]
        SFTP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateParameters {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::update_service::v1_6_9::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareInventory")]
        pub firmware_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUri")]
        pub http_push_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriOptions")]
        pub http_push_uri_options: Option<crate::update_service::v1_6_9::HttpPushUriOptions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriOptionsBusy"
        )]
        pub http_push_uri_options_busy: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriTargets")]
        pub http_push_uri_targets: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriTargetsBusy"
        )]
        pub http_push_uri_targets_busy: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxImageSizeBytes")]
        pub max_image_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MultipartHttpPushUri"
        )]
        pub multipart_http_push_uri: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareInventory")]
        pub software_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_7_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::update_service::v1_7_7::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.SimpleUpdate"
        )]
        pub update_service_simple_update: Option<crate::update_service::v1_7_7::SimpleUpdate>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.StartUpdate"
        )]
        pub update_service_start_update: Option<crate::update_service::v1_7_7::StartUpdate>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ApplyTime {
        #[default]
        #[serde(rename = "AtMaintenanceWindowStart")]
        AtMaintenanceWindowStart,
        #[serde(rename = "Immediate")]
        Immediate,
        #[serde(rename = "InMaintenanceWindowOnReset")]
        InMaintenanceWindowOnReset,
        #[serde(rename = "OnReset")]
        OnReset,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriApplyTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApplyTime")]
        pub apply_time: Option<crate::update_service::v1_7_7::ApplyTime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowDurationInSeconds"
        )]
        pub maintenance_window_duration_in_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowStartTime"
        )]
        pub maintenance_window_start_time: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriOptions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriApplyTime"
        )]
        pub http_push_uri_apply_time: Option<crate::update_service::v1_7_7::HttpPushUriApplyTime>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdateRequestBody {
        #[serde(rename = "ImageURI")]
        pub image_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::update_service::v1_7_7::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StartUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StartUpdateRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferProtocolType {
        #[default]
        #[serde(rename = "CIFS")]
        CIFS,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "HTTPS")]
        HTTPS,
        #[serde(rename = "NFS")]
        NFS,
        #[serde(rename = "NSF")]
        NSF,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "SFTP")]
        SFTP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateParameters {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::update_service::v1_7_7::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareInventory")]
        pub firmware_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUri")]
        pub http_push_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriOptions")]
        pub http_push_uri_options: Option<crate::update_service::v1_7_7::HttpPushUriOptions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriOptionsBusy"
        )]
        pub http_push_uri_options_busy: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriTargets")]
        pub http_push_uri_targets: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriTargetsBusy"
        )]
        pub http_push_uri_targets_busy: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxImageSizeBytes")]
        pub max_image_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MultipartHttpPushUri"
        )]
        pub multipart_http_push_uri: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareInventory")]
        pub software_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_8_7 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::update_service::v1_8_7::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.SimpleUpdate"
        )]
        pub update_service_simple_update: Option<crate::update_service::v1_8_7::SimpleUpdate>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.StartUpdate"
        )]
        pub update_service_start_update: Option<crate::update_service::v1_8_7::StartUpdate>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ApplyTime {
        #[default]
        #[serde(rename = "AtMaintenanceWindowStart")]
        AtMaintenanceWindowStart,
        #[serde(rename = "Immediate")]
        Immediate,
        #[serde(rename = "InMaintenanceWindowOnReset")]
        InMaintenanceWindowOnReset,
        #[serde(rename = "OnReset")]
        OnReset,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriApplyTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApplyTime")]
        pub apply_time: Option<crate::update_service::v1_8_7::ApplyTime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowDurationInSeconds"
        )]
        pub maintenance_window_duration_in_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowStartTime"
        )]
        pub maintenance_window_start_time: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriOptions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriApplyTime"
        )]
        pub http_push_uri_apply_time: Option<crate::update_service::v1_8_7::HttpPushUriApplyTime>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdateRequestBody {
        #[serde(rename = "ImageURI")]
        pub image_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::update_service::v1_8_7::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StartUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StartUpdateRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferProtocolType {
        #[default]
        #[serde(rename = "CIFS")]
        CIFS,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "HTTPS")]
        HTTPS,
        #[serde(rename = "NFS")]
        NFS,
        #[serde(rename = "NSF")]
        NSF,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "SFTP")]
        SFTP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateParameters {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::update_service::v1_8_7::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareInventory")]
        pub firmware_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUri")]
        pub http_push_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriOptions")]
        pub http_push_uri_options: Option<crate::update_service::v1_8_7::HttpPushUriOptions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriOptionsBusy"
        )]
        pub http_push_uri_options_busy: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriTargets")]
        pub http_push_uri_targets: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriTargetsBusy"
        )]
        pub http_push_uri_targets_busy: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxImageSizeBytes")]
        pub max_image_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MultipartHttpPushUri"
        )]
        pub multipart_http_push_uri: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareInventory")]
        pub software_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
}
pub mod v1_9_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::update_service::v1_9_4::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.SimpleUpdate"
        )]
        pub update_service_simple_update: Option<crate::update_service::v1_9_4::SimpleUpdate>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.StartUpdate"
        )]
        pub update_service_start_update: Option<crate::update_service::v1_9_4::StartUpdate>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ApplyTime {
        #[default]
        #[serde(rename = "AtMaintenanceWindowStart")]
        AtMaintenanceWindowStart,
        #[serde(rename = "Immediate")]
        Immediate,
        #[serde(rename = "InMaintenanceWindowOnReset")]
        InMaintenanceWindowOnReset,
        #[serde(rename = "OnReset")]
        OnReset,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriApplyTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApplyTime")]
        pub apply_time: Option<crate::update_service::v1_9_4::ApplyTime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowDurationInSeconds"
        )]
        pub maintenance_window_duration_in_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowStartTime"
        )]
        pub maintenance_window_start_time: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriOptions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriApplyTime"
        )]
        pub http_push_uri_apply_time: Option<crate::update_service::v1_9_4::HttpPushUriApplyTime>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdateRequestBody {
        #[serde(rename = "ImageURI")]
        pub image_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::update_service::v1_9_4::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StartUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StartUpdateRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferProtocolType {
        #[default]
        #[serde(rename = "CIFS")]
        CIFS,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "HTTPS")]
        HTTPS,
        #[serde(rename = "NFS")]
        NFS,
        #[serde(rename = "NSF")]
        NSF,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "SFTP")]
        SFTP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateParameters {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::update_service::v1_9_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareInventory")]
        pub firmware_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUri")]
        pub http_push_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriOptions")]
        pub http_push_uri_options: Option<crate::update_service::v1_9_4::HttpPushUriOptions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriOptionsBusy"
        )]
        pub http_push_uri_options_busy: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriTargets")]
        pub http_push_uri_targets: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriTargetsBusy"
        )]
        pub http_push_uri_targets_busy: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxImageSizeBytes")]
        pub max_image_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MultipartHttpPushUri"
        )]
        pub multipart_http_push_uri: Option<String>,
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
            rename = "RemoteServerCertificates"
        )]
        pub remote_server_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareInventory")]
        pub software_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VerifyRemoteServerCertificate"
        )]
        pub verify_remote_server_certificate: Option<bool>,
    }
}
pub mod v1_10_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::update_service::v1_10_4::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.SimpleUpdate"
        )]
        pub update_service_simple_update: Option<crate::update_service::v1_10_4::SimpleUpdate>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.StartUpdate"
        )]
        pub update_service_start_update: Option<crate::update_service::v1_10_4::StartUpdate>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ApplyTime {
        #[default]
        #[serde(rename = "AtMaintenanceWindowStart")]
        AtMaintenanceWindowStart,
        #[serde(rename = "Immediate")]
        Immediate,
        #[serde(rename = "InMaintenanceWindowOnReset")]
        InMaintenanceWindowOnReset,
        #[serde(rename = "OnReset")]
        OnReset,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriApplyTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApplyTime")]
        pub apply_time: Option<crate::update_service::v1_10_4::ApplyTime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowDurationInSeconds"
        )]
        pub maintenance_window_duration_in_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowStartTime"
        )]
        pub maintenance_window_start_time: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriOptions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriApplyTime"
        )]
        pub http_push_uri_apply_time: Option<crate::update_service::v1_10_4::HttpPushUriApplyTime>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdateRequestBody {
        #[serde(rename = "ImageURI")]
        pub image_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::update_service::v1_10_4::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StartUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StartUpdateRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferProtocolType {
        #[default]
        #[serde(rename = "CIFS")]
        CIFS,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "HTTPS")]
        HTTPS,
        #[serde(rename = "NFS")]
        NFS,
        #[serde(rename = "NSF")]
        NSF,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "SFTP")]
        SFTP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateParameters {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::update_service::v1_10_4::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientCertificates")]
        pub client_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareInventory")]
        pub firmware_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUri")]
        pub http_push_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriOptions")]
        pub http_push_uri_options: Option<crate::update_service::v1_10_4::HttpPushUriOptions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriOptionsBusy"
        )]
        pub http_push_uri_options_busy: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriTargets")]
        pub http_push_uri_targets: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriTargetsBusy"
        )]
        pub http_push_uri_targets_busy: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxImageSizeBytes")]
        pub max_image_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MultipartHttpPushUri"
        )]
        pub multipart_http_push_uri: Option<String>,
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
            rename = "RemoteServerCertificates"
        )]
        pub remote_server_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareInventory")]
        pub software_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VerifyRemoteServerCertificate"
        )]
        pub verify_remote_server_certificate: Option<bool>,
    }
}
pub mod v1_11_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::update_service::v1_11_3::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.SimpleUpdate"
        )]
        pub update_service_simple_update: Option<crate::update_service::v1_11_3::SimpleUpdate>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#UpdateService.StartUpdate"
        )]
        pub update_service_start_update: Option<crate::update_service::v1_11_3::StartUpdate>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ApplyTime {
        #[default]
        #[serde(rename = "AtMaintenanceWindowStart")]
        AtMaintenanceWindowStart,
        #[serde(rename = "Immediate")]
        Immediate,
        #[serde(rename = "InMaintenanceWindowOnReset")]
        InMaintenanceWindowOnReset,
        #[serde(rename = "OnReset")]
        OnReset,
        #[serde(rename = "OnStartUpdateRequest")]
        OnStartUpdateRequest,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriApplyTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ApplyTime")]
        pub apply_time: Option<crate::update_service::v1_11_3::ApplyTime>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowDurationInSeconds"
        )]
        pub maintenance_window_duration_in_seconds: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaintenanceWindowStartTime"
        )]
        pub maintenance_window_start_time: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HttpPushUriOptions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ForceUpdate")]
        pub force_update: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriApplyTime"
        )]
        pub http_push_uri_apply_time: Option<crate::update_service::v1_11_3::HttpPushUriApplyTime>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SimpleUpdateRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ForceUpdate")]
        pub force_update: Option<bool>,
        #[serde(rename = "ImageURI")]
        pub image_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::update_service::v1_11_3::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StartUpdate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StartUpdateRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferProtocolType {
        #[default]
        #[serde(rename = "CIFS")]
        CIFS,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "HTTPS")]
        HTTPS,
        #[serde(rename = "NFS")]
        NFS,
        #[serde(rename = "NSF")]
        NSF,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "SFTP")]
        SFTP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateParameters {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ForceUpdate")]
        pub force_update: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Targets")]
        pub targets: Option<Vec<String>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct UpdateService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::update_service::v1_11_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientCertificates")]
        pub client_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareInventory")]
        pub firmware_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUri")]
        pub http_push_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriOptions")]
        pub http_push_uri_options: Option<crate::update_service::v1_11_3::HttpPushUriOptions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriOptionsBusy"
        )]
        pub http_push_uri_options_busy: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HttpPushUriTargets")]
        pub http_push_uri_targets: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HttpPushUriTargetsBusy"
        )]
        pub http_push_uri_targets_busy: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxImageSizeBytes")]
        pub max_image_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MultipartHttpPushUri"
        )]
        pub multipart_http_push_uri: Option<String>,
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
            rename = "RemoteServerCertificates"
        )]
        pub remote_server_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SoftwareInventory")]
        pub software_inventory: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "VerifyRemoteServerCertificate"
        )]
        pub verify_remote_server_certificate: Option<bool>,
    }
}
