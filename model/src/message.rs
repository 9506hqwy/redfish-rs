use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Message {
    V010102(crate::message::v1_1_2::Message),
    V010010(crate::message::v1_0_10::Message),
}
pub mod v1_0_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Message {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Message")]
        pub message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageArgs")]
        pub message_args: Option<Vec<String>>,
        #[serde(rename = "MessageId")]
        pub message_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedProperties")]
        pub related_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Resolution")]
        pub resolution: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
}
pub mod v1_1_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Message {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Message")]
        pub message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageArgs")]
        pub message_args: Option<Vec<String>>,
        #[serde(rename = "MessageId")]
        pub message_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageSeverity")]
        pub message_severity: Option<crate::resource::Health>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedProperties")]
        pub related_properties: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Resolution")]
        pub resolution: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
}
