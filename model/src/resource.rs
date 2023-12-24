use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Condition {
    #[serde(skip_serializing_if = "Option::is_none", rename = "LogEntry")]
    pub log_entry: Option<crate::odata_v4::IdRef>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Message")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MessageArgs")]
    pub message_args: Option<Vec<String>>,
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
    pub origin_of_condition: Option<crate::odata_v4::IdRef>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Resolution")]
    pub resolution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
    pub severity: Option<crate::resource::Health>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Timestamp")]
    pub timestamp: Option<String>,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum Health {
    #[default]
    #[serde(rename = "Critical")]
    Critical,
    #[serde(rename = "OK")]
    OK,
    #[serde(rename = "Warning")]
    Warning,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Identifier {
    V011600(crate::resource::v1_16_0::Identifier),
    V011500(crate::resource::v1_15_0::Identifier),
    V011401(crate::resource::v1_14_1::Identifier),
    V011302(crate::resource::v1_13_2::Identifier),
    V011203(crate::resource::v1_12_3::Identifier),
    V011104(crate::resource::v1_11_4::Identifier),
    V011005(crate::resource::v1_10_5::Identifier),
    V010908(crate::resource::v1_9_8::Identifier),
    V010810(crate::resource::v1_8_10::Identifier),
    V010710(crate::resource::v1_7_10::Identifier),
    V010611(crate::resource::v1_6_11::Identifier),
    V010511(crate::resource::v1_5_11::Identifier),
    V010412(crate::resource::v1_4_12::Identifier),
    V010313(crate::resource::v1_3_13::Identifier),
    V010214(crate::resource::v1_2_14::Identifier),
    V010115(crate::resource::v1_1_15::Identifier),
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum IndicatorLED {
    #[default]
    #[serde(rename = "Blinking")]
    Blinking,
    #[serde(rename = "Lit")]
    Lit,
    #[serde(rename = "Off")]
    Off,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Item {
    ResourceResource(crate::resource::Resource),
    ResourceReferenceableMember(crate::resource::ReferenceableMember),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ItemOrCollection {
    ResourceResourceCollection(crate::resource::ResourceCollection),
    ResourceItem(crate::resource::Item),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Links {
    #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
    pub oem: Option<crate::resource::Oem>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Location {
    V011600(crate::resource::v1_16_0::Location),
    V011500(crate::resource::v1_15_0::Location),
    V011401(crate::resource::v1_14_1::Location),
    V011302(crate::resource::v1_13_2::Location),
    V011203(crate::resource::v1_12_3::Location),
    V011104(crate::resource::v1_11_4::Location),
    V011005(crate::resource::v1_10_5::Location),
    V010908(crate::resource::v1_9_8::Location),
    V010810(crate::resource::v1_8_10::Location),
    V010710(crate::resource::v1_7_10::Location),
    V010611(crate::resource::v1_6_11::Location),
    V010511(crate::resource::v1_5_11::Location),
    V010412(crate::resource::v1_4_12::Location),
    V010313(crate::resource::v1_3_13::Location),
    V010214(crate::resource::v1_2_14::Location),
    V010115(crate::resource::v1_1_15::Location),
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Oem {}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OemObject {}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum PowerState {
    #[default]
    #[serde(rename = "Off")]
    Off,
    #[serde(rename = "On")]
    On,
    #[serde(rename = "Paused")]
    Paused,
    #[serde(rename = "PoweringOff")]
    PoweringOff,
    #[serde(rename = "PoweringOn")]
    PoweringOn,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ReferenceableMember {
    V011600(crate::resource::v1_16_0::ReferenceableMember),
    V011500(crate::resource::v1_15_0::ReferenceableMember),
    V011401(crate::resource::v1_14_1::ReferenceableMember),
    V011302(crate::resource::v1_13_2::ReferenceableMember),
    V011203(crate::resource::v1_12_3::ReferenceableMember),
    V011104(crate::resource::v1_11_4::ReferenceableMember),
    V011005(crate::resource::v1_10_5::ReferenceableMember),
    V010908(crate::resource::v1_9_8::ReferenceableMember),
    V010810(crate::resource::v1_8_10::ReferenceableMember),
    V010710(crate::resource::v1_7_10::ReferenceableMember),
    V010611(crate::resource::v1_6_11::ReferenceableMember),
    V010511(crate::resource::v1_5_11::ReferenceableMember),
    V010412(crate::resource::v1_4_12::ReferenceableMember),
    V010313(crate::resource::v1_3_13::ReferenceableMember),
    V010214(crate::resource::v1_2_14::ReferenceableMember),
    V010115(crate::resource::v1_1_15::ReferenceableMember),
    V010013(crate::resource::v1_0_13::ReferenceableMember),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ResetType {
    #[default]
    #[serde(rename = "ForceOff")]
    ForceOff,
    #[serde(rename = "ForceOn")]
    ForceOn,
    #[serde(rename = "ForceRestart")]
    ForceRestart,
    #[serde(rename = "GracefulRestart")]
    GracefulRestart,
    #[serde(rename = "GracefulShutdown")]
    GracefulShutdown,
    #[serde(rename = "Nmi")]
    Nmi,
    #[serde(rename = "On")]
    On,
    #[serde(rename = "Pause")]
    Pause,
    #[serde(rename = "PowerCycle")]
    PowerCycle,
    #[serde(rename = "PushPowerButton")]
    PushPowerButton,
    #[serde(rename = "Resume")]
    Resume,
    #[serde(rename = "Suspend")]
    Suspend,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Resource {
    V011600(crate::resource::v1_16_0::Resource),
    V011500(crate::resource::v1_15_0::Resource),
    V011401(crate::resource::v1_14_1::Resource),
    V011302(crate::resource::v1_13_2::Resource),
    V011203(crate::resource::v1_12_3::Resource),
    V011104(crate::resource::v1_11_4::Resource),
    V011005(crate::resource::v1_10_5::Resource),
    V010908(crate::resource::v1_9_8::Resource),
    V010810(crate::resource::v1_8_10::Resource),
    V010710(crate::resource::v1_7_10::Resource),
    V010611(crate::resource::v1_6_11::Resource),
    V010511(crate::resource::v1_5_11::Resource),
    V010412(crate::resource::v1_4_12::Resource),
    V010313(crate::resource::v1_3_13::Resource),
    V010214(crate::resource::v1_2_14::Resource),
    V010115(crate::resource::v1_1_15::Resource),
    V010013(crate::resource::v1_0_13::Resource),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ResourceCollection {
    V011600(crate::resource::v1_16_0::ResourceCollection),
    V011500(crate::resource::v1_15_0::ResourceCollection),
    V011401(crate::resource::v1_14_1::ResourceCollection),
    V011302(crate::resource::v1_13_2::ResourceCollection),
    V011203(crate::resource::v1_12_3::ResourceCollection),
    V011104(crate::resource::v1_11_4::ResourceCollection),
    V011005(crate::resource::v1_10_5::ResourceCollection),
    V010908(crate::resource::v1_9_8::ResourceCollection),
    V010810(crate::resource::v1_8_10::ResourceCollection),
    V010710(crate::resource::v1_7_10::ResourceCollection),
    V010611(crate::resource::v1_6_11::ResourceCollection),
    V010511(crate::resource::v1_5_11::ResourceCollection),
    V010412(crate::resource::v1_4_12::ResourceCollection),
    V010313(crate::resource::v1_3_13::ResourceCollection),
    V010214(crate::resource::v1_2_14::ResourceCollection),
    V010115(crate::resource::v1_1_15::ResourceCollection),
    V010013(crate::resource::v1_0_13::ResourceCollection),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum State {
    #[default]
    #[serde(rename = "Absent")]
    Absent,
    #[serde(rename = "Deferring")]
    Deferring,
    #[serde(rename = "Disabled")]
    Disabled,
    #[serde(rename = "Enabled")]
    Enabled,
    #[serde(rename = "InTest")]
    InTest,
    #[serde(rename = "Qualified")]
    Qualified,
    #[serde(rename = "Quiesced")]
    Quiesced,
    #[serde(rename = "StandbyOffline")]
    StandbyOffline,
    #[serde(rename = "StandbySpare")]
    StandbySpare,
    #[serde(rename = "Starting")]
    Starting,
    #[serde(rename = "UnavailableOffline")]
    UnavailableOffline,
    #[serde(rename = "Updating")]
    Updating,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Status {
    #[serde(skip_serializing_if = "Option::is_none", rename = "Conditions")]
    pub conditions: Option<Vec<crate::resource::Condition>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Health")]
    pub health: Option<crate::resource::Health>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "HealthRollup")]
    pub health_rollup: Option<crate::resource::Health>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
    pub oem: Option<crate::resource::Oem>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "State")]
    pub state: Option<crate::resource::State>,
}
pub mod v1_0_13 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_1_15 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_1_15::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_2_14 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_2_14::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_3_0::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IndicatorLED {
        #[default]
        #[serde(rename = "Blinking")]
        Blinking,
        #[serde(rename = "Lit")]
        Lit,
        #[serde(rename = "Off")]
        Off,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_3_0::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_3_0::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_3_0::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PowerState {
        #[default]
        #[serde(rename = "Off")]
        Off,
        #[serde(rename = "On")]
        On,
        #[serde(rename = "PoweringOff")]
        PoweringOff,
        #[serde(rename = "PoweringOn")]
        PoweringOn,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MemberId")]
        pub member_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.context")]
        pub odata_context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.id")]
        pub odata_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.type")]
        pub odata_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.context")]
        pub odata_context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.id")]
        pub odata_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.type")]
        pub odata_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.context")]
        pub odata_context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.id")]
        pub odata_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "@odata.type")]
        pub odata_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
}
pub mod v1_3_13 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_3_13::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_3_13::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_3_13::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_3_13::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_4_12 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_4_12::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_4_12::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_4_12::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_4_12::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_5_11 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_5_11::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartLocation")]
        pub part_location: Option<crate::resource::v1_5_11::PartLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_5_11::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_5_11::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocationType {
        #[default]
        #[serde(rename = "Bay")]
        Bay,
        #[serde(rename = "Connector")]
        Connector,
        #[serde(rename = "Slot")]
        Slot,
        #[serde(rename = "Socket")]
        Socket,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Orientation {
        #[default]
        #[serde(rename = "BackToFront")]
        BackToFront,
        #[serde(rename = "BottomToTop")]
        BottomToTop,
        #[serde(rename = "FrontToBack")]
        FrontToBack,
        #[serde(rename = "LeftToRight")]
        LeftToRight,
        #[serde(rename = "RightToLeft")]
        RightToLeft,
        #[serde(rename = "TopToBottom")]
        TopToBottom,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PartLocation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationOrdinalValue"
        )]
        pub location_ordinal_value: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocationType")]
        pub location_type: Option<crate::resource::v1_5_11::LocationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<crate::resource::v1_5_11::Orientation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<crate::resource::v1_5_11::Reference>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceLabel")]
        pub service_label: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_5_11::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Reference {
        #[default]
        #[serde(rename = "Bottom")]
        Bottom,
        #[serde(rename = "Front")]
        Front,
        #[serde(rename = "Left")]
        Left,
        #[serde(rename = "Middle")]
        Middle,
        #[serde(rename = "Rear")]
        Rear,
        #[serde(rename = "Right")]
        Right,
        #[serde(rename = "Top")]
        Top,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_6_11 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "NQN")]
        NQN,
        #[serde(rename = "NSID")]
        NSID,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_6_11::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AltitudeMeters")]
        pub altitude_meters: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Latitude")]
        pub latitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Longitude")]
        pub longitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartLocation")]
        pub part_location: Option<crate::resource::v1_6_11::PartLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_6_11::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_6_11::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocationType {
        #[default]
        #[serde(rename = "Bay")]
        Bay,
        #[serde(rename = "Connector")]
        Connector,
        #[serde(rename = "Slot")]
        Slot,
        #[serde(rename = "Socket")]
        Socket,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Orientation {
        #[default]
        #[serde(rename = "BackToFront")]
        BackToFront,
        #[serde(rename = "BottomToTop")]
        BottomToTop,
        #[serde(rename = "FrontToBack")]
        FrontToBack,
        #[serde(rename = "LeftToRight")]
        LeftToRight,
        #[serde(rename = "RightToLeft")]
        RightToLeft,
        #[serde(rename = "TopToBottom")]
        TopToBottom,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PartLocation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationOrdinalValue"
        )]
        pub location_ordinal_value: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocationType")]
        pub location_type: Option<crate::resource::v1_6_11::LocationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<crate::resource::v1_6_11::Orientation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<crate::resource::v1_6_11::Reference>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceLabel")]
        pub service_label: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_6_11::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Reference {
        #[default]
        #[serde(rename = "Bottom")]
        Bottom,
        #[serde(rename = "Front")]
        Front,
        #[serde(rename = "Left")]
        Left,
        #[serde(rename = "Middle")]
        Middle,
        #[serde(rename = "Rear")]
        Rear,
        #[serde(rename = "Right")]
        Right,
        #[serde(rename = "Top")]
        Top,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_7_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContactInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContactName")]
        pub contact_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAddress")]
        pub email_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "NQN")]
        NQN,
        #[serde(rename = "NSID")]
        NSID,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_7_10::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AltitudeMeters")]
        pub altitude_meters: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Contacts")]
        pub contacts: Option<Vec<crate::resource::v1_7_10::ContactInfo>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Latitude")]
        pub latitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Longitude")]
        pub longitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartLocation")]
        pub part_location: Option<crate::resource::v1_7_10::PartLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_7_10::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_7_10::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocationType {
        #[default]
        #[serde(rename = "Bay")]
        Bay,
        #[serde(rename = "Connector")]
        Connector,
        #[serde(rename = "Slot")]
        Slot,
        #[serde(rename = "Socket")]
        Socket,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Orientation {
        #[default]
        #[serde(rename = "BackToFront")]
        BackToFront,
        #[serde(rename = "BottomToTop")]
        BottomToTop,
        #[serde(rename = "FrontToBack")]
        FrontToBack,
        #[serde(rename = "LeftToRight")]
        LeftToRight,
        #[serde(rename = "RightToLeft")]
        RightToLeft,
        #[serde(rename = "TopToBottom")]
        TopToBottom,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PartLocation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationOrdinalValue"
        )]
        pub location_ordinal_value: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocationType")]
        pub location_type: Option<crate::resource::v1_7_10::LocationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<crate::resource::v1_7_10::Orientation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<crate::resource::v1_7_10::Reference>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceLabel")]
        pub service_label: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_7_10::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Reference {
        #[default]
        #[serde(rename = "Bottom")]
        Bottom,
        #[serde(rename = "Front")]
        Front,
        #[serde(rename = "Left")]
        Left,
        #[serde(rename = "Middle")]
        Middle,
        #[serde(rename = "Rear")]
        Rear,
        #[serde(rename = "Right")]
        Right,
        #[serde(rename = "Top")]
        Top,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_8_10 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContactInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContactName")]
        pub contact_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAddress")]
        pub email_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "NQN")]
        NQN,
        #[serde(rename = "NSID")]
        NSID,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_8_10::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AltitudeMeters")]
        pub altitude_meters: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Contacts")]
        pub contacts: Option<Vec<crate::resource::v1_8_10::ContactInfo>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Latitude")]
        pub latitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Longitude")]
        pub longitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartLocation")]
        pub part_location: Option<crate::resource::v1_8_10::PartLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_8_10::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_8_10::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocationType {
        #[default]
        #[serde(rename = "Bay")]
        Bay,
        #[serde(rename = "Connector")]
        Connector,
        #[serde(rename = "Slot")]
        Slot,
        #[serde(rename = "Socket")]
        Socket,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Orientation {
        #[default]
        #[serde(rename = "BackToFront")]
        BackToFront,
        #[serde(rename = "BottomToTop")]
        BottomToTop,
        #[serde(rename = "FrontToBack")]
        FrontToBack,
        #[serde(rename = "LeftToRight")]
        LeftToRight,
        #[serde(rename = "RightToLeft")]
        RightToLeft,
        #[serde(rename = "TopToBottom")]
        TopToBottom,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PartLocation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationOrdinalValue"
        )]
        pub location_ordinal_value: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocationType")]
        pub location_type: Option<crate::resource::v1_8_10::LocationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<crate::resource::v1_8_10::Orientation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<crate::resource::v1_8_10::Reference>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceLabel")]
        pub service_label: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_8_10::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Reference {
        #[default]
        #[serde(rename = "Bottom")]
        Bottom,
        #[serde(rename = "Front")]
        Front,
        #[serde(rename = "Left")]
        Left,
        #[serde(rename = "Middle")]
        Middle,
        #[serde(rename = "Rear")]
        Rear,
        #[serde(rename = "Right")]
        Right,
        #[serde(rename = "Top")]
        Top,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_9_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContactInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContactName")]
        pub contact_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAddress")]
        pub email_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "NQN")]
        NQN,
        #[serde(rename = "NSID")]
        NSID,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_9_8::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AltitudeMeters")]
        pub altitude_meters: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Contacts")]
        pub contacts: Option<Vec<crate::resource::v1_9_8::ContactInfo>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Latitude")]
        pub latitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Longitude")]
        pub longitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartLocation")]
        pub part_location: Option<crate::resource::v1_9_8::PartLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_9_8::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_9_8::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocationType {
        #[default]
        #[serde(rename = "Bay")]
        Bay,
        #[serde(rename = "Connector")]
        Connector,
        #[serde(rename = "Slot")]
        Slot,
        #[serde(rename = "Socket")]
        Socket,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Orientation {
        #[default]
        #[serde(rename = "BackToFront")]
        BackToFront,
        #[serde(rename = "BottomToTop")]
        BottomToTop,
        #[serde(rename = "FrontToBack")]
        FrontToBack,
        #[serde(rename = "LeftToRight")]
        LeftToRight,
        #[serde(rename = "RightToLeft")]
        RightToLeft,
        #[serde(rename = "TopToBottom")]
        TopToBottom,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PartLocation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationOrdinalValue"
        )]
        pub location_ordinal_value: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocationType")]
        pub location_type: Option<crate::resource::v1_9_8::LocationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<crate::resource::v1_9_8::Orientation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<crate::resource::v1_9_8::Reference>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceLabel")]
        pub service_label: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_9_8::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Reference {
        #[default]
        #[serde(rename = "Bottom")]
        Bottom,
        #[serde(rename = "Front")]
        Front,
        #[serde(rename = "Left")]
        Left,
        #[serde(rename = "Middle")]
        Middle,
        #[serde(rename = "Rear")]
        Rear,
        #[serde(rename = "Right")]
        Right,
        #[serde(rename = "Top")]
        Top,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_10_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContactInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContactName")]
        pub contact_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAddress")]
        pub email_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "NGUID")]
        NGUID,
        #[serde(rename = "NQN")]
        NQN,
        #[serde(rename = "NSID")]
        NSID,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_10_5::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AltitudeMeters")]
        pub altitude_meters: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Contacts")]
        pub contacts: Option<Vec<crate::resource::v1_10_5::ContactInfo>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Latitude")]
        pub latitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Longitude")]
        pub longitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartLocation")]
        pub part_location: Option<crate::resource::v1_10_5::PartLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_10_5::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_10_5::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocationType {
        #[default]
        #[serde(rename = "Bay")]
        Bay,
        #[serde(rename = "Connector")]
        Connector,
        #[serde(rename = "Slot")]
        Slot,
        #[serde(rename = "Socket")]
        Socket,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Orientation {
        #[default]
        #[serde(rename = "BackToFront")]
        BackToFront,
        #[serde(rename = "BottomToTop")]
        BottomToTop,
        #[serde(rename = "FrontToBack")]
        FrontToBack,
        #[serde(rename = "LeftToRight")]
        LeftToRight,
        #[serde(rename = "RightToLeft")]
        RightToLeft,
        #[serde(rename = "TopToBottom")]
        TopToBottom,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PartLocation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationOrdinalValue"
        )]
        pub location_ordinal_value: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocationType")]
        pub location_type: Option<crate::resource::v1_10_5::LocationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<crate::resource::v1_10_5::Orientation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<crate::resource::v1_10_5::Reference>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceLabel")]
        pub service_label: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_10_5::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Reference {
        #[default]
        #[serde(rename = "Bottom")]
        Bottom,
        #[serde(rename = "Front")]
        Front,
        #[serde(rename = "Left")]
        Left,
        #[serde(rename = "Middle")]
        Middle,
        #[serde(rename = "Rear")]
        Rear,
        #[serde(rename = "Right")]
        Right,
        #[serde(rename = "Top")]
        Top,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_11_4 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContactInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContactName")]
        pub contact_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAddress")]
        pub email_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "NGUID")]
        NGUID,
        #[serde(rename = "NQN")]
        NQN,
        #[serde(rename = "NSID")]
        NSID,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_11_4::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AltitudeMeters")]
        pub altitude_meters: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Contacts")]
        pub contacts: Option<Vec<crate::resource::v1_11_4::ContactInfo>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Latitude")]
        pub latitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Longitude")]
        pub longitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartLocation")]
        pub part_location: Option<crate::resource::v1_11_4::PartLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_11_4::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_11_4::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocationType {
        #[default]
        #[serde(rename = "Bay")]
        Bay,
        #[serde(rename = "Connector")]
        Connector,
        #[serde(rename = "Slot")]
        Slot,
        #[serde(rename = "Socket")]
        Socket,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Orientation {
        #[default]
        #[serde(rename = "BackToFront")]
        BackToFront,
        #[serde(rename = "BottomToTop")]
        BottomToTop,
        #[serde(rename = "FrontToBack")]
        FrontToBack,
        #[serde(rename = "LeftToRight")]
        LeftToRight,
        #[serde(rename = "RightToLeft")]
        RightToLeft,
        #[serde(rename = "TopToBottom")]
        TopToBottom,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PartLocation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationOrdinalValue"
        )]
        pub location_ordinal_value: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocationType")]
        pub location_type: Option<crate::resource::v1_11_4::LocationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<crate::resource::v1_11_4::Orientation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<crate::resource::v1_11_4::Reference>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceLabel")]
        pub service_label: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_11_4::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Reference {
        #[default]
        #[serde(rename = "Bottom")]
        Bottom,
        #[serde(rename = "Front")]
        Front,
        #[serde(rename = "Left")]
        Left,
        #[serde(rename = "Middle")]
        Middle,
        #[serde(rename = "Rear")]
        Rear,
        #[serde(rename = "Right")]
        Right,
        #[serde(rename = "Top")]
        Top,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_12_3 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContactInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContactName")]
        pub contact_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAddress")]
        pub email_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "NGUID")]
        NGUID,
        #[serde(rename = "NQN")]
        NQN,
        #[serde(rename = "NSID")]
        NSID,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_12_3::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AltitudeMeters")]
        pub altitude_meters: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Contacts")]
        pub contacts: Option<Vec<crate::resource::v1_12_3::ContactInfo>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Latitude")]
        pub latitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Longitude")]
        pub longitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartLocation")]
        pub part_location: Option<crate::resource::v1_12_3::PartLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_12_3::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_12_3::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocationType {
        #[default]
        #[serde(rename = "Backplane")]
        Backplane,
        #[serde(rename = "Bay")]
        Bay,
        #[serde(rename = "Connector")]
        Connector,
        #[serde(rename = "Slot")]
        Slot,
        #[serde(rename = "Socket")]
        Socket,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Orientation {
        #[default]
        #[serde(rename = "BackToFront")]
        BackToFront,
        #[serde(rename = "BottomToTop")]
        BottomToTop,
        #[serde(rename = "FrontToBack")]
        FrontToBack,
        #[serde(rename = "LeftToRight")]
        LeftToRight,
        #[serde(rename = "RightToLeft")]
        RightToLeft,
        #[serde(rename = "TopToBottom")]
        TopToBottom,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PartLocation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationOrdinalValue"
        )]
        pub location_ordinal_value: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocationType")]
        pub location_type: Option<crate::resource::v1_12_3::LocationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<crate::resource::v1_12_3::Orientation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<crate::resource::v1_12_3::Reference>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceLabel")]
        pub service_label: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_12_3::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Reference {
        #[default]
        #[serde(rename = "Bottom")]
        Bottom,
        #[serde(rename = "Front")]
        Front,
        #[serde(rename = "Left")]
        Left,
        #[serde(rename = "Middle")]
        Middle,
        #[serde(rename = "Rear")]
        Rear,
        #[serde(rename = "Right")]
        Right,
        #[serde(rename = "Top")]
        Top,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_13_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContactInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContactName")]
        pub contact_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAddress")]
        pub email_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "NGUID")]
        NGUID,
        #[serde(rename = "NQN")]
        NQN,
        #[serde(rename = "NSID")]
        NSID,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_13_2::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AltitudeMeters")]
        pub altitude_meters: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Contacts")]
        pub contacts: Option<Vec<crate::resource::v1_13_2::ContactInfo>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Latitude")]
        pub latitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Longitude")]
        pub longitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartLocation")]
        pub part_location: Option<crate::resource::v1_13_2::PartLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_13_2::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_13_2::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocationType {
        #[default]
        #[serde(rename = "Backplane")]
        Backplane,
        #[serde(rename = "Bay")]
        Bay,
        #[serde(rename = "Connector")]
        Connector,
        #[serde(rename = "Embedded")]
        Embedded,
        #[serde(rename = "Slot")]
        Slot,
        #[serde(rename = "Socket")]
        Socket,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Orientation {
        #[default]
        #[serde(rename = "BackToFront")]
        BackToFront,
        #[serde(rename = "BottomToTop")]
        BottomToTop,
        #[serde(rename = "FrontToBack")]
        FrontToBack,
        #[serde(rename = "LeftToRight")]
        LeftToRight,
        #[serde(rename = "RightToLeft")]
        RightToLeft,
        #[serde(rename = "TopToBottom")]
        TopToBottom,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PartLocation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationOrdinalValue"
        )]
        pub location_ordinal_value: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocationType")]
        pub location_type: Option<crate::resource::v1_13_2::LocationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<crate::resource::v1_13_2::Orientation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<crate::resource::v1_13_2::Reference>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceLabel")]
        pub service_label: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_13_2::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Reference {
        #[default]
        #[serde(rename = "Bottom")]
        Bottom,
        #[serde(rename = "Front")]
        Front,
        #[serde(rename = "Left")]
        Left,
        #[serde(rename = "Middle")]
        Middle,
        #[serde(rename = "Rear")]
        Rear,
        #[serde(rename = "Right")]
        Right,
        #[serde(rename = "Top")]
        Top,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_14_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContactInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContactName")]
        pub contact_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAddress")]
        pub email_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "MACAddress")]
        MACAddress,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "NGUID")]
        NGUID,
        #[serde(rename = "NQN")]
        NQN,
        #[serde(rename = "NSID")]
        NSID,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_14_1::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AltitudeMeters")]
        pub altitude_meters: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Contacts")]
        pub contacts: Option<Vec<crate::resource::v1_14_1::ContactInfo>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Latitude")]
        pub latitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Longitude")]
        pub longitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartLocation")]
        pub part_location: Option<crate::resource::v1_14_1::PartLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_14_1::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_14_1::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocationType {
        #[default]
        #[serde(rename = "Backplane")]
        Backplane,
        #[serde(rename = "Bay")]
        Bay,
        #[serde(rename = "Connector")]
        Connector,
        #[serde(rename = "Embedded")]
        Embedded,
        #[serde(rename = "Slot")]
        Slot,
        #[serde(rename = "Socket")]
        Socket,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Orientation {
        #[default]
        #[serde(rename = "BackToFront")]
        BackToFront,
        #[serde(rename = "BottomToTop")]
        BottomToTop,
        #[serde(rename = "FrontToBack")]
        FrontToBack,
        #[serde(rename = "LeftToRight")]
        LeftToRight,
        #[serde(rename = "RightToLeft")]
        RightToLeft,
        #[serde(rename = "TopToBottom")]
        TopToBottom,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PartLocation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationOrdinalValue"
        )]
        pub location_ordinal_value: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocationType")]
        pub location_type: Option<crate::resource::v1_14_1::LocationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<crate::resource::v1_14_1::Orientation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<crate::resource::v1_14_1::Reference>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceLabel")]
        pub service_label: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_14_1::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Reference {
        #[default]
        #[serde(rename = "Bottom")]
        Bottom,
        #[serde(rename = "Front")]
        Front,
        #[serde(rename = "Left")]
        Left,
        #[serde(rename = "Middle")]
        Middle,
        #[serde(rename = "Rear")]
        Rear,
        #[serde(rename = "Right")]
        Right,
        #[serde(rename = "Top")]
        Top,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_15_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContactInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContactName")]
        pub contact_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAddress")]
        pub email_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "GCXLID")]
        GCXLID,
        #[serde(rename = "MACAddress")]
        MACAddress,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "NGUID")]
        NGUID,
        #[serde(rename = "NQN")]
        NQN,
        #[serde(rename = "NSID")]
        NSID,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_15_0::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AltitudeMeters")]
        pub altitude_meters: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Contacts")]
        pub contacts: Option<Vec<crate::resource::v1_15_0::ContactInfo>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Latitude")]
        pub latitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Longitude")]
        pub longitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartLocation")]
        pub part_location: Option<crate::resource::v1_15_0::PartLocation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_15_0::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_15_0::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocationType {
        #[default]
        #[serde(rename = "Backplane")]
        Backplane,
        #[serde(rename = "Bay")]
        Bay,
        #[serde(rename = "Connector")]
        Connector,
        #[serde(rename = "Embedded")]
        Embedded,
        #[serde(rename = "Slot")]
        Slot,
        #[serde(rename = "Socket")]
        Socket,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Orientation {
        #[default]
        #[serde(rename = "BackToFront")]
        BackToFront,
        #[serde(rename = "BottomToTop")]
        BottomToTop,
        #[serde(rename = "FrontToBack")]
        FrontToBack,
        #[serde(rename = "LeftToRight")]
        LeftToRight,
        #[serde(rename = "RightToLeft")]
        RightToLeft,
        #[serde(rename = "TopToBottom")]
        TopToBottom,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PartLocation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationOrdinalValue"
        )]
        pub location_ordinal_value: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocationType")]
        pub location_type: Option<crate::resource::v1_15_0::LocationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<crate::resource::v1_15_0::Orientation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<crate::resource::v1_15_0::Reference>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceLabel")]
        pub service_label: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_15_0::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Reference {
        #[default]
        #[serde(rename = "Bottom")]
        Bottom,
        #[serde(rename = "Front")]
        Front,
        #[serde(rename = "Left")]
        Left,
        #[serde(rename = "Middle")]
        Middle,
        #[serde(rename = "Rear")]
        Rear,
        #[serde(rename = "Right")]
        Right,
        #[serde(rename = "Top")]
        Top,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
pub mod v1_16_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ContactInfo {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ContactName")]
        pub contact_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EmailAddress")]
        pub email_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DurableNameFormat {
        #[default]
        #[serde(rename = "EUI")]
        EUI,
        #[serde(rename = "FC_WWN")]
        FCWWN,
        #[serde(rename = "GCXLID")]
        GCXLID,
        #[serde(rename = "MACAddress")]
        MACAddress,
        #[serde(rename = "NAA")]
        NAA,
        #[serde(rename = "NGUID")]
        NGUID,
        #[serde(rename = "NQN")]
        NQN,
        #[serde(rename = "NSID")]
        NSID,
        #[serde(rename = "UUID")]
        UUID,
        #[serde(rename = "iQN")]
        IQN,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Identifier {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableName")]
        pub durable_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DurableNameFormat")]
        pub durable_name_format: Option<crate::resource::v1_16_0::DurableNameFormat>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AltitudeMeters")]
        pub altitude_meters: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Contacts")]
        pub contacts: Option<Vec<crate::resource::v1_16_0::ContactInfo>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Info")]
        pub info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InfoFormat")]
        pub info_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Latitude")]
        pub latitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Longitude")]
        pub longitude: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PartLocation")]
        pub part_location: Option<crate::resource::v1_16_0::PartLocation>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PartLocationContext"
        )]
        pub part_location_context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_16_0::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_16_0::PostalAddress>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocationType {
        #[default]
        #[serde(rename = "Backplane")]
        Backplane,
        #[serde(rename = "Bay")]
        Bay,
        #[serde(rename = "Connector")]
        Connector,
        #[serde(rename = "Embedded")]
        Embedded,
        #[serde(rename = "Slot")]
        Slot,
        #[serde(rename = "Socket")]
        Socket,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Orientation {
        #[default]
        #[serde(rename = "BackToFront")]
        BackToFront,
        #[serde(rename = "BottomToTop")]
        BottomToTop,
        #[serde(rename = "FrontToBack")]
        FrontToBack,
        #[serde(rename = "LeftToRight")]
        LeftToRight,
        #[serde(rename = "RightToLeft")]
        RightToLeft,
        #[serde(rename = "TopToBottom")]
        TopToBottom,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PartLocation {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LocationOrdinalValue"
        )]
        pub location_ordinal_value: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LocationType")]
        pub location_type: Option<crate::resource::v1_16_0::LocationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<crate::resource::v1_16_0::Orientation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<crate::resource::v1_16_0::Reference>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceLabel")]
        pub service_label: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Placement {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Rack")]
        pub rack: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffset")]
        pub rack_offset: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RackOffsetUnits")]
        pub rack_offset_units: Option<crate::resource::v1_16_0::RackUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PostalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalCode")]
        pub additional_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalInfo")]
        pub additional_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Building")]
        pub building: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Community")]
        pub community: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "District")]
        pub district: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Division")]
        pub division: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Floor")]
        pub floor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GPSCoords")]
        pub gps_coords: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumber")]
        pub house_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HouseNumberSuffix")]
        pub house_number_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Landmark")]
        pub landmark: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LeadingStreetDirection"
        )]
        pub leading_street_direction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Neighborhood")]
        pub neighborhood: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PlaceType")]
        pub place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "POBox")]
        pub po_box: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Road")]
        pub road: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadBranch")]
        pub road_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPostModifier")]
        pub road_post_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadPreModifier")]
        pub road_pre_modifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSection")]
        pub road_section: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RoadSubBranch")]
        pub road_sub_branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Room")]
        pub room: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Seat")]
        pub seat: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Street")]
        pub street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetSuffix")]
        pub street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Territory")]
        pub territory: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TrailingStreetSuffix"
        )]
        pub trailing_street_suffix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Unit")]
        pub unit: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum Reference {
        #[default]
        #[serde(rename = "Bottom")]
        Bottom,
        #[serde(rename = "Front")]
        Front,
        #[serde(rename = "Left")]
        Left,
        #[serde(rename = "Middle")]
        Middle,
        #[serde(rename = "Rear")]
        Rear,
        #[serde(rename = "Right")]
        Right,
        #[serde(rename = "Top")]
        Top,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ReferenceableMember {
        #[serde(rename = "MemberId")]
        pub member_id: String,
        #[serde(rename = "@odata.id")]
        pub odata_id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Resource {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
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
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ResourceCollection {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
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
    }
}
