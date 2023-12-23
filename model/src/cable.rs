pub mod v1_2_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::cable::v1_2_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Cable {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::cable::v1_2_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CableClass")]
        pub cable_class: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CableStatus")]
        pub cable_status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CableType")]
        pub cable_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DownstreamConnectorTypes"
        )]
        pub downstream_connector_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DownstreamName")]
        pub downstream_name: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LengthMeters")]
        pub length_meters: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::cable::v1_2_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpstreamConnectorTypes"
        )]
        pub upstream_connector_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpstreamName")]
        pub upstream_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserDescription")]
        pub user_description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Vendor")]
        pub vendor: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CableClass {
        #[default]
        #[serde(rename = "Fabric")]
        Fabric,
        #[serde(rename = "Fan")]
        Fan,
        #[serde(rename = "General")]
        General,
        #[serde(rename = "Network")]
        Network,
        #[serde(rename = "PCIe")]
        PCIe,
        #[serde(rename = "Power")]
        Power,
        #[serde(rename = "Serial")]
        Serial,
        #[serde(rename = "Storage")]
        Storage,
        #[serde(rename = "USB")]
        USB,
        #[serde(rename = "Video")]
        Video,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CableStatus {
        #[default]
        #[serde(rename = "Degraded")]
        Degraded,
        #[serde(rename = "Disabled")]
        Disabled,
        #[serde(rename = "Failed")]
        Failed,
        #[serde(rename = "Normal")]
        Normal,
        #[serde(rename = "SetByService")]
        SetByService,
        #[serde(rename = "Testing")]
        Testing,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ConnectorType {
        #[default]
        #[serde(rename = "ACPower")]
        ACPower,
        #[serde(rename = "CDFP")]
        CDFP,
        #[serde(rename = "DB9")]
        DB9,
        #[serde(rename = "DCPower")]
        DCPower,
        #[serde(rename = "DisplayPort")]
        DisplayPort,
        #[serde(rename = "HDMI")]
        HDMI,
        #[serde(rename = "ICI")]
        ICI,
        #[serde(rename = "IPASS")]
        IPASS,
        #[serde(rename = "OSFP")]
        OSFP,
        #[serde(rename = "PCIe")]
        PCIe,
        #[serde(rename = "Proprietary")]
        Proprietary,
        #[serde(rename = "QSFP")]
        QSFP,
        #[serde(rename = "RJ45")]
        RJ45,
        #[serde(rename = "SATA")]
        SATA,
        #[serde(rename = "SCSI")]
        SCSI,
        #[serde(rename = "SFP")]
        SFP,
        #[serde(rename = "SFPPlus")]
        SFPPlus,
        #[serde(rename = "SlimSAS")]
        SlimSAS,
        #[serde(rename = "USBA")]
        USBA,
        #[serde(rename = "USBC")]
        USBC,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DownstreamChassis")]
        pub downstream_chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DownstreamChassis@odata.count"
        )]
        pub downstream_chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DownstreamPorts")]
        pub downstream_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DownstreamPorts@odata.count"
        )]
        pub downstream_ports_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DownstreamResources"
        )]
        pub downstream_resources: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DownstreamResources@odata.count"
        )]
        pub downstream_resources_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpstreamChassis")]
        pub upstream_chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpstreamChassis@odata.count"
        )]
        pub upstream_chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpstreamPorts")]
        pub upstream_ports: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpstreamPorts@odata.count"
        )]
        pub upstream_ports_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UpstreamResources")]
        pub upstream_resources: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UpstreamResources@odata.count"
        )]
        pub upstream_resources_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
