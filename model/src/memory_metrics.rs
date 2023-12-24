use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MemoryMetrics {
    V010700(crate::memory_metrics::v1_7_0::MemoryMetrics),
    V010600(crate::memory_metrics::v1_6_0::MemoryMetrics),
    V010500(crate::memory_metrics::v1_5_0::MemoryMetrics),
    V010401(crate::memory_metrics::v1_4_1::MemoryMetrics),
    V010301(crate::memory_metrics::v1_3_1::MemoryMetrics),
    V010202(crate::memory_metrics::v1_2_2::MemoryMetrics),
    V010108(crate::memory_metrics::v1_1_8::MemoryMetrics),
    V010008(crate::memory_metrics::v1_0_8::MemoryMetrics),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#MemoryMetrics.ClearCurrentPeriod"
        )]
        pub memory_metrics_clear_current_period:
            Option<crate::memory_metrics::v1_0_8::ClearCurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_metrics::v1_0_8::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AlarmTrips {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressParityError")]
        pub address_parity_error: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCError"
        )]
        pub correctable_ecc_error: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareBlock")]
        pub spare_block: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperature")]
        pub temperature: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCError"
        )]
        pub uncorrectable_ecc_error: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriodRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HealthData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlarmTrips")]
        pub alarm_trips: Option<crate::memory_metrics::v1_0_8::AlarmTrips>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLossDetected")]
        pub data_loss_detected: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastShutdownSuccess"
        )]
        pub last_shutdown_success: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PerformanceDegraded"
        )]
        pub performance_degraded: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingSpareBlockPercentage"
        )]
        pub remaining_spare_block_percentage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LifeTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory_metrics::v1_0_8::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentPeriod")]
        pub current_period: Option<crate::memory_metrics::v1_0_8::CurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HealthData")]
        pub health_data: Option<crate::memory_metrics::v1_0_8::HealthData>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifeTime")]
        pub life_time: Option<crate::memory_metrics::v1_0_8::LifeTime>,
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
    pub struct OemActions {}
}
pub mod v1_1_8 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#MemoryMetrics.ClearCurrentPeriod"
        )]
        pub memory_metrics_clear_current_period:
            Option<crate::memory_metrics::v1_1_8::ClearCurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_metrics::v1_1_8::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AlarmTrips {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressParityError")]
        pub address_parity_error: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCError"
        )]
        pub correctable_ecc_error: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareBlock")]
        pub spare_block: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperature")]
        pub temperature: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCError"
        )]
        pub uncorrectable_ecc_error: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriodRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HealthData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlarmTrips")]
        pub alarm_trips: Option<crate::memory_metrics::v1_1_8::AlarmTrips>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLossDetected")]
        pub data_loss_detected: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastShutdownSuccess"
        )]
        pub last_shutdown_success: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PerformanceDegraded"
        )]
        pub performance_degraded: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingSpareBlockPercentage"
        )]
        pub remaining_spare_block_percentage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LifeTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory_metrics::v1_1_8::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentPeriod")]
        pub current_period: Option<crate::memory_metrics::v1_1_8::CurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HealthData")]
        pub health_data: Option<crate::memory_metrics::v1_1_8::HealthData>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifeTime")]
        pub life_time: Option<crate::memory_metrics::v1_1_8::LifeTime>,
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
    pub struct OemActions {}
}
pub mod v1_2_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#MemoryMetrics.ClearCurrentPeriod"
        )]
        pub memory_metrics_clear_current_period:
            Option<crate::memory_metrics::v1_2_2::ClearCurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_metrics::v1_2_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AlarmTrips {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressParityError")]
        pub address_parity_error: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCError"
        )]
        pub correctable_ecc_error: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareBlock")]
        pub spare_block: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperature")]
        pub temperature: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCError"
        )]
        pub uncorrectable_ecc_error: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriodRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HealthData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlarmTrips")]
        pub alarm_trips: Option<crate::memory_metrics::v1_2_2::AlarmTrips>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLossDetected")]
        pub data_loss_detected: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastShutdownSuccess"
        )]
        pub last_shutdown_success: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PerformanceDegraded"
        )]
        pub performance_degraded: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingSpareBlockPercentage"
        )]
        pub remaining_spare_block_percentage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LifeTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory_metrics::v1_2_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BandwidthPercent")]
        pub bandwidth_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentPeriod")]
        pub current_period: Option<crate::memory_metrics::v1_2_2::CurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HealthData")]
        pub health_data: Option<crate::memory_metrics::v1_2_2::HealthData>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifeTime")]
        pub life_time: Option<crate::memory_metrics::v1_2_2::LifeTime>,
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
    pub struct OemActions {}
}
pub mod v1_3_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#MemoryMetrics.ClearCurrentPeriod"
        )]
        pub memory_metrics_clear_current_period:
            Option<crate::memory_metrics::v1_3_1::ClearCurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_metrics::v1_3_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AlarmTrips {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressParityError")]
        pub address_parity_error: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCError"
        )]
        pub correctable_ecc_error: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareBlock")]
        pub spare_block: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperature")]
        pub temperature: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCError"
        )]
        pub uncorrectable_ecc_error: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriodRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HealthData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlarmTrips")]
        pub alarm_trips: Option<crate::memory_metrics::v1_3_1::AlarmTrips>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLossDetected")]
        pub data_loss_detected: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastShutdownSuccess"
        )]
        pub last_shutdown_success: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PerformanceDegraded"
        )]
        pub performance_degraded: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingSpareBlockPercentage"
        )]
        pub remaining_spare_block_percentage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LifeTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory_metrics::v1_3_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BandwidthPercent")]
        pub bandwidth_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentPeriod")]
        pub current_period: Option<crate::memory_metrics::v1_3_1::CurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HealthData")]
        pub health_data: Option<crate::memory_metrics::v1_3_1::HealthData>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifeTime")]
        pub life_time: Option<crate::memory_metrics::v1_3_1::LifeTime>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMHz")]
        pub operating_speed_mhz: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_4_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#MemoryMetrics.ClearCurrentPeriod"
        )]
        pub memory_metrics_clear_current_period:
            Option<crate::memory_metrics::v1_4_1::ClearCurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_metrics::v1_4_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AlarmTrips {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressParityError")]
        pub address_parity_error: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCError"
        )]
        pub correctable_ecc_error: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareBlock")]
        pub spare_block: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperature")]
        pub temperature: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCError"
        )]
        pub uncorrectable_ecc_error: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriodRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCErrorCount"
        )]
        pub correctable_ecc_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCErrorCount"
        )]
        pub uncorrectable_ecc_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HealthData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlarmTrips")]
        pub alarm_trips: Option<crate::memory_metrics::v1_4_1::AlarmTrips>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLossDetected")]
        pub data_loss_detected: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastShutdownSuccess"
        )]
        pub last_shutdown_success: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PerformanceDegraded"
        )]
        pub performance_degraded: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingSpareBlockPercentage"
        )]
        pub remaining_spare_block_percentage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LifeTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCErrorCount"
        )]
        pub correctable_ecc_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCErrorCount"
        )]
        pub uncorrectable_ecc_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory_metrics::v1_4_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BandwidthPercent")]
        pub bandwidth_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentPeriod")]
        pub current_period: Option<crate::memory_metrics::v1_4_1::CurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HealthData")]
        pub health_data: Option<crate::memory_metrics::v1_4_1::HealthData>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifeTime")]
        pub life_time: Option<crate::memory_metrics::v1_4_1::LifeTime>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMHz")]
        pub operating_speed_mhz: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_5_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#MemoryMetrics.ClearCurrentPeriod"
        )]
        pub memory_metrics_clear_current_period:
            Option<crate::memory_metrics::v1_5_0::ClearCurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_metrics::v1_5_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AlarmTrips {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressParityError")]
        pub address_parity_error: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCError"
        )]
        pub correctable_ecc_error: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareBlock")]
        pub spare_block: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperature")]
        pub temperature: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCError"
        )]
        pub uncorrectable_ecc_error: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriodRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCErrorCount"
        )]
        pub correctable_ecc_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IndeterminateCorrectableErrorCount"
        )]
        pub indeterminate_correctable_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IndeterminateUncorrectableErrorCount"
        )]
        pub indeterminate_uncorrectable_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCErrorCount"
        )]
        pub uncorrectable_ecc_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HealthData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlarmTrips")]
        pub alarm_trips: Option<crate::memory_metrics::v1_5_0::AlarmTrips>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLossDetected")]
        pub data_loss_detected: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastShutdownSuccess"
        )]
        pub last_shutdown_success: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PerformanceDegraded"
        )]
        pub performance_degraded: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingSpareBlockPercentage"
        )]
        pub remaining_spare_block_percentage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LifeTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCErrorCount"
        )]
        pub correctable_ecc_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IndeterminateCorrectableErrorCount"
        )]
        pub indeterminate_correctable_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IndeterminateUncorrectableErrorCount"
        )]
        pub indeterminate_uncorrectable_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCErrorCount"
        )]
        pub uncorrectable_ecc_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory_metrics::v1_5_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BandwidthPercent")]
        pub bandwidth_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentPeriod")]
        pub current_period: Option<crate::memory_metrics::v1_5_0::CurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HealthData")]
        pub health_data: Option<crate::memory_metrics::v1_5_0::HealthData>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifeTime")]
        pub life_time: Option<crate::memory_metrics::v1_5_0::LifeTime>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMHz")]
        pub operating_speed_mhz: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_6_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#MemoryMetrics.ClearCurrentPeriod"
        )]
        pub memory_metrics_clear_current_period:
            Option<crate::memory_metrics::v1_6_0::ClearCurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_metrics::v1_6_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AlarmTrips {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressParityError")]
        pub address_parity_error: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCError"
        )]
        pub correctable_ecc_error: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareBlock")]
        pub spare_block: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperature")]
        pub temperature: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCError"
        )]
        pub uncorrectable_ecc_error: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AlertCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCError"
        )]
        pub correctable_ecc_error: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareBlock")]
        pub spare_block: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperature")]
        pub temperature: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCError"
        )]
        pub uncorrectable_ecc_error: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXL {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlertCapabilities")]
        pub alert_capabilities: Option<crate::memory_metrics::v1_6_0::AlertCapabilities>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriodRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCErrorCount"
        )]
        pub correctable_ecc_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IndeterminateCorrectableErrorCount"
        )]
        pub indeterminate_correctable_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IndeterminateUncorrectableErrorCount"
        )]
        pub indeterminate_uncorrectable_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCErrorCount"
        )]
        pub uncorrectable_ecc_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HealthData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlarmTrips")]
        pub alarm_trips: Option<crate::memory_metrics::v1_6_0::AlarmTrips>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLossDetected")]
        pub data_loss_detected: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastShutdownSuccess"
        )]
        pub last_shutdown_success: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PerformanceDegraded"
        )]
        pub performance_degraded: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingSpareBlockPercentage"
        )]
        pub remaining_spare_block_percentage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LifeTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCErrorCount"
        )]
        pub correctable_ecc_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IndeterminateCorrectableErrorCount"
        )]
        pub indeterminate_correctable_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IndeterminateUncorrectableErrorCount"
        )]
        pub indeterminate_uncorrectable_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCErrorCount"
        )]
        pub uncorrectable_ecc_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory_metrics::v1_6_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BandwidthPercent")]
        pub bandwidth_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectedPersistentErrorCount"
        )]
        pub corrected_persistent_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectedVolatileErrorCount"
        )]
        pub corrected_volatile_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentPeriod")]
        pub current_period: Option<crate::memory_metrics::v1_6_0::CurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXL")]
        pub cxl: Option<crate::memory_metrics::v1_6_0::CXL>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DirtyShutdownCount")]
        pub dirty_shutdown_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HealthData")]
        pub health_data: Option<crate::memory_metrics::v1_6_0::HealthData>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifeTime")]
        pub life_time: Option<crate::memory_metrics::v1_6_0::LifeTime>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMHz")]
        pub operating_speed_mhz: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
