pub type PowerSupply = crate::power_supply::v1_5_2::PowerSupply;
pub mod v1_5_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power_supply::v1_5_1::OemActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#PowerSupply.Reset")]
        pub power_supply_reset: Option<crate::power_supply::v1_5_1::Reset>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EfficiencyRating {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EfficiencyPercent")]
        pub efficiency_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LoadPercent")]
        pub load_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InputRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityWatts")]
        pub capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NominalVoltageType")]
        pub nominal_voltage_type: Option<crate::circuit::NominalVoltageType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LineStatus {
        #[default]
        #[serde(rename = "LossOfInput")]
        LossOfInput,
        #[serde(rename = "Normal")]
        Normal,
        #[serde(rename = "OutOfRange")]
        OutOfRange,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Outlet")]
        pub outlet: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerOutlets")]
        pub power_outlets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOutlets@odata.count"
        )]
        pub power_outlets_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweringChassis")]
        pub powering_chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweringChassis@odata.count"
        )]
        pub powering_chassis_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OutputRail {
        #[serde(skip_serializing_if = "Option::is_none", rename = "NominalVoltage")]
        pub nominal_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupply {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power_supply::v1_5_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EfficiencyRatings")]
        pub efficiency_ratings: Option<Vec<crate::power_supply::v1_5_1::EfficiencyRating>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ElectricalSourceManagerURIs"
        )]
        pub electrical_source_manager_ur_is: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ElectricalSourceNames"
        )]
        pub electrical_source_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotPluggable")]
        pub hot_pluggable: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InputNominalVoltageType"
        )]
        pub input_nominal_voltage_type: Option<crate::circuit::NominalVoltageType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputRanges")]
        pub input_ranges: Option<Vec<crate::power_supply::v1_5_1::InputRange>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LineInputStatus")]
        pub line_input_status: Option<crate::power_supply::v1_5_1::LineStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::power_supply::v1_5_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OutputNominalVoltageType"
        )]
        pub output_nominal_voltage_type: Option<crate::circuit::NominalVoltageType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputRails")]
        pub output_rails: Option<Vec<crate::power_supply::v1_5_1::OutputRail>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhaseWiringType")]
        pub phase_wiring_type: Option<crate::circuit::PhaseWiringType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlugType")]
        pub plug_type: Option<crate::circuit::PlugType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplyType")]
        pub power_supply_type: Option<crate::power_supply::v1_5_1::PowerSupplyType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProductionDate")]
        pub production_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Replaceable")]
        pub replaceable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "ACorDC")]
        ACorDC,
        #[serde(rename = "DC")]
        DC,
        #[serde(rename = "DCRegulator")]
        DCRegulator,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Reset {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResetType")]
        pub reset_type: Option<crate::resource::ResetType>,
    }
}
pub mod v1_5_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::power_supply::v1_5_2::OemActions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "#PowerSupply.Reset")]
        pub power_supply_reset: Option<crate::power_supply::v1_5_2::Reset>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EfficiencyRating {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EfficiencyPercent")]
        pub efficiency_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LoadPercent")]
        pub load_percent: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct InputRange {
        #[serde(skip_serializing_if = "Option::is_none", rename = "CapacityWatts")]
        pub capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NominalVoltageType")]
        pub nominal_voltage_type: Option<crate::circuit::NominalVoltageType>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LineStatus {
        #[default]
        #[serde(rename = "LossOfInput")]
        LossOfInput,
        #[serde(rename = "Normal")]
        Normal,
        #[serde(rename = "OutOfRange")]
        OutOfRange,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Outlet")]
        pub outlet: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerOutlets")]
        pub power_outlets: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PowerOutlets@odata.count"
        )]
        pub power_outlets_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PoweringChassis")]
        pub powering_chassis: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PoweringChassis@odata.count"
        )]
        pub powering_chassis_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OutputRail {
        #[serde(skip_serializing_if = "Option::is_none", rename = "NominalVoltage")]
        pub nominal_voltage: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalContext")]
        pub physical_context: Option<crate::physical_context::PhysicalContext>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PowerSupply {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::power_supply::v1_5_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EfficiencyRatings")]
        pub efficiency_ratings: Option<Vec<crate::power_supply::v1_5_2::EfficiencyRating>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ElectricalSourceManagerURIs"
        )]
        pub electrical_source_manager_ur_is: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ElectricalSourceNames"
        )]
        pub electrical_source_names: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HotPluggable")]
        pub hot_pluggable: Option<bool>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "InputNominalVoltageType"
        )]
        pub input_nominal_voltage_type: Option<crate::circuit::NominalVoltageType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InputRanges")]
        pub input_ranges: Option<Vec<crate::power_supply::v1_5_2::InputRange>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LineInputStatus")]
        pub line_input_status: Option<crate::power_supply::v1_5_2::LineStatus>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::power_supply::v1_5_2::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationIndicatorActive"
        )]
        pub location_indicator_active: Option<bool>,
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
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OutputNominalVoltageType"
        )]
        pub output_nominal_voltage_type: Option<crate::circuit::NominalVoltageType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OutputRails")]
        pub output_rails: Option<Vec<crate::power_supply::v1_5_2::OutputRail>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartNumber")]
        pub part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhaseWiringType")]
        pub phase_wiring_type: Option<crate::circuit::PhaseWiringType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlugType")]
        pub plug_type: Option<crate::circuit::PlugType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerCapacityWatts")]
        pub power_capacity_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerSupplyType")]
        pub power_supply_type: Option<crate::power_supply::v1_5_2::PowerSupplyType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProductionDate")]
        pub production_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Replaceable")]
        pub replaceable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SparePartNumber")]
        pub spare_part_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Version")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerSupplyType {
        #[default]
        #[serde(rename = "AC")]
        AC,
        #[serde(rename = "ACorDC")]
        ACorDC,
        #[serde(rename = "DC")]
        DC,
        #[serde(rename = "DCRegulator")]
        DCRegulator,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Reset {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResetRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResetType")]
        pub reset_type: Option<crate::resource::ResetType>,
    }
}
