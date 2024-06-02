pub type IOConnectivityLineOfService =
    crate::swordfish::io_connectivity_line_of_service::v1_2_1::IOConnectivityLineOfService;
pub mod v1_2_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::io_connectivity_line_of_service::v1_2_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOConnectivityLineOfService { # [serde (skip_serializing_if = "Option::is_none" , rename = "AccessProtocols")] pub access_protocols : Option < Vec < crate :: swordfish :: io_connectivity_line_of_service :: v1_2_1 :: IOConnectivityLineOfServiceAccessProtocols > > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Actions")] pub actions : Option < crate :: swordfish :: io_connectivity_line_of_service :: v1_2_1 :: Actions > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Description")] pub description : Option < crate :: swordfish :: io_connectivity_line_of_service :: v1_2_1 :: IOConnectivityLineOfServiceDescription > , # [serde (rename = "Id")] pub id : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "MaxBytesPerSecond")] pub max_bytes_per_second : Option < i64 > , # [serde (skip_serializing_if = "Option::is_none" , rename = "MaxIOPS")] pub max_iops : Option < i64 > , # [serde (rename = "Name")] pub name : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "@odata.context")] pub odata_context : Option < String > , # [serde (skip_serializing_if = "Option::is_none" , rename = "@odata.etag")] pub odata_etag : Option < String > , # [serde (rename = "@odata.id")] pub odata_id : String , # [serde (rename = "@odata.type")] pub odata_type : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "Oem")] pub oem : Option < crate :: resource :: Oem > }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum IOConnectivityLineOfServiceAccessProtocols {
        V000001 (crate :: swordfish :: io_connectivity_line_of_service :: v1_2_1 :: IOConnectivityLineOfServiceAccessProtocolsN1) , ProtocolProtocol (crate :: protocol :: Protocol) }
    impl Default for IOConnectivityLineOfServiceAccessProtocols {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IOConnectivityLineOfServiceAccessProtocolsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum IOConnectivityLineOfServiceDescription {
        V000001 (crate :: swordfish :: io_connectivity_line_of_service :: v1_2_1 :: IOConnectivityLineOfServiceDescriptionN1) , ResourceDescription (String) }
    impl Default for IOConnectivityLineOfServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IOConnectivityLineOfServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
