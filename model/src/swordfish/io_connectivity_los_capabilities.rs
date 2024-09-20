pub type IOConnectivityLoSCapabilities =
    crate::swordfish::io_connectivity_los_capabilities::v1_2_0::IOConnectivityLoSCapabilities;
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::swordfish::io_connectivity_los_capabilities::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct IOConnectivityLoSCapabilities { # [serde (skip_serializing_if = "Option::is_none" , rename = "Actions")] pub actions : Option < crate :: swordfish :: io_connectivity_los_capabilities :: v1_2_0 :: Actions > , # [serde (skip_serializing_if = "Option::is_none" , rename = "Description")] pub description : Option < crate :: swordfish :: io_connectivity_los_capabilities :: v1_2_0 :: IOConnectivityLoSCapabilitiesDescription > , # [serde (rename = "Id")] pub id : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "Identifier")] pub identifier : Option < crate :: resource :: Identifier > , # [serde (skip_serializing_if = "Option::is_none" , rename = "MaxSupportedBytesPerSecond")] pub max_supported_bytes_per_second : Option < i64 > , # [serde (skip_serializing_if = "Option::is_none" , rename = "MaxSupportedIOPS")] pub max_supported_iops : Option < i64 > , # [serde (rename = "Name")] pub name : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "@odata.context")] pub odata_context : Option < String > , # [serde (skip_serializing_if = "Option::is_none" , rename = "@odata.etag")] pub odata_etag : Option < String > , # [serde (rename = "@odata.id")] pub odata_id : String , # [serde (rename = "@odata.type")] pub odata_type : String , # [serde (skip_serializing_if = "Option::is_none" , rename = "Oem")] pub oem : Option < crate :: resource :: Oem > , # [serde (skip_serializing_if = "Option::is_none" , rename = "SupportedAccessProtocols")] pub supported_access_protocols : Option < Vec < crate :: swordfish :: io_connectivity_los_capabilities :: v1_2_0 :: IOConnectivityLoSCapabilitiesSupportedAccessProtocols > > , # [serde (skip_serializing_if = "Option::is_none" , rename = "SupportedLinesOfService")] pub supported_lines_of_service : Option < Vec < crate :: swordfish :: io_connectivity_line_of_service :: IOConnectivityLineOfService > > , # [serde (skip_serializing_if = "Option::is_none" , rename = "SupportedLinesOfService@odata.count")] pub supported_lines_of_service_odata_count : Option < i64 > }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum IOConnectivityLoSCapabilitiesDescription {
        V000001 (crate :: swordfish :: io_connectivity_los_capabilities :: v1_2_0 :: IOConnectivityLoSCapabilitiesDescriptionN1) , ResourceDescription (String) }
    impl Default for IOConnectivityLoSCapabilitiesDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IOConnectivityLoSCapabilitiesDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum IOConnectivityLoSCapabilitiesSupportedAccessProtocols {
        V000001 (crate :: swordfish :: io_connectivity_los_capabilities :: v1_2_0 :: IOConnectivityLoSCapabilitiesSupportedAccessProtocolsN1) , ProtocolProtocol (crate :: protocol :: Protocol) }
    impl Default for IOConnectivityLoSCapabilitiesSupportedAccessProtocols {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IOConnectivityLoSCapabilitiesSupportedAccessProtocolsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
