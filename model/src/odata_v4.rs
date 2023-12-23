use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IdRef {
    #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.id")]
    pub odata_id: Option<String>,
}
