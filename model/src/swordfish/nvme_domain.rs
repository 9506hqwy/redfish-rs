pub type NVMeDomain = crate::swordfish::nvme_domain::v1_2_0::NVMeDomain;
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::nvme_domain::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DomainContents {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Controllers")]
        pub controllers: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Controllers@odata.count"
        )]
        pub controllers_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Namespaces")]
        pub namespaces: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Namespaces@odata.count"
        )]
        pub namespaces_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssociatedDomains")]
        pub associated_domains: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AssociatedDomains@odata.count"
        )]
        pub associated_domains_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeDomain {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::swordfish::nvme_domain::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ANAGroupId")]
        pub ana_group_id: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AvailableFirmwareImages"
        )]
        pub available_firmware_images: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AvailableFirmwareImages@odata.count"
        )]
        pub available_firmware_images_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::swordfish::nvme_domain::v1_2_0::NVMeDomainDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DomainContents")]
        pub domain_contents:
            Option<crate::swordfish::nvme_domain::v1_2_0::NVMeDomainDomainContents>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DomainMembers")]
        pub domain_members: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DomainMembers@odata.count"
        )]
        pub domain_members_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareImages")]
        pub firmware_images: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirmwareImages@odata.count"
        )]
        pub firmware_images_odata_count: Option<i64>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::swordfish::nvme_domain::v1_2_0::Links>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxNamespacesSupportedPerController"
        )]
        pub max_namespaces_supported_per_controller: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaximumCapacityPerEnduranceGroupBytes"
        )]
        pub maximum_capacity_per_endurance_group_bytes: Option<i64>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TotalDomainCapacityBytes"
        )]
        pub total_domain_capacity_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnallocatedDomainCapacityBytes"
        )]
        pub unallocated_domain_capacity_bytes: Option<i64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeDomainDescription {
        V000001(crate::swordfish::nvme_domain::v1_2_0::NVMeDomainDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for NVMeDomainDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeDomainDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeDomainDomainContents {
        V010200(crate::swordfish::nvme_domain::v1_2_0::DomainContents),
        V000001(crate::swordfish::nvme_domain::v1_2_0::NVMeDomainDomainContentsN1),
    }
    impl Default for NVMeDomainDomainContents {
        fn default() -> Self {
            Self::V010200(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeDomainDomainContentsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
