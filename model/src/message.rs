pub type Message = crate::message::v1_3_0::Message;
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
pub mod v1_2_1 {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResolutionSteps")]
        pub resolution_steps: Option<Vec<crate::resolution_step::ResolutionStep>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
    }
}
pub mod v1_3_0 {
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResolutionSteps")]
        pub resolution_steps: Option<Vec<crate::resolution_step::ResolutionStep>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UserAuthenticationSource"
        )]
        pub user_authentication_source: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
}
