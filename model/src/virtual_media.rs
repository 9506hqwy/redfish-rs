pub type VirtualMedia = crate::virtual_media::v1_6_2::VirtualMedia;
pub mod v1_6_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::virtual_media::v1_6_1::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#VirtualMedia.EjectMedia"
        )]
        pub virtual_media_eject_media: Option<crate::virtual_media::v1_6_1::EjectMedia>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#VirtualMedia.InsertMedia"
        )]
        pub virtual_media_insert_media: Option<crate::virtual_media::v1_6_1::InsertMedia>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectedVia {
        #[default]
        #[serde(rename = "Applet")]
        Applet,
        #[serde(rename = "NotConnected")]
        NotConnected,
        #[serde(rename = "Oem")]
        Oem,
        #[serde(rename = "URI")]
        URI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EjectMedia {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EjectMediaRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EjectPolicy {
        #[default]
        #[serde(rename = "AfterUse")]
        AfterUse,
        #[serde(rename = "OnPowerOff")]
        OnPowerOff,
        #[serde(rename = "Persistent")]
        Persistent,
        #[serde(rename = "Session")]
        Session,
        #[serde(rename = "Timed")]
        Timed,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InsertMedia {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InsertMediaRequestBody {
        #[serde(rename = "Image")]
        pub image: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Inserted")]
        pub inserted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferMethod")]
        pub transfer_method: Option<crate::virtual_media::v1_6_1::TransferMethod>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransferProtocolType"
        )]
        pub transfer_protocol_type: Option<crate::virtual_media::v1_6_1::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteProtected")]
        pub write_protected: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MediaType {
        #[default]
        #[serde(rename = "CD")]
        CD,
        #[serde(rename = "DVD")]
        DVD,
        #[serde(rename = "Floppy")]
        Floppy,
        #[serde(rename = "USBStick")]
        USBStick,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferMethod {
        #[default]
        #[serde(rename = "Stream")]
        Stream,
        #[serde(rename = "Upload")]
        Upload,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferProtocolType {
        #[default]
        #[serde(rename = "CIFS")]
        CIFS,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "HTTPS")]
        HTTPS,
        #[serde(rename = "NFS")]
        NFS,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "SFTP")]
        SFTP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualMedia {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::virtual_media::v1_6_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientCertificates")]
        pub client_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedVia")]
        pub connected_via: Option<crate::virtual_media::v1_6_1::ConnectedVia>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EjectPolicy")]
        pub eject_policy: Option<crate::virtual_media::v1_6_1::EjectPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EjectTimeout")]
        pub eject_timeout: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Image")]
        pub image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ImageName")]
        pub image_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Inserted")]
        pub inserted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaTypes")]
        pub media_types: Option<Vec<crate::virtual_media::v1_6_1::MediaType>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferMethod")]
        pub transfer_method: Option<crate::virtual_media::v1_6_1::TransferMethod>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransferProtocolType"
        )]
        pub transfer_protocol_type: Option<crate::virtual_media::v1_6_1::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VerifyCertificate")]
        pub verify_certificate: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteProtected")]
        pub write_protected: Option<bool>,
    }
}
pub mod v1_6_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::virtual_media::v1_6_2::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#VirtualMedia.EjectMedia"
        )]
        pub virtual_media_eject_media: Option<crate::virtual_media::v1_6_2::EjectMedia>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#VirtualMedia.InsertMedia"
        )]
        pub virtual_media_insert_media: Option<crate::virtual_media::v1_6_2::InsertMedia>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectedVia {
        #[default]
        #[serde(rename = "Applet")]
        Applet,
        #[serde(rename = "NotConnected")]
        NotConnected,
        #[serde(rename = "Oem")]
        Oem,
        #[serde(rename = "URI")]
        URI,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EjectMedia {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EjectMediaRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EjectPolicy {
        #[default]
        #[serde(rename = "AfterUse")]
        AfterUse,
        #[serde(rename = "OnPowerOff")]
        OnPowerOff,
        #[serde(rename = "Persistent")]
        Persistent,
        #[serde(rename = "Session")]
        Session,
        #[serde(rename = "Timed")]
        Timed,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InsertMedia {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InsertMediaRequestBody {
        #[serde(rename = "Image")]
        pub image: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Inserted")]
        pub inserted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferMethod")]
        pub transfer_method: Option<crate::virtual_media::v1_6_2::TransferMethod>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransferProtocolType"
        )]
        pub transfer_protocol_type: Option<crate::virtual_media::v1_6_2::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteProtected")]
        pub write_protected: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MediaType {
        #[default]
        #[serde(rename = "CD")]
        CD,
        #[serde(rename = "DVD")]
        DVD,
        #[serde(rename = "Floppy")]
        Floppy,
        #[serde(rename = "USBStick")]
        USBStick,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferMethod {
        #[default]
        #[serde(rename = "Stream")]
        Stream,
        #[serde(rename = "Upload")]
        Upload,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferProtocolType {
        #[default]
        #[serde(rename = "CIFS")]
        CIFS,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "HTTP")]
        HTTP,
        #[serde(rename = "HTTPS")]
        HTTPS,
        #[serde(rename = "NFS")]
        NFS,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SCP")]
        SCP,
        #[serde(rename = "SFTP")]
        SFTP,
        #[serde(rename = "TFTP")]
        TFTP,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct VirtualMedia {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::virtual_media::v1_6_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ClientCertificates")]
        pub client_certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ConnectedVia")]
        pub connected_via: Option<crate::virtual_media::v1_6_2::ConnectedVia>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EjectPolicy")]
        pub eject_policy: Option<crate::virtual_media::v1_6_2::EjectPolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EjectTimeout")]
        pub eject_timeout: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Image")]
        pub image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ImageName")]
        pub image_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Inserted")]
        pub inserted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaTypes")]
        pub media_types: Option<Vec<crate::virtual_media::v1_6_2::MediaType>>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferMethod")]
        pub transfer_method: Option<crate::virtual_media::v1_6_2::TransferMethod>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransferProtocolType"
        )]
        pub transfer_protocol_type: Option<crate::virtual_media::v1_6_2::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VerifyCertificate")]
        pub verify_certificate: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "WriteProtected")]
        pub write_protected: Option<bool>,
    }
}
