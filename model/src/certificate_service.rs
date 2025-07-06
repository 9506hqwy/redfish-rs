pub type CertificateService = crate::certificate_service::v1_1_0::CertificateService;
pub mod v1_1_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#CertificateService.GenerateCSR"
        )]
        pub certificate_service_generate_csr:
            Option<crate::certificate_service::v1_1_0::GenerateCSR>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#CertificateService.ReplaceCertificate"
        )]
        pub certificate_service_replace_certificate:
            Option<crate::certificate_service::v1_1_0::ReplaceCertificate>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::certificate_service::v1_1_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CertificateService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::certificate_service::v1_1_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CertificateLocations"
        )]
        pub certificate_locations: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::certificate_service::v1_1_0::CertificateServiceDescription>,
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
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum CertificateServiceDescription {
        V000001(crate::certificate_service::v1_1_0::CertificateServiceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for CertificateServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CertificateServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenerateCSR {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenerateCSRRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlternativeNames")]
        pub alternative_names: Option<Vec<String>>,
        #[serde(rename = "CertificateCollection")]
        pub certificate_collection: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ChallengePassword")]
        pub challenge_password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(rename = "CommonName")]
        pub common_name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContactPerson")]
        pub contact_person: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Email")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GivenName")]
        pub given_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Initials")]
        pub initials: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyBitLength")]
        pub key_bit_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyCurveId")]
        pub key_curve_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyPairAlgorithm")]
        pub key_pair_algorithm: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KeyUsage")]
        pub key_usage: Option<Vec<crate::certificate::KeyUsage>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Organization")]
        pub organization: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OrganizationalUnit")]
        pub organizational_unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "State")]
        pub state: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Surname")]
        pub surname: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UnstructuredName")]
        pub unstructured_name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenerateCSRResponse {
        #[serde(rename = "CertificateCollection")]
        pub certificate_collection: crate::odata_v4::IdRef,
        #[serde(rename = "CSRString")]
        pub csr_string: String,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReplaceCertificate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReplaceCertificateRequestBody {
        #[serde(rename = "CertificateString")]
        pub certificate_string: String,
        #[serde(rename = "CertificateType")]
        pub certificate_type: crate::certificate::CertificateType,
        #[serde(rename = "CertificateUri")]
        pub certificate_uri: crate::odata_v4::IdRef,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
    }
}
