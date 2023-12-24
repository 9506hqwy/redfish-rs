use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IOStatistics {
    V010004(crate::swordfish::io_statistics::v1_0_4::IOStatistics),
}
pub mod v1_0_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOStatistics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonIORequestTime")]
        pub non_io_request_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonIORequests")]
        pub non_io_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadHitIORequests")]
        pub read_hit_io_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadIOKiBytes")]
        pub read_io_kibytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadIORequestTime")]
        pub read_io_request_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadIORequests")]
        pub read_io_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteHitIORequests")]
        pub write_hit_io_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteIOKiBytes")]
        pub write_io_kibytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteIORequestTime")]
        pub write_io_request_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteIORequests")]
        pub write_io_requests: Option<i64>,
    }
}
pub mod v1_0_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOStatistics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonIORequestTime")]
        pub non_io_request_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NonIORequests")]
        pub non_io_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadHitIORequests")]
        pub read_hit_io_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadIOKiBytes")]
        pub read_io_kibytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadIORequestTime")]
        pub read_io_request_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReadIORequests")]
        pub read_io_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteHitIORequests")]
        pub write_hit_io_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteIOKiBytes")]
        pub write_io_kibytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteIORequestTime")]
        pub write_io_request_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteIORequests")]
        pub write_io_requests: Option<i64>,
    }
}
