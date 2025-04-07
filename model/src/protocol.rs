use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum Protocol {
    #[default]
    #[serde(rename = "AHCI")]
    AHCI,
    #[serde(rename = "CXL")]
    CXL,
    #[serde(rename = "DVI")]
    DVI,
    #[serde(rename = "DisplayPort")]
    DisplayPort,
    #[serde(rename = "Ethernet")]
    Ethernet,
    #[serde(rename = "FC")]
    FC,
    #[serde(rename = "FCP")]
    FCP,
    #[serde(rename = "FCoE")]
    FCoE,
    #[serde(rename = "FICON")]
    FICON,
    #[serde(rename = "FTP")]
    FTP,
    #[serde(rename = "GenZ")]
    GenZ,
    #[serde(rename = "HDMI")]
    HDMI,
    #[serde(rename = "HTTP")]
    HTTP,
    #[serde(rename = "HTTPS")]
    HTTPS,
    #[serde(rename = "I2C")]
    I2C,
    #[serde(rename = "InfiniBand")]
    InfiniBand,
    #[serde(rename = "MultiProtocol")]
    MultiProtocol,
    #[serde(rename = "NFSv3")]
    NFSv3,
    #[serde(rename = "NFSv4")]
    NFSv4,
    #[serde(rename = "NVLink")]
    NVLink,
    #[serde(rename = "NVMe")]
    NVMe,
    #[serde(rename = "NVMeOverFabrics")]
    NVMeOverFabrics,
    #[serde(rename = "OEM")]
    OEM,
    #[serde(rename = "PCIe")]
    PCIe,
    #[serde(rename = "QPI")]
    QPI,
    #[serde(rename = "RoCE")]
    RoCE,
    #[serde(rename = "RoCEv2")]
    RoCEv2,
    #[serde(rename = "SAS")]
    SAS,
    #[serde(rename = "SATA")]
    SATA,
    #[serde(rename = "SFTP")]
    SFTP,
    #[serde(rename = "SMB")]
    SMB,
    #[serde(rename = "TCP")]
    TCP,
    #[serde(rename = "TFTP")]
    TFTP,
    #[serde(rename = "UDP")]
    UDP,
    #[serde(rename = "UEC")]
    UEC,
    #[serde(rename = "UHCI")]
    UHCI,
    #[serde(rename = "UPI")]
    UPI,
    #[serde(rename = "USB")]
    USB,
    #[serde(rename = "VGA")]
    VGA,
    #[serde(rename = "eMMC")]
    EMMC,
    #[serde(rename = "iSCSI")]
    ISCSI,
    #[serde(rename = "iWARP")]
    IWARP,
}
