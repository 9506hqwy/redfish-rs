pub type SerialInterface = crate::serial_interface::v1_2_0::SerialInterface;
pub mod v1_1_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::serial_interface::v1_1_8::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BitRate {
        #[default]
        #[serde(rename = "115200")]
        N115200,
        #[serde(rename = "1200")]
        N1200,
        #[serde(rename = "19200")]
        N19200,
        #[serde(rename = "230400")]
        N230400,
        #[serde(rename = "2400")]
        N2400,
        #[serde(rename = "38400")]
        N38400,
        #[serde(rename = "4800")]
        N4800,
        #[serde(rename = "57600")]
        N57600,
        #[serde(rename = "9600")]
        N9600,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectorType {
        #[default]
        #[serde(rename = "DB25 Female")]
        DB25Female,
        #[serde(rename = "DB25 Male")]
        DB25Male,
        #[serde(rename = "DB9 Female")]
        DB9Female,
        #[serde(rename = "DB9 Male")]
        DB9Male,
        #[serde(rename = "RJ11")]
        RJ11,
        #[serde(rename = "RJ45")]
        RJ45,
        #[serde(rename = "USB")]
        USB,
        #[serde(rename = "mUSB")]
        MUSB,
        #[serde(rename = "uUSB")]
        UUSB,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataBits {
        #[default]
        #[serde(rename = "5")]
        N5,
        #[serde(rename = "6")]
        N6,
        #[serde(rename = "7")]
        N7,
        #[serde(rename = "8")]
        N8,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FlowControl {
        #[default]
        #[serde(rename = "Hardware")]
        Hardware,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "Software")]
        Software,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Parity {
        #[default]
        #[serde(rename = "Even")]
        Even,
        #[serde(rename = "Mark")]
        Mark,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "Odd")]
        Odd,
        #[serde(rename = "Space")]
        Space,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PinOut {
        #[default]
        #[serde(rename = "Cisco")]
        Cisco,
        #[serde(rename = "Cyclades")]
        Cyclades,
        #[serde(rename = "Digi")]
        Digi,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SerialInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::serial_interface::v1_1_8::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BitRate")]
        pub bit_rate: Option<crate::serial_interface::v1_1_8::BitRate>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectorType")]
        pub connector_type: Option<crate::serial_interface::v1_1_8::ConnectorType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataBits")]
        pub data_bits: Option<crate::serial_interface::v1_1_8::DataBits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FlowControl")]
        pub flow_control: Option<crate::serial_interface::v1_1_8::FlowControl>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceEnabled")]
        pub interface_enabled: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Parity")]
        pub parity: Option<crate::serial_interface::v1_1_8::Parity>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PinOut")]
        pub pin_out: Option<crate::serial_interface::v1_1_8::PinOut>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignalType")]
        pub signal_type: Option<crate::serial_interface::v1_1_8::SignalType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StopBits")]
        pub stop_bits: Option<crate::serial_interface::v1_1_8::StopBits>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SignalType {
        #[default]
        #[serde(rename = "Rs232")]
        Rs232,
        #[serde(rename = "Rs485")]
        Rs485,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StopBits {
        #[default]
        #[serde(rename = "1")]
        N1,
        #[serde(rename = "2")]
        N2,
    }
}
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::serial_interface::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum BitRate {
        #[default]
        #[serde(rename = "115200")]
        N115200,
        #[serde(rename = "1200")]
        N1200,
        #[serde(rename = "19200")]
        N19200,
        #[serde(rename = "230400")]
        N230400,
        #[serde(rename = "2400")]
        N2400,
        #[serde(rename = "38400")]
        N38400,
        #[serde(rename = "4800")]
        N4800,
        #[serde(rename = "57600")]
        N57600,
        #[serde(rename = "9600")]
        N9600,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectorType {
        #[default]
        #[serde(rename = "DB25 Female")]
        DB25Female,
        #[serde(rename = "DB25 Male")]
        DB25Male,
        #[serde(rename = "DB9 Female")]
        DB9Female,
        #[serde(rename = "DB9 Male")]
        DB9Male,
        #[serde(rename = "RJ11")]
        RJ11,
        #[serde(rename = "RJ45")]
        RJ45,
        #[serde(rename = "USB")]
        USB,
        #[serde(rename = "mUSB")]
        MUSB,
        #[serde(rename = "uUSB")]
        UUSB,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DataBits {
        #[default]
        #[serde(rename = "5")]
        N5,
        #[serde(rename = "6")]
        N6,
        #[serde(rename = "7")]
        N7,
        #[serde(rename = "8")]
        N8,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum FlowControl {
        #[default]
        #[serde(rename = "Hardware")]
        Hardware,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "Software")]
        Software,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Parity {
        #[default]
        #[serde(rename = "Even")]
        Even,
        #[serde(rename = "Mark")]
        Mark,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "Odd")]
        Odd,
        #[serde(rename = "Space")]
        Space,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PinOut {
        #[default]
        #[serde(rename = "Cisco")]
        Cisco,
        #[serde(rename = "Cyclades")]
        Cyclades,
        #[serde(rename = "Digi")]
        Digi,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SerialInterface {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::serial_interface::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BitRate")]
        pub bit_rate: Option<crate::serial_interface::v1_2_0::BitRate>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectorType")]
        pub connector_type: Option<crate::serial_interface::v1_2_0::ConnectorType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataBits")]
        pub data_bits: Option<crate::serial_interface::v1_2_0::DataBits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FlowControl")]
        pub flow_control: Option<crate::serial_interface::v1_2_0::FlowControl>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InterfaceEnabled")]
        pub interface_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::serial_interface::v1_2_0::Links>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Parity")]
        pub parity: Option<crate::serial_interface::v1_2_0::Parity>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PinOut")]
        pub pin_out: Option<crate::serial_interface::v1_2_0::PinOut>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SignalType")]
        pub signal_type: Option<crate::serial_interface::v1_2_0::SignalType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StopBits")]
        pub stop_bits: Option<crate::serial_interface::v1_2_0::StopBits>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SignalType {
        #[default]
        #[serde(rename = "Rs232")]
        Rs232,
        #[serde(rename = "Rs485")]
        Rs485,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StopBits {
        #[default]
        #[serde(rename = "1")]
        N1,
        #[serde(rename = "2")]
        N2,
    }
}
