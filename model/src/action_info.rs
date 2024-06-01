pub type ActionInfo = crate::action_info::v1_4_1::ActionInfo;
pub type Parameters = crate::action_info::v1_4_1::Parameters;
pub mod v1_4_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ActionInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Parameters")]
        pub parameters: Option<Vec<crate::action_info::v1_4_1::Parameters>>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ParameterTypes {
        #[default]
        #[serde(rename = "Boolean")]
        Boolean,
        #[serde(rename = "Number")]
        Number,
        #[serde(rename = "NumberArray")]
        NumberArray,
        #[serde(rename = "Object")]
        Object,
        #[serde(rename = "ObjectArray")]
        ObjectArray,
        #[serde(rename = "String")]
        String,
        #[serde(rename = "StringArray")]
        StringArray,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Parameters {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableNumbers")]
        pub allowable_numbers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowablePattern")]
        pub allowable_pattern: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllowableValueDescriptions"
        )]
        pub allowable_value_descriptions: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AllowableValues")]
        pub allowable_values: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ArraySizeMaximum")]
        pub array_size_maximum: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ArraySizeMinimum")]
        pub array_size_minimum: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataType")]
        pub data_type: Option<crate::action_info::v1_4_1::ParameterTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaximumValue")]
        pub maximum_value: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MinimumValue")]
        pub minimum_value: Option<f64>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ObjectDataType")]
        pub object_data_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Required")]
        pub required: Option<bool>,
    }
}
