pub type Aggregate = crate::aggregate::v1_0_3::Aggregate;
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Aggregate.AddElements"
        )]
        pub aggregate_add_elements: Option<crate::aggregate::v1_0_1::AddElements>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Aggregate.RemoveElements"
        )]
        pub aggregate_remove_elements: Option<crate::aggregate::v1_0_1::RemoveElements>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Aggregate.Reset")]
        pub aggregate_reset: Option<crate::aggregate::v1_0_1::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Aggregate.SetDefaultBootOrder"
        )]
        pub aggregate_set_default_boot_order: Option<crate::aggregate::v1_0_1::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::aggregate::v1_0_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddElements {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddElementsRequestBody {
        #[serde(rename = "Elements")]
        pub elements: Vec<crate::resource::Resource>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Aggregate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::aggregate::v1_0_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Elements")]
        pub elements: Vec<crate::resource::Resource>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElementsCount")]
        pub elements_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Elements@odata.count"
        )]
        pub elements_odata_count: Option<i64>,
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
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveElements {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveElementsRequestBody {
        #[serde(rename = "Elements")]
        pub elements: Vec<crate::resource::Resource>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Reset {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BatchSize")]
        pub batch_size: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DelayBetweenBatchesInSeconds"
        )]
        pub delay_between_batches_in_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResetType")]
        pub reset_type: Option<crate::resource::ResetType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
}
pub mod v1_0_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Aggregate.AddElements"
        )]
        pub aggregate_add_elements: Option<crate::aggregate::v1_0_3::AddElements>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Aggregate.RemoveElements"
        )]
        pub aggregate_remove_elements: Option<crate::aggregate::v1_0_3::RemoveElements>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#Aggregate.Reset")]
        pub aggregate_reset: Option<crate::aggregate::v1_0_3::Reset>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#Aggregate.SetDefaultBootOrder"
        )]
        pub aggregate_set_default_boot_order: Option<crate::aggregate::v1_0_3::SetDefaultBootOrder>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::aggregate::v1_0_3::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddElements {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AddElementsRequestBody {
        #[serde(rename = "Elements")]
        pub elements: Vec<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Aggregate {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::aggregate::v1_0_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::aggregate::v1_0_3::AggregateDescription>,
        #[serde(rename = "Elements")]
        pub elements: Vec<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ElementsCount")]
        pub elements_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Elements@odata.count"
        )]
        pub elements_odata_count: Option<i64>,
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
    pub enum AggregateDescription {
        V000001(crate::aggregate::v1_0_3::AggregateDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for AggregateDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AggregateDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveElements {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct RemoveElementsRequestBody {
        #[serde(rename = "Elements")]
        pub elements: Vec<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Reset {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BatchSize")]
        pub batch_size: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DelayBetweenBatchesInSeconds"
        )]
        pub delay_between_batches_in_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResetType")]
        pub reset_type: Option<crate::resource::ResetType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrder {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SetDefaultBootOrderRequestBody {}
}
