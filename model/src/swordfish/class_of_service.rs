pub type ClassOfService = crate::swordfish::class_of_service::v1_2_0::ClassOfService;
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::class_of_service::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClassOfService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::class_of_service::v1_2_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClassOfServiceVersion"
        )]
        pub class_of_service_version: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DataProtectionLinesOfService"
        )]
        pub data_protection_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DataProtectionLinesOfService@odata.count"
        )]
        pub data_protection_lines_of_service_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DataSecurityLinesOfService"
        )]
        pub data_security_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DataSecurityLinesOfService@odata.count"
        )]
        pub data_security_lines_of_service_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DataStorageLinesOfService"
        )]
        pub data_storage_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DataStorageLinesOfService@odata.count"
        )]
        pub data_storage_lines_of_service_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description:
            Option<crate::swordfish::class_of_service::v1_2_0::ClassOfServiceDescription>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifier")]
        pub identifier: Option<crate::resource::Identifier>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IOConnectivityLinesOfService"
        )]
        pub io_connectivity_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IOConnectivityLinesOfService@odata.count"
        )]
        pub io_connectivity_lines_of_service_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IOPerformanceLinesOfService"
        )]
        pub io_performance_lines_of_service: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IOPerformanceLinesOfService@odata.count"
        )]
        pub io_performance_lines_of_service_odata_count: Option<i64>,
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
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ClassOfServiceDescription {
        V000001(crate::swordfish::class_of_service::v1_2_0::ClassOfServiceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ClassOfServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ClassOfServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