pub mod v1_7_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#MemoryMetrics.ClearCurrentPeriod"
        )]
        pub memory_metrics_clear_current_period:
            Option<crate::memory_metrics::v1_7_0::ClearCurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::memory_metrics::v1_7_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AlarmTrips {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AddressParityError")]
        pub address_parity_error: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCError"
        )]
        pub correctable_ecc_error: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareBlock")]
        pub spare_block: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperature")]
        pub temperature: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCError"
        )]
        pub uncorrectable_ecc_error: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AlertCapabilities {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCError"
        )]
        pub correctable_ecc_error: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpareBlock")]
        pub spare_block: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Temperature")]
        pub temperature: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCError"
        )]
        pub uncorrectable_ecc_error: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXL {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlertCapabilities")]
        pub alert_capabilities: Option<crate::memory_metrics::v1_7_0::AlertCapabilities>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ClearCurrentPeriodRequestBody {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CurrentPeriod {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCErrorCount"
        )]
        pub correctable_ecc_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IndeterminateCorrectableErrorCount"
        )]
        pub indeterminate_correctable_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IndeterminateUncorrectableErrorCount"
        )]
        pub indeterminate_uncorrectable_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCErrorCount"
        )]
        pub uncorrectable_ecc_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HealthData {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AlarmTrips")]
        pub alarm_trips: Option<crate::memory_metrics::v1_7_0::AlarmTrips>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DataLossDetected")]
        pub data_loss_detected: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LastShutdownSuccess"
        )]
        pub last_shutdown_success: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PerformanceDegraded"
        )]
        pub performance_degraded: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PredictedMediaLifeLeftPercent"
        )]
        pub predicted_media_life_left_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RemainingSpareBlockPercentage"
        )]
        pub remaining_spare_block_percentage: Option<f64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct LifeTime {
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksRead")]
        pub blocks_read: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlocksWritten")]
        pub blocks_written: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableECCErrorCount"
        )]
        pub correctable_ecc_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IndeterminateCorrectableErrorCount"
        )]
        pub indeterminate_correctable_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "IndeterminateUncorrectableErrorCount"
        )]
        pub indeterminate_uncorrectable_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableECCErrorCount"
        )]
        pub uncorrectable_ecc_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MemoryMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::memory_metrics::v1_7_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BandwidthPercent")]
        pub bandwidth_percent: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "BlockSizeBytes")]
        pub block_size_bytes: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CapacityUtilizationPercent"
        )]
        pub capacity_utilization_percent: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectedPersistentErrorCount"
        )]
        pub corrected_persistent_error_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectedVolatileErrorCount"
        )]
        pub corrected_volatile_error_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentPeriod")]
        pub current_period: Option<crate::memory_metrics::v1_7_0::CurrentPeriod>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXL")]
        pub cxl: Option<crate::memory_metrics::v1_7_0::CXL>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DirtyShutdownCount")]
        pub dirty_shutdown_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HealthData")]
        pub health_data: Option<crate::memory_metrics::v1_7_0::HealthData>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LifeTime")]
        pub life_time: Option<crate::memory_metrics::v1_7_0::LifeTime>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "OperatingSpeedMHz")]
        pub operating_speed_mhz: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
}
