pub mod v1_4_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#LogService.ClearLog"
        )]
        pub log_service_clear_log: Option<crate::log_service::v1_4_0::ClearLog>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#LogService.CollectDiagnosticData"
        )]
        pub log_service_collect_diagnostic_data:
            Option<crate::log_service::v1_4_0::CollectDiagnosticData>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::log_service::v1_4_0::OemActions>,
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
        pub diagnostic_data_type: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OEMDiagnosticDataType"
        )]
        pub oem_diagnostic_data_type: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum LogDiagnosticDataTypes {
        #[default]
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
        pub actions: Option<crate::log_service::v1_4_0::Actions>,
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
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Entries")]
        pub entries: Option<crate::odata_v4::IdRef>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogEntryType")]
        pub log_entry_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LogPurposes")]
        pub log_purposes: Option<Vec<String>>,
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
        pub over_write_policy: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Overflow")]
        pub overflow: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Persistency")]
        pub persistency: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceEnabled")]
        pub service_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SyslogFilters")]
        pub syslog_filters: Option<Vec<crate::log_service::v1_4_0::SyslogFilter>>,
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
        pub log_facilities: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LowestSeverity")]
        pub lowest_severity: Option<String>,
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
}
