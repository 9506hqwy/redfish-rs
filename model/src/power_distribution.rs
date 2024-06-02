pub type PowerDistribution = crate::power_distribution::v1_4_0::PowerDistribution;
pub mod v1_3_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power_distribution::v1_3_3::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#PowerDistribution.TransferControl"
        )]
        pub power_distribution_transfer_control:
            Option<crate::power_distribution::v1_3_3::TransferControl>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Facility")]
        pub facility: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerDistribution {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power_distribution::v1_3_3::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Branches")]
        pub branches: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "EquipmentType")]
        pub equipment_type: crate::power_distribution::v1_3_3::PowerEquipmentType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Feeders")]
        pub feeders: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::power_distribution::v1_3_3::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Mains")]
        pub mains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MainsRedundancy")]
        pub mains_redundancy: Option<crate::redundancy::RedundantGroup>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutletGroups")]
        pub outlet_groups: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Outlets")]
        pub outlets: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplies")]
        pub power_supplies: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerSupplyRedundancy"
        )]
        pub power_supply_redundancy: Option<Vec<crate::redundancy::RedundantGroup>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProductionDate")]
        pub production_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sensors")]
        pub sensors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Subfeeds")]
        pub subfeeds: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransferConfiguration"
        )]
        pub transfer_configuration:
            Option<crate::power_distribution::v1_3_3::TransferConfiguration>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferCriteria")]
        pub transfer_criteria: Option<crate::power_distribution::v1_3_3::TransferCriteria>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerEquipmentType {
        #[default]
        #[serde(rename = "AutomaticTransferSwitch")]
        AutomaticTransferSwitch,
        #[serde(rename = "BatteryShelf")]
        BatteryShelf,
        #[serde(rename = "Bus")]
        Bus,
        #[serde(rename = "FloorPDU")]
        FloorPDU,
        #[serde(rename = "ManualTransferSwitch")]
        ManualTransferSwitch,
        #[serde(rename = "PowerShelf")]
        PowerShelf,
        #[serde(rename = "RackPDU")]
        RackPDU,
        #[serde(rename = "Switchgear")]
        Switchgear,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TransferConfiguration {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveMainsId")]
        pub active_mains_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutoTransferEnabled"
        )]
        pub auto_transfer_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClosedTransitionAllowed"
        )]
        pub closed_transition_allowed: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClosedTransitionTimeoutSeconds"
        )]
        pub closed_transition_timeout_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PreferredMainsId")]
        pub preferred_mains_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RetransferDelaySeconds"
        )]
        pub retransfer_delay_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RetransferEnabled")]
        pub retransfer_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransferDelaySeconds"
        )]
        pub transfer_delay_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferInhibit")]
        pub transfer_inhibit: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TransferControl {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TransferControlRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TransferCriteria {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OverNominalFrequencyHz"
        )]
        pub over_nominal_frequency_hz: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OverVoltageRMSPercentage"
        )]
        pub over_voltage_rms_percentage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransferSensitivity"
        )]
        pub transfer_sensitivity:
            Option<crate::power_distribution::v1_3_3::TransferSensitivityType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnderNominalFrequencyHz"
        )]
        pub under_nominal_frequency_hz: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnderVoltageRMSPercentage"
        )]
        pub under_voltage_rms_percentage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferSensitivityType {
        #[default]
        #[serde(rename = "High")]
        High,
        #[serde(rename = "Low")]
        Low,
        #[serde(rename = "Medium")]
        Medium,
    }
}
pub mod v1_4_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power_distribution::v1_4_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#PowerDistribution.TransferControl"
        )]
        pub power_distribution_transfer_control:
            Option<crate::power_distribution::v1_4_0::TransferControl>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Chassis")]
        pub chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Chassis@odata.count"
        )]
        pub chassis_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Facility")]
        pub facility: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ManagedBy")]
        pub managed_by: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ManagedBy@odata.count"
        )]
        pub managed_by_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerDistribution {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power_distribution::v1_4_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Branches")]
        pub branches: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::power_distribution::v1_4_0::PowerDistributionDescription>,
        #[serde(rename = "EquipmentType")]
        pub equipment_type: crate::power_distribution::v1_4_0::PowerEquipmentType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Feeders")]
        pub feeders: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::power_distribution::v1_4_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Mains")]
        pub mains: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MainsRedundancy")]
        pub mains_redundancy: Option<crate::redundancy::RedundantGroup>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::odata_v4::IdRef>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutletGroups")]
        pub outlet_groups: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Outlets")]
        pub outlets: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityVA")]
        pub power_capacity_va: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplies")]
        pub power_supplies: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerSupplyRedundancy"
        )]
        pub power_supply_redundancy: Option<Vec<crate::redundancy::RedundantGroup>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProductionDate")]
        pub production_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Sensors")]
        pub sensors: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Subfeeds")]
        pub subfeeds: Option<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransferConfiguration"
        )]
        pub transfer_configuration:
            Option<crate::power_distribution::v1_4_0::PowerDistributionTransferConfiguration>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferCriteria")]
        pub transfer_criteria:
            Option<crate::power_distribution::v1_4_0::PowerDistributionTransferCriteria>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserLabel")]
        pub user_label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UUID")]
        pub uuid: Option<crate::power_distribution::v1_4_0::PowerDistributionUUID>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerDistributionDescription {
        V000001(crate::power_distribution::v1_4_0::PowerDistributionDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for PowerDistributionDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerDistributionDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerDistributionTransferConfiguration {
        V010400(crate::power_distribution::v1_4_0::TransferConfiguration),
        V000001(crate::power_distribution::v1_4_0::PowerDistributionTransferConfigurationN1),
    }
    impl Default for PowerDistributionTransferConfiguration {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerDistributionTransferConfigurationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerDistributionTransferCriteria {
        V010400(crate::power_distribution::v1_4_0::TransferCriteria),
        V000001(crate::power_distribution::v1_4_0::PowerDistributionTransferCriteriaN1),
    }
    impl Default for PowerDistributionTransferCriteria {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerDistributionTransferCriteriaN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PowerDistributionUUID {
        V000001(crate::power_distribution::v1_4_0::PowerDistributionUUIDN1),
        ResourceUUID(String),
    }
    impl Default for PowerDistributionUUID {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerDistributionUUIDN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerEquipmentType {
        #[default]
        #[serde(rename = "AutomaticTransferSwitch")]
        AutomaticTransferSwitch,
        #[serde(rename = "BatteryShelf")]
        BatteryShelf,
        #[serde(rename = "Bus")]
        Bus,
        #[serde(rename = "FloorPDU")]
        FloorPDU,
        #[serde(rename = "ManualTransferSwitch")]
        ManualTransferSwitch,
        #[serde(rename = "PowerShelf")]
        PowerShelf,
        #[serde(rename = "RackPDU")]
        RackPDU,
        #[serde(rename = "Switchgear")]
        Switchgear,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TransferConfiguration {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ActiveMainsId")]
        pub active_mains_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutoTransferEnabled"
        )]
        pub auto_transfer_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClosedTransitionAllowed"
        )]
        pub closed_transition_allowed: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ClosedTransitionTimeoutSeconds"
        )]
        pub closed_transition_timeout_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PreferredMainsId")]
        pub preferred_mains_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RetransferDelaySeconds"
        )]
        pub retransfer_delay_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RetransferEnabled")]
        pub retransfer_enabled: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransferDelaySeconds"
        )]
        pub transfer_delay_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferInhibit")]
        pub transfer_inhibit: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TransferControl {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TransferControlRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct TransferCriteria {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OverNominalFrequencyHz"
        )]
        pub over_nominal_frequency_hz: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OverVoltageRMSPercentage"
        )]
        pub over_voltage_rms_percentage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransferSensitivity"
        )]
        pub transfer_sensitivity:
            Option<crate::power_distribution::v1_4_0::TransferCriteriaTransferSensitivity>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnderNominalFrequencyHz"
        )]
        pub under_nominal_frequency_hz: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UnderVoltageRMSPercentage"
        )]
        pub under_voltage_rms_percentage: Option<f64>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum TransferCriteriaTransferSensitivity {
        V010400(crate::power_distribution::v1_4_0::TransferSensitivityType),
        V000001(crate::power_distribution::v1_4_0::TransferCriteriaTransferSensitivityN1),
    }
    impl Default for TransferCriteriaTransferSensitivity {
        fn default() -> Self {
            Self::V010400(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferCriteriaTransferSensitivityN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum TransferSensitivityType {
        #[default]
        #[serde(rename = "High")]
        High,
        #[serde(rename = "Low")]
        Low,
        #[serde(rename = "Medium")]
        Medium,
    }
}
