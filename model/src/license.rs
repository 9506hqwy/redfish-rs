use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum License {
    V010101(crate::license::v1_1_1::License),
    V010002(crate::license::v1_0_2::License),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::license::v1_0_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthorizationScope {
        #[default]
        #[serde(rename = "Capacity")]
        Capacity,
        #[serde(rename = "Device")]
        Device,
        #[serde(rename = "Service")]
        Service,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContactInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContactName")]
        pub contact_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAddress")]
        pub email_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct License {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::license::v1_0_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthorizationScope")]
        pub authorization_scope: Option<crate::license::v1_0_2::AuthorizationScope>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Contact")]
        pub contact: Option<crate::license::v1_0_2::ContactInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DownloadURI")]
        pub download_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntitlementId")]
        pub entitlement_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpirationDate")]
        pub expiration_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GracePeriodDays")]
        pub grace_period_days: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InstallDate")]
        pub install_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LicenseInfoURI")]
        pub license_info_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LicenseOrigin")]
        pub license_origin: Option<crate::license::v1_0_2::LicenseOrigin>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LicenseString")]
        pub license_string: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LicenseType")]
        pub license_type: Option<crate::license::v1_0_2::LicenseType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::license::v1_0_2::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxAuthorizedDevices"
        )]
        pub max_authorized_devices: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemainingDuration")]
        pub remaining_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemainingUseCount")]
        pub remaining_use_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Removable")]
        pub removable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LicenseOrigin {
        #[default]
        #[serde(rename = "BuiltIn")]
        BuiltIn,
        #[serde(rename = "Installed")]
        Installed,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LicenseType {
        #[default]
        #[serde(rename = "Production")]
        Production,
        #[serde(rename = "Prototype")]
        Prototype,
        #[serde(rename = "Trial")]
        Trial,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthorizedDevices")]
        pub authorized_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthorizedDevices@odata.count"
        )]
        pub authorized_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_1_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::license::v1_1_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AuthorizationScope {
        #[default]
        #[serde(rename = "Capacity")]
        Capacity,
        #[serde(rename = "Device")]
        Device,
        #[serde(rename = "Service")]
        Service,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContactInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContactName")]
        pub contact_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAddress")]
        pub email_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct License {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::license::v1_1_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthorizationScope")]
        pub authorization_scope: Option<crate::license::v1_1_1::AuthorizationScope>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Contact")]
        pub contact: Option<crate::license::v1_1_1::ContactInfo>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DownloadURI")]
        pub download_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntitlementId")]
        pub entitlement_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExpirationDate")]
        pub expiration_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GracePeriodDays")]
        pub grace_period_days: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InstallDate")]
        pub install_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LicenseInfoURI")]
        pub license_info_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LicenseOrigin")]
        pub license_origin: Option<crate::license::v1_1_1::LicenseOrigin>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LicenseString")]
        pub license_string: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LicenseType")]
        pub license_type: Option<crate::license::v1_1_1::LicenseType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::license::v1_1_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxAuthorizedDevices"
        )]
        pub max_authorized_devices: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemainingDuration")]
        pub remaining_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RemainingUseCount")]
        pub remaining_use_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Removable")]
        pub removable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LicenseOrigin {
        #[default]
        #[serde(rename = "BuiltIn")]
        BuiltIn,
        #[serde(rename = "Installed")]
        Installed,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LicenseType {
        #[default]
        #[serde(rename = "Production")]
        Production,
        #[serde(rename = "Prototype")]
        Prototype,
        #[serde(rename = "Trial")]
        Trial,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AuthorizedDevices")]
        pub authorized_devices: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthorizedDevices@odata.count"
        )]
        pub authorized_devices_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetServices")]
        pub target_services: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TargetServices@odata.count"
        )]
        pub target_services_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
