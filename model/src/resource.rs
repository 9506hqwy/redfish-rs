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
    #[serde(skip_serializing_if = "Option::is_none", rename = "ResolutionSteps")]
    pub resolution_steps: Option<Vec<crate::resolution_step::ResolutionStep>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
    pub severity: Option<crate::resource::Health>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Timestamp")]
    pub timestamp: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "UserAuthenticationSource"
    )]
    pub user_authentication_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
    pub username: Option<String>,
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
pub type Identifier = crate::resource::v1_20_0::Identifier;
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
pub type Item = crate::resource::Resource;
pub type ItemOrCollection = crate::resource::ResourceCollection;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Links {
    #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
    pub oem: Option<crate::resource::Oem>,
}
pub type Location = crate::resource::v1_20_0::Location;
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
pub type ReferenceableMember = crate::resource::v1_20_0::ReferenceableMember;
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
pub type Resource = crate::resource::v1_20_0::Resource;
pub type ResourceCollection = crate::resource::v1_20_0::ResourceCollection;
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum State {
    #[default]
    #[serde(rename = "Absent")]
    Absent,
    #[serde(rename = "Deferring")]
    Deferring,
    #[serde(rename = "Degraded")]
    Degraded,
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
    pub conditions: Option<Vec<crate::resource::StatusConditions>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Health")]
    pub health: Option<crate::resource::StatusHealth>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "HealthRollup")]
    pub health_rollup: Option<crate::resource::StatusHealthRollup>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
    pub oem: Option<crate::resource::Oem>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "State")]
    pub state: Option<crate::resource::StatusState>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum StatusConditions {
    V000001(crate::resource::StatusConditionsN1),
    ResourceCondition(crate::resource::Condition),
}
impl Default for StatusConditions {
    fn default() -> Self {
        Self::V000001(Default::default())
    }
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum StatusConditionsN1 {
    #[default]
    #[serde(rename = "null")]
    Null,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum StatusHealth {
    V000001(crate::resource::StatusHealthN1),
    ResourceHealth(crate::resource::Health),
}
impl Default for StatusHealth {
    fn default() -> Self {
        Self::V000001(Default::default())
    }
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum StatusHealthN1 {
    #[default]
    #[serde(rename = "null")]
    Null,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum StatusHealthRollup {
    V000001(crate::resource::StatusHealthRollupN1),
    ResourceHealth(crate::resource::Health),
}
impl Default for StatusHealthRollup {
    fn default() -> Self {
        Self::V000001(Default::default())
    }
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum StatusHealthRollupN1 {
    #[default]
    #[serde(rename = "null")]
    Null,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum StatusState {
    V000001(crate::resource::StatusStateN1),
    ResourceState(crate::resource::State),
}
impl Default for StatusState {
    fn default() -> Self {
        Self::V000001(Default::default())
    }
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum StatusStateN1 {
    #[default]
    #[serde(rename = "null")]
    Null,
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
pub mod v1_20_0 {
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
        pub durable_name_format: Option<crate::resource::v1_20_0::IdentifierDurableNameFormat>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum IdentifierDurableNameFormat {
        V012000(crate::resource::v1_20_0::DurableNameFormat),
        V000001(crate::resource::v1_20_0::IdentifierDurableNameFormatN1),
    }
    impl Default for IdentifierDurableNameFormat {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum IdentifierDurableNameFormatN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Location {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AltitudeMeters")]
        pub altitude_meters: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Contacts")]
        pub contacts: Option<Vec<crate::resource::v1_20_0::LocationContacts>>,
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
        pub part_location: Option<crate::resource::v1_20_0::PartLocation>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PartLocationContext"
        )]
        pub part_location_context: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PhysicalAddress")]
        pub physical_address: Option<crate::resource::v1_20_0::PhysicalAddress>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Placement")]
        pub placement: Option<crate::resource::v1_20_0::Placement>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalAddress")]
        pub postal_address: Option<crate::resource::v1_20_0::PostalAddress>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LocationContacts {
        V012000(crate::resource::v1_20_0::ContactInfo),
        V000001(crate::resource::v1_20_0::LocationContactsN1),
    }
    impl Default for LocationContacts {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LocationContactsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub location_type: Option<crate::resource::v1_20_0::PartLocationLocationType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
        pub orientation: Option<crate::resource::v1_20_0::PartLocationOrientation>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Reference")]
        pub reference: Option<crate::resource::v1_20_0::PartLocationReference>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceLabel")]
        pub service_label: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PartLocationLocationType {
        V012000(crate::resource::v1_20_0::LocationType),
        V000001(crate::resource::v1_20_0::PartLocationLocationTypeN1),
    }
    impl Default for PartLocationLocationType {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PartLocationLocationTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PartLocationOrientation {
        V012000(crate::resource::v1_20_0::Orientation),
        V000001(crate::resource::v1_20_0::PartLocationOrientationN1),
    }
    impl Default for PartLocationOrientation {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PartLocationOrientationN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PartLocationReference {
        V012000(crate::resource::v1_20_0::Reference),
        V000001(crate::resource::v1_20_0::PartLocationReferenceN1),
    }
    impl Default for PartLocationReference {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PartLocationReferenceN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PhysicalAddress {
        #[serde(skip_serializing_if = "Option::is_none", rename = "City")]
        pub city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Country")]
        pub country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ISOCountryCode")]
        pub iso_country_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ISOSubdivisionCode")]
        pub iso_subdivision_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PostalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StateOrProvince")]
        pub state_or_province: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "StreetAddress")]
        pub street_address: Option<String>,
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
        pub rack_offset_units: Option<crate::resource::v1_20_0::PlacementRackOffsetUnits>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Row")]
        pub row: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PlacementRackOffsetUnits {
        V012000(crate::resource::v1_20_0::RackUnits),
        V000001(crate::resource::v1_20_0::PlacementRackOffsetUnitsN1),
    }
    impl Default for PlacementRackOffsetUnits {
        fn default() -> Self {
            Self::V012000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum PlacementRackOffsetUnitsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
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
        pub description: Option<crate::resource::v1_20_0::ResourceDescription>,
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
        pub description: Option<crate::resource::v1_20_0::ResourceCollectionDescription>,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ResourceCollectionDescription {
        V000001(crate::resource::v1_20_0::ResourceCollectionDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ResourceCollectionDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResourceCollectionDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ResourceDescription {
        V000001(crate::resource::v1_20_0::ResourceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for ResourceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ResourceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
