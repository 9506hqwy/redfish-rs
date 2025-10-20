pub type LogService = crate::log_service::v1_8_1::LogService;
pub mod v1_8_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#LogService.ClearLog"
        )]
        pub log_service_clear_log: Option<crate::log_service::v1_8_0::ClearLog>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#LogService.CollectDiagnosticData"
        )]
        pub log_service_collect_diagnostic_data:
            Option<crate::log_service::v1_8_0::CollectDiagnosticData>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#LogService.PushDiagnosticData"
        )]
        pub log_service_push_diagnostic_data:
            Option<crate::log_service::v1_8_0::PushDiagnosticData>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::log_service::v1_8_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutoClearResolvedEntries {
        #[default]
        #[serde(rename = "ClearEventGroup")]
        ClearEventGroup,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "RetainCauseResolutionEntries")]
        RetainCauseResolutionEntries,
        #[serde(rename = "UpdateCauseEntry")]
        UpdateCauseEntry,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearLog {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearLogRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogEntriesETag")]
        pub log_entries_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CollectDiagnosticData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CollectDiagnosticDataRequestBody {
        #[serde(rename = "DiagnosticDataType")]
        pub diagnostic_data_type: crate::log_service::v1_8_0::LogDiagnosticDataTypes,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OEMDiagnosticDataType"
        )]
        pub oem_diagnostic_data_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetDevice")]
        pub target_device: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetURI")]
        pub target_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::log_service::v1_8_0::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DiagnosticDataDetails {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiagnosticDataType")]
        pub diagnostic_data_type:
            Option<crate::log_service::v1_8_0::DiagnosticDataDetailsDiagnosticDataType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EstimatedDuration")]
        pub estimated_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EstimatedSizeBytes")]
        pub estimated_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OEMDiagnosticDataType"
        )]
        pub oem_diagnostic_data_type: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DiagnosticDataDetailsDiagnosticDataType {
        V010800(crate::log_service::v1_8_0::LogDiagnosticDataTypes),
        V000001(crate::log_service::v1_8_0::DiagnosticDataDetailsDiagnosticDataTypeN1),
    }
    impl Default for DiagnosticDataDetailsDiagnosticDataType {
        fn default() -> Self {
            Self::V010800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DiagnosticDataDetailsDiagnosticDataTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogDiagnosticDataTypes {
        #[default]
        #[serde(rename = "Device")]
        Device,
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
    pub enum LogEntryTypes {
        #[default]
        #[serde(rename = "CXL")]
        CXL,
        #[serde(rename = "Event")]
        Event,
        #[serde(rename = "Multiple")]
        Multiple,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SEL")]
        SEL,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogPurpose {
        #[default]
        #[serde(rename = "Diagnostic")]
        Diagnostic,
        #[serde(rename = "ExternalEntity")]
        ExternalEntity,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Operations")]
        Operations,
        #[serde(rename = "Security")]
        Security,
        #[serde(rename = "Telemetry")]
        Telemetry,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LogService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::log_service::v1_8_0::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutoClearResolvedEntries"
        )]
        pub auto_clear_resolved_entries:
            Option<crate::log_service::v1_8_0::LogServiceAutoClearResolvedEntries>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AutoDSTEnabled")]
        pub auto_dst_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DateTime")]
        pub date_time: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DateTimeLocalOffset"
        )]
        pub date_time_local_offset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::log_service::v1_8_0::LogServiceDescription>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DiagnosticDataDetails"
        )]
        pub diagnostic_data_details:
            Option<Vec<crate::log_service::v1_8_0::LogServiceDiagnosticDataDetails>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Entries")]
        pub entries: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogEntryType")]
        pub log_entry_type: Option<crate::log_service::v1_8_0::LogServiceLogEntryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogPurposes")]
        pub log_purposes: Option<Vec<crate::log_service::v1_8_0::LogServiceLogPurposes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxNumberOfRecords")]
        pub max_number_of_records: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OEMLogPurpose")]
        pub oem_log_purpose: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OverWritePolicy")]
        pub over_write_policy: Option<crate::log_service::v1_8_0::OverWritePolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Overflow")]
        pub overflow: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Persistency")]
        pub persistency: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyslogFilters")]
        pub syslog_filters: Option<Vec<crate::log_service::v1_8_0::LogServiceSyslogFilters>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogServiceAutoClearResolvedEntries {
        V010800(crate::log_service::v1_8_0::AutoClearResolvedEntries),
        V000001(crate::log_service::v1_8_0::LogServiceAutoClearResolvedEntriesN1),
    }
    impl Default for LogServiceAutoClearResolvedEntries {
        fn default() -> Self {
            Self::V010800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogServiceAutoClearResolvedEntriesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogServiceDescription {
        V000001(crate::log_service::v1_8_0::LogServiceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for LogServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogServiceDiagnosticDataDetails {
        V010800(crate::log_service::v1_8_0::DiagnosticDataDetails),
        V000001(crate::log_service::v1_8_0::LogServiceDiagnosticDataDetailsN1),
    }
    impl Default for LogServiceDiagnosticDataDetails {
        fn default() -> Self {
            Self::V010800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogServiceDiagnosticDataDetailsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogServiceLogEntryType {
        V010800(crate::log_service::v1_8_0::LogEntryTypes),
        V000001(crate::log_service::v1_8_0::LogServiceLogEntryTypeN1),
    }
    impl Default for LogServiceLogEntryType {
        fn default() -> Self {
            Self::V010800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogServiceLogEntryTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogServiceLogPurposes {
        V010800(crate::log_service::v1_8_0::LogPurpose),
        V000001(crate::log_service::v1_8_0::LogServiceLogPurposesN1),
    }
    impl Default for LogServiceLogPurposes {
        fn default() -> Self {
            Self::V010800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogServiceLogPurposesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogServiceSyslogFilters {
        V010800(crate::log_service::v1_8_0::SyslogFilter),
        V000001(crate::log_service::v1_8_0::LogServiceSyslogFiltersN1),
    }
    impl Default for LogServiceSyslogFilters {
        fn default() -> Self {
            Self::V010800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogServiceSyslogFiltersN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OverWritePolicy {
        #[default]
        #[serde(rename = "NeverOverWrites")]
        NeverOverWrites,
        #[serde(rename = "Unknown")]
        Unknown,
        #[serde(rename = "WrapsWhenFull")]
        WrapsWhenFull,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PushDiagnosticData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PushDiagnosticDataRequestBody {
        #[serde(rename = "AdditionalDataURI")]
        pub additional_data_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(rename = "TargetURI")]
        pub target_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::log_service::v1_8_0::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogFacility {
        #[default]
        #[serde(rename = "Auth")]
        Auth,
        #[serde(rename = "Authpriv")]
        Authpriv,
        #[serde(rename = "Console")]
        Console,
        #[serde(rename = "Cron")]
        Cron,
        #[serde(rename = "Daemon")]
        Daemon,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "Kern")]
        Kern,
        #[serde(rename = "LPR")]
        LPR,
        #[serde(rename = "Local0")]
        Local0,
        #[serde(rename = "Local1")]
        Local1,
        #[serde(rename = "Local2")]
        Local2,
        #[serde(rename = "Local3")]
        Local3,
        #[serde(rename = "Local4")]
        Local4,
        #[serde(rename = "Local5")]
        Local5,
        #[serde(rename = "Local6")]
        Local6,
        #[serde(rename = "Local7")]
        Local7,
        #[serde(rename = "Mail")]
        Mail,
        #[serde(rename = "NTP")]
        NTP,
        #[serde(rename = "News")]
        News,
        #[serde(rename = "Security")]
        Security,
        #[serde(rename = "SolarisCron")]
        SolarisCron,
        #[serde(rename = "Syslog")]
        Syslog,
        #[serde(rename = "UUCP")]
        UUCP,
        #[serde(rename = "User")]
        User,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SyslogFilter {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogFacilities")]
        pub log_facilities: Option<Vec<crate::log_service::v1_8_0::SyslogFilterLogFacilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestSeverity")]
        pub lowest_severity: Option<crate::log_service::v1_8_0::SyslogFilterLowestSeverity>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SyslogFilterLogFacilities {
        V010800(crate::log_service::v1_8_0::SyslogFacility),
        V000001(crate::log_service::v1_8_0::SyslogFilterLogFacilitiesN1),
    }
    impl Default for SyslogFilterLogFacilities {
        fn default() -> Self {
            Self::V010800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogFilterLogFacilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SyslogFilterLowestSeverity {
        V010800(crate::log_service::v1_8_0::SyslogSeverity),
        V000001(crate::log_service::v1_8_0::SyslogFilterLowestSeverityN1),
    }
    impl Default for SyslogFilterLowestSeverity {
        fn default() -> Self {
            Self::V010800(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogFilterLowestSeverityN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogSeverity {
        #[default]
        #[serde(rename = "Alert")]
        Alert,
        #[serde(rename = "All")]
        All,
        #[serde(rename = "Critical")]
        Critical,
        #[serde(rename = "Debug")]
        Debug,
        #[serde(rename = "Emergency")]
        Emergency,
        #[serde(rename = "Error")]
        Error,
        #[serde(rename = "Informational")]
        Informational,
        #[serde(rename = "Notice")]
        Notice,
        #[serde(rename = "Warning")]
        Warning,
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
}
pub mod v1_8_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#LogService.ClearLog"
        )]
        pub log_service_clear_log: Option<crate::log_service::v1_8_1::ClearLog>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#LogService.CollectDiagnosticData"
        )]
        pub log_service_collect_diagnostic_data:
            Option<crate::log_service::v1_8_1::CollectDiagnosticData>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#LogService.PushDiagnosticData"
        )]
        pub log_service_push_diagnostic_data:
            Option<crate::log_service::v1_8_1::PushDiagnosticData>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::log_service::v1_8_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum AutoClearResolvedEntries {
        #[default]
        #[serde(rename = "ClearEventGroup")]
        ClearEventGroup,
        #[serde(rename = "None")]
        None,
        #[serde(rename = "RetainCauseResolutionEntries")]
        RetainCauseResolutionEntries,
        #[serde(rename = "UpdateCauseEntry")]
        UpdateCauseEntry,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearLog {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearLogRequestBody {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogEntriesETag")]
        pub log_entries_etag: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CollectDiagnosticData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CollectDiagnosticDataRequestBody {
        #[serde(rename = "DiagnosticDataType")]
        pub diagnostic_data_type: crate::log_service::v1_8_1::LogDiagnosticDataTypes,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OEMDiagnosticDataType"
        )]
        pub oem_diagnostic_data_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetDevice")]
        pub target_device: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TargetURI")]
        pub target_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::log_service::v1_8_1::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DiagnosticDataDetails {
        #[serde(skip_serializing_if = "Option::is_none", rename = "DiagnosticDataType")]
        pub diagnostic_data_type:
            Option<crate::log_service::v1_8_1::DiagnosticDataDetailsDiagnosticDataType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EstimatedDuration")]
        pub estimated_duration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EstimatedSizeBytes")]
        pub estimated_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OEMDiagnosticDataType"
        )]
        pub oem_diagnostic_data_type: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum DiagnosticDataDetailsDiagnosticDataType {
        V010801(crate::log_service::v1_8_1::LogDiagnosticDataTypes),
        V000001(crate::log_service::v1_8_1::DiagnosticDataDetailsDiagnosticDataTypeN1),
    }
    impl Default for DiagnosticDataDetailsDiagnosticDataType {
        fn default() -> Self {
            Self::V010801(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DiagnosticDataDetailsDiagnosticDataTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogDiagnosticDataTypes {
        #[default]
        #[serde(rename = "Device")]
        Device,
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
    pub enum LogEntryTypes {
        #[default]
        #[serde(rename = "CXL")]
        CXL,
        #[serde(rename = "Event")]
        Event,
        #[serde(rename = "Multiple")]
        Multiple,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "SEL")]
        SEL,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogPurpose {
        #[default]
        #[serde(rename = "Diagnostic")]
        Diagnostic,
        #[serde(rename = "ExternalEntity")]
        ExternalEntity,
        #[serde(rename = "OEM")]
        OEM,
        #[serde(rename = "Operations")]
        Operations,
        #[serde(rename = "Security")]
        Security,
        #[serde(rename = "Telemetry")]
        Telemetry,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LogService {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::log_service::v1_8_1::Actions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AutoClearResolvedEntries"
        )]
        pub auto_clear_resolved_entries:
            Option<crate::log_service::v1_8_1::LogServiceAutoClearResolvedEntries>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AutoDSTEnabled")]
        pub auto_dst_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DateTime")]
        pub date_time: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DateTimeLocalOffset"
        )]
        pub date_time_local_offset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::log_service::v1_8_1::LogServiceDescription>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DiagnosticDataDetails"
        )]
        pub diagnostic_data_details:
            Option<Vec<crate::log_service::v1_8_1::LogServiceDiagnosticDataDetails>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Entries")]
        pub entries: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogEntryType")]
        pub log_entry_type: Option<crate::log_service::v1_8_1::LogServiceLogEntryType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogPurposes")]
        pub log_purposes: Option<Vec<crate::log_service::v1_8_1::LogServiceLogPurposes>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxNumberOfRecords")]
        pub max_number_of_records: Option<i64>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OEMLogPurpose")]
        pub oem_log_purpose: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "OverWritePolicy")]
        pub over_write_policy: Option<crate::log_service::v1_8_1::OverWritePolicy>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Overflow")]
        pub overflow: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Persistency")]
        pub persistency: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyslogFilters")]
        pub syslog_filters: Option<Vec<crate::log_service::v1_8_1::LogServiceSyslogFilters>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogServiceAutoClearResolvedEntries {
        V010801(crate::log_service::v1_8_1::AutoClearResolvedEntries),
        V000001(crate::log_service::v1_8_1::LogServiceAutoClearResolvedEntriesN1),
    }
    impl Default for LogServiceAutoClearResolvedEntries {
        fn default() -> Self {
            Self::V010801(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogServiceAutoClearResolvedEntriesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogServiceDescription {
        V000001(crate::log_service::v1_8_1::LogServiceDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for LogServiceDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogServiceDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogServiceDiagnosticDataDetails {
        V010801(crate::log_service::v1_8_1::DiagnosticDataDetails),
        V000001(crate::log_service::v1_8_1::LogServiceDiagnosticDataDetailsN1),
    }
    impl Default for LogServiceDiagnosticDataDetails {
        fn default() -> Self {
            Self::V010801(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogServiceDiagnosticDataDetailsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogServiceLogEntryType {
        V010801(crate::log_service::v1_8_1::LogEntryTypes),
        V000001(crate::log_service::v1_8_1::LogServiceLogEntryTypeN1),
    }
    impl Default for LogServiceLogEntryType {
        fn default() -> Self {
            Self::V010801(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogServiceLogEntryTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogServiceLogPurposes {
        V010801(crate::log_service::v1_8_1::LogPurpose),
        V000001(crate::log_service::v1_8_1::LogServiceLogPurposesN1),
    }
    impl Default for LogServiceLogPurposes {
        fn default() -> Self {
            Self::V010801(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogServiceLogPurposesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LogServiceSyslogFilters {
        V010801(crate::log_service::v1_8_1::SyslogFilter),
        V000001(crate::log_service::v1_8_1::LogServiceSyslogFiltersN1),
    }
    impl Default for LogServiceSyslogFilters {
        fn default() -> Self {
            Self::V010801(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogServiceSyslogFiltersN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum OverWritePolicy {
        #[default]
        #[serde(rename = "NeverOverWrites")]
        NeverOverWrites,
        #[serde(rename = "Unknown")]
        Unknown,
        #[serde(rename = "WrapsWhenFull")]
        WrapsWhenFull,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PushDiagnosticData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PushDiagnosticDataRequestBody {
        #[serde(rename = "AdditionalDataURI")]
        pub additional_data_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(rename = "TargetURI")]
        pub target_uri: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TransferProtocol")]
        pub transfer_protocol: Option<crate::log_service::v1_8_1::TransferProtocolType>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "UserName")]
        pub user_name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogFacility {
        #[default]
        #[serde(rename = "Auth")]
        Auth,
        #[serde(rename = "Authpriv")]
        Authpriv,
        #[serde(rename = "Console")]
        Console,
        #[serde(rename = "Cron")]
        Cron,
        #[serde(rename = "Daemon")]
        Daemon,
        #[serde(rename = "FTP")]
        FTP,
        #[serde(rename = "Kern")]
        Kern,
        #[serde(rename = "LPR")]
        LPR,
        #[serde(rename = "Local0")]
        Local0,
        #[serde(rename = "Local1")]
        Local1,
        #[serde(rename = "Local2")]
        Local2,
        #[serde(rename = "Local3")]
        Local3,
        #[serde(rename = "Local4")]
        Local4,
        #[serde(rename = "Local5")]
        Local5,
        #[serde(rename = "Local6")]
        Local6,
        #[serde(rename = "Local7")]
        Local7,
        #[serde(rename = "Mail")]
        Mail,
        #[serde(rename = "NTP")]
        NTP,
        #[serde(rename = "News")]
        News,
        #[serde(rename = "Security")]
        Security,
        #[serde(rename = "SolarisCron")]
        SolarisCron,
        #[serde(rename = "Syslog")]
        Syslog,
        #[serde(rename = "UUCP")]
        UUCP,
        #[serde(rename = "User")]
        User,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SyslogFilter {
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogFacilities")]
        pub log_facilities: Option<Vec<crate::log_service::v1_8_1::SyslogFilterLogFacilities>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestSeverity")]
        pub lowest_severity: Option<crate::log_service::v1_8_1::SyslogFilterLowestSeverity>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SyslogFilterLogFacilities {
        V010801(crate::log_service::v1_8_1::SyslogFacility),
        V000001(crate::log_service::v1_8_1::SyslogFilterLogFacilitiesN1),
    }
    impl Default for SyslogFilterLogFacilities {
        fn default() -> Self {
            Self::V010801(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogFilterLogFacilitiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum SyslogFilterLowestSeverity {
        V010801(crate::log_service::v1_8_1::SyslogSeverity),
        V000001(crate::log_service::v1_8_1::SyslogFilterLowestSeverityN1),
    }
    impl Default for SyslogFilterLowestSeverity {
        fn default() -> Self {
            Self::V010801(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogFilterLowestSeverityN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SyslogSeverity {
        #[default]
        #[serde(rename = "Alert")]
        Alert,
        #[serde(rename = "All")]
        All,
        #[serde(rename = "Critical")]
        Critical,
        #[serde(rename = "Debug")]
        Debug,
        #[serde(rename = "Emergency")]
        Emergency,
        #[serde(rename = "Error")]
        Error,
        #[serde(rename = "Informational")]
        Informational,
        #[serde(rename = "Notice")]
        Notice,
        #[serde(rename = "Warning")]
        Warning,
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
}
