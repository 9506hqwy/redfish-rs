pub type LogEntry = crate::log_entry::v1_16_1::LogEntry;
pub mod v1_16_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::log_entry::v1_16_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CPER {
        #[serde(skip_serializing_if = "Option::is_none", rename = "NotificationType")]
        pub notification_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SectionType")]
        pub section_type: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLEntryType {
        #[default]
        #[serde(rename = "DynamicCapacity")]
        DynamicCapacity,
        #[serde(rename = "Failure")]
        Failure,
        #[serde(rename = "Fatal")]
        Fatal,
        #[serde(rename = "Informational")]
        Informational,
        #[serde(rename = "Warning")]
        Warning,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventSeverity {
        #[default]
        #[serde(rename = "Critical")]
        Critical,
        #[serde(rename = "OK")]
        OK,
        #[serde(rename = "Warning")]
        Warning,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedLogEntries")]
        pub related_log_entries: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedLogEntries@odata.count"
        )]
        pub related_log_entries_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogDiagnosticDataTypes {
        #[default]
        #[serde(rename = "CPER")]
        CPER,
        #[serde(rename = "CPERSection")]
        CPERSection,
        #[serde(rename = "Manager")]
        Manager,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "PreOS")]
        PreOS,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LogEntry {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::log_entry::v1_16_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalDataSizeBytes"
        )]
        pub additional_data_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalDataURI")]
        pub additional_data_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CPER")]
        pub cper: Option<crate::log_entry::v1_16_0::CPER>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Created")]
        pub created: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXLEntryType")]
        pub cxl_entry_type: Option<crate::log_entry::v1_16_0::CXLEntryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiagnosticData")]
        pub diagnostic_data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiagnosticDataType")]
        pub diagnostic_data_type: Option<crate::log_entry::v1_16_0::LogDiagnosticDataTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntryCode")]
        pub entry_code: Option<crate::log_entry::v1_16_0::LogEntryCode>,
        #[serde(rename = "EntryType")]
        pub entry_type: crate::log_entry::v1_16_0::LogEntryType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventGroupId")]
        pub event_group_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventId")]
        pub event_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventTimestamp")]
        pub event_timestamp: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventType")]
        pub event_type: Option<crate::event::EventType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirstOverflowTimestamp"
        )]
        pub first_overflow_timestamp: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GeneratorId")]
        pub generator_id: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastOverflowTimestamp"
        )]
        pub last_overflow_timestamp: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::log_entry::v1_16_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Message")]
        pub message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageArgs")]
        pub message_args: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageId")]
        pub message_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Modified")]
        pub modified: Option<String>,
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
            rename = "OEMDiagnosticDataType"
        )]
        pub oem_diagnostic_data_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemLogEntryCode")]
        pub oem_log_entry_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemRecordFormat")]
        pub oem_record_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemSensorType")]
        pub oem_sensor_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Originator")]
        pub originator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginatorType")]
        pub originator_type: Option<crate::log_entry::v1_16_0::OriginatorTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OverflowErrorCount")]
        pub overflow_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Persistency")]
        pub persistency: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Resolution")]
        pub resolution: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResolutionSteps")]
        pub resolution_steps: Option<Vec<crate::resolution_step::ResolutionStep>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Resolved")]
        pub resolved: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorNumber")]
        pub sensor_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorType")]
        pub sensor_type: Option<crate::log_entry::v1_16_0::SensorType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServiceProviderNotified"
        )]
        pub service_provider_notified: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<crate::log_entry::v1_16_0::EventSeverity>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SpecificEventExistsInGroup"
        )]
        pub specific_event_exists_in_group: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogEntryCode {
        #[default]
        #[serde(rename = "Assert")]
        Assert,
        #[serde(rename = "D0 Power State")]
        D0PowerState,
        #[serde(rename = "D1 Power State")]
        D1PowerState,
        #[serde(rename = "D2 Power State")]
        D2PowerState,
        #[serde(rename = "D3 Power State")]
        D3PowerState,
        #[serde(rename = "Deassert")]
        Deassert,
        #[serde(rename = "Device Disabled")]
        DeviceDisabled,
        #[serde(rename = "Device Enabled")]
        DeviceEnabled,
        #[serde(rename = "Device Inserted / Device Present")]
        DeviceInsertedDevicePresent,
        #[serde(rename = "Device Removed / Device Absent")]
        DeviceRemovedDeviceAbsent,
        #[serde(rename = "Fully Redundant")]
        FullyRedundant,
        #[serde(rename = "Informational")]
        Informational,
        #[serde(rename = "Install Error")]
        InstallError,
        #[serde(rename = "Limit Exceeded")]
        LimitExceeded,
        #[serde(rename = "Limit Not Exceeded")]
        LimitNotExceeded,
        #[serde(rename = "Lower Critical - going high")]
        LowerCriticalGoingHigh,
        #[serde(rename = "Lower Critical - going low")]
        LowerCriticalGoingLow,
        #[serde(rename = "Lower Non-critical - going high")]
        LowerNonCriticalGoingHigh,
        #[serde(rename = "Lower Non-critical - going low")]
        LowerNonCriticalGoingLow,
        #[serde(rename = "Lower Non-recoverable - going high")]
        LowerNonRecoverableGoingHigh,
        #[serde(rename = "Lower Non-recoverable - going low")]
        LowerNonRecoverableGoingLow,
        #[serde(rename = "Monitor")]
        Monitor,
        #[serde(rename = "Non-redundant:Insufficient Resources")]
        NonRedundantInsufficientResources,
        #[serde(rename = "Non-redundant:Sufficient Resources from Insufficient Resources")]
        NonRedundantSufficientResourcesFromInsufficientResources,
        #[serde(rename = "Non-redundant:Sufficient Resources from Redundant")]
        NonRedundantSufficientResourcesFromRedundant,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Performance Lags")]
        PerformanceLags,
        #[serde(rename = "Performance Met")]
        PerformanceMet,
        #[serde(rename = "Predictive Failure asserted")]
        PredictiveFailureAsserted,
        #[serde(rename = "Predictive Failure deasserted")]
        PredictiveFailureDeasserted,
        #[serde(rename = "Redundancy Degraded")]
        RedundancyDegraded,
        #[serde(rename = "Redundancy Degraded from Fully Redundant")]
        RedundancyDegradedFromFullyRedundant,
        #[serde(rename = "Redundancy Degraded from Non-redundant")]
        RedundancyDegradedFromNonRedundant,
        #[serde(rename = "Redundancy Lost")]
        RedundancyLost,
        #[serde(rename = "State Asserted")]
        StateAsserted,
        #[serde(rename = "State Deasserted")]
        StateDeasserted,
        #[serde(rename = "Transition to Active")]
        TransitionToActive,
        #[serde(rename = "Transition to Busy")]
        TransitionToBusy,
        #[serde(rename = "Transition to Critical from Non-recoverable")]
        TransitionToCriticalFromNonRecoverable,
        #[serde(rename = "Transition to Critical from less severe")]
        TransitionToCriticalFromLessSevere,
        #[serde(rename = "Transition to Degraded")]
        TransitionToDegraded,
        #[serde(rename = "Transition to Idle")]
        TransitionToIdle,
        #[serde(rename = "Transition to In Test")]
        TransitionToInTest,
        #[serde(rename = "Transition to Non-Critical from OK")]
        TransitionToNonCriticalFromOK,
        #[serde(rename = "Transition to Non-Critical from more severe")]
        TransitionToNonCriticalFromMoreSevere,
        #[serde(rename = "Transition to Non-recoverable")]
        TransitionToNonRecoverable,
        #[serde(rename = "Transition to Non-recoverable from less severe")]
        TransitionToNonRecoverableFromLessSevere,
        #[serde(rename = "Transition to OK")]
        TransitionToOK,
        #[serde(rename = "Transition to Off Duty")]
        TransitionToOffDuty,
        #[serde(rename = "Transition to Off Line")]
        TransitionToOffLine,
        #[serde(rename = "Transition to On Line")]
        TransitionToOnLine,
        #[serde(rename = "Transition to Power Off")]
        TransitionToPowerOff,
        #[serde(rename = "Transition to Power Save")]
        TransitionToPowerSave,
        #[serde(rename = "Transition to Running")]
        TransitionToRunning,
        #[serde(rename = "Upper Critical - going high")]
        UpperCriticalGoingHigh,
        #[serde(rename = "Upper Critical - going low")]
        UpperCriticalGoingLow,
        #[serde(rename = "Upper Non-critical - going high")]
        UpperNonCriticalGoingHigh,
        #[serde(rename = "Upper Non-critical - going low")]
        UpperNonCriticalGoingLow,
        #[serde(rename = "Upper Non-recoverable - going high")]
        UpperNonRecoverableGoingHigh,
        #[serde(rename = "Upper Non-recoverable - going low")]
        UpperNonRecoverableGoingLow,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogEntryType {
        #[default]
        #[serde(rename = "CXL")]
        CXL,
        #[serde(rename = "Event")]
        Event,
        #[serde(rename = "Oem")]
        Oem,
        #[serde(rename = "SEL")]
        SEL,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OriginatorTypes {
        #[default]
        #[serde(rename = "Client")]
        Client,
        #[serde(rename = "Internal")]
        Internal,
        #[serde(rename = "SupportingService")]
        SupportingService,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorType {
        #[default]
        #[serde(rename = "Add-in Card")]
        AddInCard,
        #[serde(rename = "BaseOSBoot/InstallationStatus")]
        BaseOSBootInstallationStatus,
        #[serde(rename = "Battery")]
        Battery,
        #[serde(rename = "Boot Error")]
        BootError,
        #[serde(rename = "Button/Switch")]
        ButtonSwitch,
        #[serde(rename = "Cable/Interconnect")]
        CableInterconnect,
        #[serde(rename = "Chassis")]
        Chassis,
        #[serde(rename = "ChipSet")]
        ChipSet,
        #[serde(rename = "CoolingDevice")]
        CoolingDevice,
        #[serde(rename = "Critical Interrupt")]
        CriticalInterrupt,
        #[serde(rename = "Current")]
        Current,
        #[serde(rename = "Drive Slot/Bay")]
        DriveSlotBay,
        #[serde(rename = "Entity Presence")]
        EntityPresence,
        #[serde(rename = "Event Logging Disabled")]
        EventLoggingDisabled,
        #[serde(rename = "FRUState")]
        FRUState,
        #[serde(rename = "Fan")]
        Fan,
        #[serde(rename = "LAN")]
        LAN,
        #[serde(rename = "Management Subsystem Health")]
        ManagementSubsystemHealth,
        #[serde(rename = "Memory")]
        Memory,
        #[serde(rename = "Microcontroller/Coprocessor")]
        MicrocontrollerCoprocessor,
        #[serde(rename = "Module/Board")]
        ModuleBoard,
        #[serde(rename = "Monitor ASIC/IC")]
        MonitorASICIC,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OS Stop/Shutdown")]
        OSStopShutdown,
        #[serde(rename = "Other FRU")]
        OtherFRU,
        #[serde(rename = "Other Units-based Sensor")]
        OtherUnitsBasedSensor,
        #[serde(rename = "POST Memory Resize")]
        POSTMemoryResize,
        #[serde(rename = "Physical Chassis Security")]
        PhysicalChassisSecurity,
        #[serde(rename = "Platform Alert")]
        PlatformAlert,
        #[serde(rename = "Platform Security Violation Attempt")]
        PlatformSecurityViolationAttempt,
        #[serde(rename = "Power Supply / Converter")]
        PowerSupplyConverter,
        #[serde(rename = "PowerUnit")]
        PowerUnit,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "Session Audit")]
        SessionAudit,
        #[serde(rename = "Slot/Connector")]
        SlotConnector,
        #[serde(rename = "System ACPI PowerState")]
        SystemACPIPowerState,
        #[serde(rename = "System Event")]
        SystemEvent,
        #[serde(rename = "System Firmware Progress")]
        SystemFirmwareProgress,
        #[serde(rename = "SystemBoot/Restart")]
        SystemBootRestart,
        #[serde(rename = "Temperature")]
        Temperature,
        #[serde(rename = "Terminator")]
        Terminator,
        #[serde(rename = "Version Change")]
        VersionChange,
        #[serde(rename = "Voltage")]
        Voltage,
        #[serde(rename = "Watchdog")]
        Watchdog,
    }
}
pub mod v1_16_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::log_entry::v1_16_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CPER {
        #[serde(skip_serializing_if = "Option::is_none", rename = "NotificationType")]
        pub notification_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SectionType")]
        pub section_type: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum CXLEntryType {
        #[default]
        #[serde(rename = "DynamicCapacity")]
        DynamicCapacity,
        #[serde(rename = "Failure")]
        Failure,
        #[serde(rename = "Fatal")]
        Fatal,
        #[serde(rename = "Informational")]
        Informational,
        #[serde(rename = "Warning")]
        Warning,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum EventSeverity {
        #[default]
        #[serde(rename = "Critical")]
        Critical,
        #[serde(rename = "OK")]
        OK,
        #[serde(rename = "Warning")]
        Warning,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginOfCondition")]
        pub origin_of_condition: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedItem")]
        pub related_item: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedItem@odata.count"
        )]
        pub related_item_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RelatedLogEntries")]
        pub related_log_entries: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RelatedLogEntries@odata.count"
        )]
        pub related_log_entries_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogDiagnosticDataTypes {
        #[default]
        #[serde(rename = "CPER")]
        CPER,
        #[serde(rename = "CPERSection")]
        CPERSection,
        #[serde(rename = "Manager")]
        Manager,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OS")]
        OS,
        #[serde(rename = "PreOS")]
        PreOS,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LogEntry {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::log_entry::v1_16_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AdditionalDataSizeBytes"
        )]
        pub additional_data_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalDataURI")]
        pub additional_data_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CPER")]
        pub cper: Option<crate::log_entry::v1_16_1::CPER>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Created")]
        pub created: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXLEntryType")]
        pub cxl_entry_type: Option<crate::log_entry::v1_16_1::CXLEntryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::log_entry::v1_16_1::LogEntryDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiagnosticData")]
        pub diagnostic_data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiagnosticDataType")]
        pub diagnostic_data_type: Option<crate::log_entry::v1_16_1::LogEntryDiagnosticDataType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EntryCode")]
        pub entry_code: Option<crate::log_entry::v1_16_1::LogEntryEntryCode>,
        #[serde(rename = "EntryType")]
        pub entry_type: crate::log_entry::v1_16_1::LogEntryType,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventGroupId")]
        pub event_group_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventId")]
        pub event_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventTimestamp")]
        pub event_timestamp: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EventType")]
        pub event_type: Option<crate::event::EventType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "FirstOverflowTimestamp"
        )]
        pub first_overflow_timestamp: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GeneratorId")]
        pub generator_id: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastOverflowTimestamp"
        )]
        pub last_overflow_timestamp: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::log_entry::v1_16_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Message")]
        pub message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageArgs")]
        pub message_args: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MessageId")]
        pub message_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Modified")]
        pub modified: Option<String>,
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
            rename = "OEMDiagnosticDataType"
        )]
        pub oem_diagnostic_data_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemLogEntryCode")]
        pub oem_log_entry_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemRecordFormat")]
        pub oem_record_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OemSensorType")]
        pub oem_sensor_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Originator")]
        pub originator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OriginatorType")]
        pub originator_type: Option<crate::log_entry::v1_16_1::OriginatorTypes>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OverflowErrorCount")]
        pub overflow_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Persistency")]
        pub persistency: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Resolution")]
        pub resolution: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ResolutionSteps")]
        pub resolution_steps: Option<Vec<crate::resolution_step::ResolutionStep>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Resolved")]
        pub resolved: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorNumber")]
        pub sensor_number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SensorType")]
        pub sensor_type: Option<crate::log_entry::v1_16_1::LogEntrySensorType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ServiceProviderNotified"
        )]
        pub service_provider_notified: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Severity")]
        pub severity: Option<crate::log_entry::v1_16_1::LogEntrySeverity>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SpecificEventExistsInGroup"
        )]
        pub specific_event_exists_in_group: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogEntryCode {
        #[default]
        #[serde(rename = "Assert")]
        Assert,
        #[serde(rename = "D0 Power State")]
        D0PowerState,
        #[serde(rename = "D1 Power State")]
        D1PowerState,
        #[serde(rename = "D2 Power State")]
        D2PowerState,
        #[serde(rename = "D3 Power State")]
        D3PowerState,
        #[serde(rename = "Deassert")]
        Deassert,
        #[serde(rename = "Device Disabled")]
        DeviceDisabled,
        #[serde(rename = "Device Enabled")]
        DeviceEnabled,
        #[serde(rename = "Device Inserted / Device Present")]
        DeviceInsertedDevicePresent,
        #[serde(rename = "Device Removed / Device Absent")]
        DeviceRemovedDeviceAbsent,
        #[serde(rename = "Fully Redundant")]
        FullyRedundant,
        #[serde(rename = "Informational")]
        Informational,
        #[serde(rename = "Install Error")]
        InstallError,
        #[serde(rename = "Limit Exceeded")]
        LimitExceeded,
        #[serde(rename = "Limit Not Exceeded")]
        LimitNotExceeded,
        #[serde(rename = "Lower Critical - going high")]
        LowerCriticalGoingHigh,
        #[serde(rename = "Lower Critical - going low")]
        LowerCriticalGoingLow,
        #[serde(rename = "Lower Non-critical - going high")]
        LowerNonCriticalGoingHigh,
        #[serde(rename = "Lower Non-critical - going low")]
        LowerNonCriticalGoingLow,
        #[serde(rename = "Lower Non-recoverable - going high")]
        LowerNonRecoverableGoingHigh,
        #[serde(rename = "Lower Non-recoverable - going low")]
        LowerNonRecoverableGoingLow,
        #[serde(rename = "Monitor")]
        Monitor,
        #[serde(rename = "Non-redundant:Insufficient Resources")]
        NonRedundantInsufficientResources,
        #[serde(rename = "Non-redundant:Sufficient Resources from Insufficient Resources")]
        NonRedundantSufficientResourcesFromInsufficientResources,
        #[serde(rename = "Non-redundant:Sufficient Resources from Redundant")]
        NonRedundantSufficientResourcesFromRedundant,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Performance Lags")]
        PerformanceLags,
        #[serde(rename = "Performance Met")]
        PerformanceMet,
        #[serde(rename = "Predictive Failure asserted")]
        PredictiveFailureAsserted,
        #[serde(rename = "Predictive Failure deasserted")]
        PredictiveFailureDeasserted,
        #[serde(rename = "Redundancy Degraded")]
        RedundancyDegraded,
        #[serde(rename = "Redundancy Degraded from Fully Redundant")]
        RedundancyDegradedFromFullyRedundant,
        #[serde(rename = "Redundancy Degraded from Non-redundant")]
        RedundancyDegradedFromNonRedundant,
        #[serde(rename = "Redundancy Lost")]
        RedundancyLost,
        #[serde(rename = "State Asserted")]
        StateAsserted,
        #[serde(rename = "State Deasserted")]
        StateDeasserted,
        #[serde(rename = "Transition to Active")]
        TransitionToActive,
        #[serde(rename = "Transition to Busy")]
        TransitionToBusy,
        #[serde(rename = "Transition to Critical from Non-recoverable")]
        TransitionToCriticalFromNonRecoverable,
        #[serde(rename = "Transition to Critical from less severe")]
        TransitionToCriticalFromLessSevere,
        #[serde(rename = "Transition to Degraded")]
        TransitionToDegraded,
        #[serde(rename = "Transition to Idle")]
        TransitionToIdle,
        #[serde(rename = "Transition to In Test")]
        TransitionToInTest,
        #[serde(rename = "Transition to Non-Critical from OK")]
        TransitionToNonCriticalFromOK,
        #[serde(rename = "Transition to Non-Critical from more severe")]
        TransitionToNonCriticalFromMoreSevere,
        #[serde(rename = "Transition to Non-recoverable")]
        TransitionToNonRecoverable,
        #[serde(rename = "Transition to Non-recoverable from less severe")]
        TransitionToNonRecoverableFromLessSevere,
        #[serde(rename = "Transition to OK")]
        TransitionToOK,
        #[serde(rename = "Transition to Off Duty")]
        TransitionToOffDuty,
        #[serde(rename = "Transition to Off Line")]
        TransitionToOffLine,
        #[serde(rename = "Transition to On Line")]
        TransitionToOnLine,
        #[serde(rename = "Transition to Power Off")]
        TransitionToPowerOff,
        #[serde(rename = "Transition to Power Save")]
        TransitionToPowerSave,
        #[serde(rename = "Transition to Running")]
        TransitionToRunning,
        #[serde(rename = "Upper Critical - going high")]
        UpperCriticalGoingHigh,
        #[serde(rename = "Upper Critical - going low")]
        UpperCriticalGoingLow,
        #[serde(rename = "Upper Non-critical - going high")]
        UpperNonCriticalGoingHigh,
        #[serde(rename = "Upper Non-critical - going low")]
        UpperNonCriticalGoingLow,
        #[serde(rename = "Upper Non-recoverable - going high")]
        UpperNonRecoverableGoingHigh,
        #[serde(rename = "Upper Non-recoverable - going low")]
        UpperNonRecoverableGoingLow,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogEntryDescription {
        V000001(crate::log_entry::v1_16_1::LogEntryDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for LogEntryDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogEntryDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogEntryDiagnosticDataType {
        V011601(crate::log_entry::v1_16_1::LogDiagnosticDataTypes),
        V000001(crate::log_entry::v1_16_1::LogEntryDiagnosticDataTypeN1),
    }
    impl Default for LogEntryDiagnosticDataType {
        fn default() -> Self {
            Self::V011601(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogEntryDiagnosticDataTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogEntryEntryCode {
        V011601(crate::log_entry::v1_16_1::LogEntryCode),
        V000001(crate::log_entry::v1_16_1::LogEntryEntryCodeN1),
    }
    impl Default for LogEntryEntryCode {
        fn default() -> Self {
            Self::V011601(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogEntryEntryCodeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogEntrySensorType {
        V011601(crate::log_entry::v1_16_1::SensorType),
        V000001(crate::log_entry::v1_16_1::LogEntrySensorTypeN1),
    }
    impl Default for LogEntrySensorType {
        fn default() -> Self {
            Self::V011601(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogEntrySensorTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogEntrySeverity {
        V011601(crate::log_entry::v1_16_1::EventSeverity),
        V000001(crate::log_entry::v1_16_1::LogEntrySeverityN1),
    }
    impl Default for LogEntrySeverity {
        fn default() -> Self {
            Self::V011601(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogEntrySeverityN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogEntryType {
        #[default]
        #[serde(rename = "CXL")]
        CXL,
        #[serde(rename = "Event")]
        Event,
        #[serde(rename = "Oem")]
        Oem,
        #[serde(rename = "SEL")]
        SEL,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OriginatorTypes {
        #[default]
        #[serde(rename = "Client")]
        Client,
        #[serde(rename = "Internal")]
        Internal,
        #[serde(rename = "SupportingService")]
        SupportingService,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SensorType {
        #[default]
        #[serde(rename = "Add-in Card")]
        AddInCard,
        #[serde(rename = "BaseOSBoot/InstallationStatus")]
        BaseOSBootInstallationStatus,
        #[serde(rename = "Battery")]
        Battery,
        #[serde(rename = "Boot Error")]
        BootError,
        #[serde(rename = "Button/Switch")]
        ButtonSwitch,
        #[serde(rename = "Cable/Interconnect")]
        CableInterconnect,
        #[serde(rename = "Chassis")]
        Chassis,
        #[serde(rename = "ChipSet")]
        ChipSet,
        #[serde(rename = "CoolingDevice")]
        CoolingDevice,
        #[serde(rename = "Critical Interrupt")]
        CriticalInterrupt,
        #[serde(rename = "Current")]
        Current,
        #[serde(rename = "Drive Slot/Bay")]
        DriveSlotBay,
        #[serde(rename = "Entity Presence")]
        EntityPresence,
        #[serde(rename = "Event Logging Disabled")]
        EventLoggingDisabled,
        #[serde(rename = "FRUState")]
        FRUState,
        #[serde(rename = "Fan")]
        Fan,
        #[serde(rename = "LAN")]
        LAN,
        #[serde(rename = "Management Subsystem Health")]
        ManagementSubsystemHealth,
        #[serde(rename = "Memory")]
        Memory,
        #[serde(rename = "Microcontroller/Coprocessor")]
        MicrocontrollerCoprocessor,
        #[serde(rename = "Module/Board")]
        ModuleBoard,
        #[serde(rename = "Monitor ASIC/IC")]
        MonitorASICIC,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "OS Stop/Shutdown")]
        OSStopShutdown,
        #[serde(rename = "Other FRU")]
        OtherFRU,
        #[serde(rename = "Other Units-based Sensor")]
        OtherUnitsBasedSensor,
        #[serde(rename = "POST Memory Resize")]
        POSTMemoryResize,
        #[serde(rename = "Physical Chassis Security")]
        PhysicalChassisSecurity,
        #[serde(rename = "Platform Alert")]
        PlatformAlert,
        #[serde(rename = "Platform Security Violation Attempt")]
        PlatformSecurityViolationAttempt,
        #[serde(rename = "Power Supply / Converter")]
        PowerSupplyConverter,
        #[serde(rename = "PowerUnit")]
        PowerUnit,
        #[serde(rename = "Processor")]
        Processor,
        #[serde(rename = "Session Audit")]
        SessionAudit,
        #[serde(rename = "Slot/Connector")]
        SlotConnector,
        #[serde(rename = "System ACPI PowerState")]
        SystemACPIPowerState,
        #[serde(rename = "System Event")]
        SystemEvent,
        #[serde(rename = "System Firmware Progress")]
        SystemFirmwareProgress,
        #[serde(rename = "SystemBoot/Restart")]
        SystemBootRestart,
        #[serde(rename = "Temperature")]
        Temperature,
        #[serde(rename = "Terminator")]
        Terminator,
        #[serde(rename = "Version Change")]
        VersionChange,
        #[serde(rename = "Voltage")]
        Voltage,
        #[serde(rename = "Watchdog")]
        Watchdog,
    }
}
