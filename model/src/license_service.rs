pub type LicenseService = crate::license_service::v1_1_0::LicenseService;
pub mod v1_1_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#LicenseService.Install"
        )]
        pub license_service_install: Option<crate::license_service::v1_1_0::Install>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::license_service::v1_1_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Install {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InstallRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthorizedDevices")]
        pub authorized_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(rename = "LicenseFileURI")]
        pub license_file_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetServices")]
        pub target_services: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::license_service::v1_1_0::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LicenseService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::license_service::v1_1_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LicenseExpirationWarningDays"
        )]
        pub license_expiration_warning_days: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Licenses")]
        pub licenses: Option<crate::odata_v4::IdRef>,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
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
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "SFTP")]
        SFTP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
}
