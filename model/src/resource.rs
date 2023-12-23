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
    pub severity: Option<String>,
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
    ResourceV1N10N5Identifier(crate::resource::v1_10_5::Identifier),
    ResourceV1N11N4Identifier(crate::resource::v1_11_4::Identifier),
    ResourceV1N12N3Identifier(crate::resource::v1_12_3::Identifier),
    ResourceV1N13N2Identifier(crate::resource::v1_13_2::Identifier),
    ResourceV1N14N1Identifier(crate::resource::v1_14_1::Identifier),
    ResourceV1N15N0Identifier(crate::resource::v1_15_0::Identifier),
    ResourceV1N16N0Identifier(crate::resource::v1_16_0::Identifier),
    ResourceV1N1N15Identifier(crate::resource::v1_1_15::Identifier),
    ResourceV1N2N14Identifier(crate::resource::v1_2_14::Identifier),
    ResourceV1N3N13Identifier(crate::resource::v1_3_13::Identifier),
    ResourceV1N4N12Identifier(crate::resource::v1_4_12::Identifier),
    ResourceV1N5N11Identifier(crate::resource::v1_5_11::Identifier),
    ResourceV1N6N11Identifier(crate::resource::v1_6_11::Identifier),
    ResourceV1N7N10Identifier(crate::resource::v1_7_10::Identifier),
    ResourceV1N8N10Identifier(crate::resource::v1_8_10::Identifier),
    ResourceV1N9N8Identifier(crate::resource::v1_9_8::Identifier),
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
pub enum Location {
    ResourceV1N10N5Location(crate::resource::v1_10_5::Location),
    ResourceV1N11N4Location(crate::resource::v1_11_4::Location),
    ResourceV1N12N3Location(crate::resource::v1_12_3::Location),
    ResourceV1N13N2Location(crate::resource::v1_13_2::Location),
    ResourceV1N14N1Location(crate::resource::v1_14_1::Location),
    ResourceV1N15N0Location(crate::resource::v1_15_0::Location),
    ResourceV1N16N0Location(crate::resource::v1_16_0::Location),
    ResourceV1N1N15Location(crate::resource::v1_1_15::Location),
    ResourceV1N2N14Location(crate::resource::v1_2_14::Location),
    ResourceV1N3N13Location(crate::resource::v1_3_13::Location),
    ResourceV1N4N12Location(crate::resource::v1_4_12::Location),
    ResourceV1N5N11Location(crate::resource::v1_5_11::Location),
    ResourceV1N6N11Location(crate::resource::v1_6_11::Location),
    ResourceV1N7N10Location(crate::resource::v1_7_10::Location),
    ResourceV1N8N10Location(crate::resource::v1_8_10::Location),
    ResourceV1N9N8Location(crate::resource::v1_9_8::Location),
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Oem {}
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
    ResourceV1N0N13Resource(crate::resource::v1_0_13::Resource),
    ResourceV1N10N5Resource(crate::resource::v1_10_5::Resource),
    ResourceV1N11N4Resource(crate::resource::v1_11_4::Resource),
    ResourceV1N12N3Resource(crate::resource::v1_12_3::Resource),
    ResourceV1N13N2Resource(crate::resource::v1_13_2::Resource),
    ResourceV1N14N1Resource(crate::resource::v1_14_1::Resource),
    ResourceV1N15N0Resource(crate::resource::v1_15_0::Resource),
    ResourceV1N16N0Resource(crate::resource::v1_16_0::Resource),
    ResourceV1N1N15Resource(crate::resource::v1_1_15::Resource),
    ResourceV1N2N14Resource(crate::resource::v1_2_14::Resource),
    ResourceV1N3N13Resource(crate::resource::v1_3_13::Resource),
    ResourceV1N4N12Resource(crate::resource::v1_4_12::Resource),
    ResourceV1N5N11Resource(crate::resource::v1_5_11::Resource),
    ResourceV1N6N11Resource(crate::resource::v1_6_11::Resource),
    ResourceV1N7N10Resource(crate::resource::v1_7_10::Resource),
    ResourceV1N8N10Resource(crate::resource::v1_8_10::Resource),
    ResourceV1N9N8Resource(crate::resource::v1_9_8::Resource),
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
    pub health: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "HealthRollup")]
    pub health_rollup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
    pub oem: Option<crate::resource::Oem>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "State")]
    pub state: Option<String>,
}
pub mod v1_0_13 {
    use serde::{Deserialize, Serialize};
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
        pub durable_name_format: Option<String>,
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
        pub location_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<String>,
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
        pub rack_offset_units: Option<String>,
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
        pub durable_name_format: Option<String>,
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
        pub location_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<String>,
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
        pub rack_offset_units: Option<String>,
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
        pub durable_name_format: Option<String>,
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
        pub location_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<String>,
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
        pub rack_offset_units: Option<String>,
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
        pub durable_name_format: Option<String>,
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
        pub location_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<String>,
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
        pub rack_offset_units: Option<String>,
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
        pub durable_name_format: Option<String>,
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
        pub location_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<String>,
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
        pub rack_offset_units: Option<String>,
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
        pub durable_name_format: Option<String>,
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
        pub location_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<String>,
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
        pub rack_offset_units: Option<String>,
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
        pub durable_name_format: Option<String>,
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
        pub location_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<String>,
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
        pub rack_offset_units: Option<String>,
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
        pub durable_name_format: Option<String>,
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
        pub durable_name_format: Option<String>,
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
}
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
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
        pub rack_offset_units: Option<String>,
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
    pub enum RackUnits {
        #[default]
        #[serde(rename = "EIA_310")]
        EIAN310,
        #[serde(rename = "OpenU")]
        OpenU,
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
        pub durable_name_format: Option<String>,
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
        pub rack_offset_units: Option<String>,
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
        pub durable_name_format: Option<String>,
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
        pub rack_offset_units: Option<String>,
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
        pub durable_name_format: Option<String>,
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
        pub location_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<String>,
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
        pub rack_offset_units: Option<String>,
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
        pub durable_name_format: Option<String>,
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
        pub location_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<String>,
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
        pub rack_offset_units: Option<String>,
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
        pub durable_name_format: Option<String>,
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
        pub location_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<String>,
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
        pub rack_offset_units: Option<String>,
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
        pub durable_name_format: Option<String>,
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
        pub location_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<String>,
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
        pub rack_offset_units: Option<String>,
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
        pub durable_name_format: Option<String>,
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
        pub location_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<String>,
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
        pub rack_offset_units: Option<String>,
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
}
