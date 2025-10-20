pub type NVMeSMARTCriticalWarnings = crate::storage_controller::v1_10_0::NVMeSMARTCriticalWarnings;
pub type StorageController = crate::storage_controller::v1_10_0::StorageController;
pub mod v1_9_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ANAAccessState {
        #[default]
        #[serde(rename = "Inaccessible")]
        Inaccessible,
        #[serde(rename = "NonOptimized")]
        NonOptimized,
        #[serde(rename = "Optimized")]
        Optimized,
        #[serde(rename = "PersistentLoss")]
        PersistentLoss,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ANACharacteristics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::storage_controller::v1_9_1::ANACharacteristicsAccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volume")]
        pub volume: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ANACharacteristicsAccessState {
        V010901(crate::storage_controller::v1_9_1::ANAAccessState),
        V000001(crate::storage_controller::v1_9_1::ANACharacteristicsAccessStateN1),
    }
    impl Default for ANACharacteristicsAccessState {
        fn default() -> Self {
            Self::V010901(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ANACharacteristicsAccessStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::storage_controller::v1_9_1::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageController.AttachNamespaces"
        )]
        pub storage_controller_attach_namespaces:
            Option<crate::storage_controller::v1_9_1::AttachNamespaces>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageController.DetachNamespaces"
        )]
        pub storage_controller_detach_namespaces:
            Option<crate::storage_controller::v1_9_1::DetachNamespaces>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageController.SecurityReceive"
        )]
        pub storage_controller_security_receive:
            Option<crate::storage_controller::v1_9_1::SecurityReceive>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageController.SecuritySend"
        )]
        pub storage_controller_security_send:
            Option<crate::storage_controller::v1_9_1::SecuritySend>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AttachDetachNamespacesResponse {
        #[serde(rename = "AttachedVolumes")]
        pub attached_volumes: Vec<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AttachedVolumes@odata.count"
        )]
        pub attached_volumes_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AttachNamespaces {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AttachNamespacesRequestBody {
        #[serde(rename = "Namespaces")]
        pub namespaces: Vec<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CacheSummary {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentCacheSizeMiB"
        )]
        pub persistent_cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TotalCacheSizeMiB")]
        pub total_cache_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DetachNamespaces {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DetachNamespacesRequestBody {
        #[serde(rename = "Namespaces")]
        pub namespaces: Vec<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AttachedVolumes")]
        pub attached_volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AttachedVolumes@odata.count"
        )]
        pub attached_volumes_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Batteries")]
        pub batteries: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Batteries@odata.count"
        )]
        pub batteries_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctions"
        )]
        pub network_device_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctions@odata.count"
        )]
        pub network_device_functions_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeDiscoveredSubsystems"
        )]
        pub nvme_discovered_subsystems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeDiscoveredSubsystems@odata.count"
        )]
        pub nvme_discovered_subsystems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeControllerAttributes {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReportsNamespaceGranularity"
        )]
        pub reports_namespace_granularity: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportsUUIDList")]
        pub reports_uuid_list: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Supports128BitHostId"
        )]
        pub supports128_bit_host_id: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsEnduranceGroups"
        )]
        pub supports_endurance_groups: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsExceedingPowerOfNonOperationalState"
        )]
        pub supports_exceeding_power_of_non_operational_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportsNVMSets")]
        pub supports_nvm_sets: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsPredictableLatencyMode"
        )]
        pub supports_predictable_latency_mode: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsReadRecoveryLevels"
        )]
        pub supports_read_recovery_levels: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsReservations"
        )]
        pub supports_reservations: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsSQAssociations"
        )]
        pub supports_sq_associations: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsTrafficBasedKeepAlive"
        )]
        pub supports_traffic_based_keep_alive: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeControllerProperties {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocatedCompletionQueues"
        )]
        pub allocated_completion_queues: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocatedSubmissionQueues"
        )]
        pub allocated_submission_queues: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ANACharacteristics")]
        pub ana_characteristics: Option<
            Vec<crate::storage_controller::v1_9_1::NVMeControllerPropertiesANACharacteristics>,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControllerType")]
        pub controller_type:
            Option<crate::storage_controller::v1_9_1::NVMeControllerPropertiesControllerType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DiscoveryTransportServiceId"
        )]
        pub discovery_transport_service_id: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxAttachedNamespaces"
        )]
        pub max_attached_namespaces: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxQueueSize")]
        pub max_queue_size: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeControllerAttributes"
        )]
        pub nvme_controller_attributes: Option<
            crate::storage_controller::v1_9_1::NVMeControllerPropertiesNVMeControllerAttributes,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeSMARTCriticalWarnings"
        )]
        pub nvme_smart_critical_warnings: Option<
            crate::storage_controller::v1_9_1::NVMeControllerPropertiesNVMeSMARTCriticalWarnings,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeVersion")]
        pub nvme_version: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeControllerPropertiesANACharacteristics {
        V010901(crate::storage_controller::v1_9_1::ANACharacteristics),
        V000001(crate::storage_controller::v1_9_1::NVMeControllerPropertiesANACharacteristicsN1),
    }
    impl Default for NVMeControllerPropertiesANACharacteristics {
        fn default() -> Self {
            Self::V010901(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeControllerPropertiesANACharacteristicsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeControllerPropertiesControllerType {
        V010901(crate::storage_controller::v1_9_1::NVMeControllerType),
        V000001(crate::storage_controller::v1_9_1::NVMeControllerPropertiesControllerTypeN1),
    }
    impl Default for NVMeControllerPropertiesControllerType {
        fn default() -> Self {
            Self::V010901(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeControllerPropertiesControllerTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeControllerPropertiesNVMeControllerAttributes {
        V010901(crate::storage_controller::v1_9_1::NVMeControllerAttributes),
        V000001(
            crate::storage_controller::v1_9_1::NVMeControllerPropertiesNVMeControllerAttributesN1,
        ),
    }
    impl Default for NVMeControllerPropertiesNVMeControllerAttributes {
        fn default() -> Self {
            Self::V010901(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeControllerPropertiesNVMeControllerAttributesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeControllerPropertiesNVMeSMARTCriticalWarnings {
        V010901(crate::storage_controller::v1_9_1::NVMeSMARTCriticalWarnings),
        V000001(
            crate::storage_controller::v1_9_1::NVMeControllerPropertiesNVMeSMARTCriticalWarningsN1,
        ),
    }
    impl Default for NVMeControllerPropertiesNVMeSMARTCriticalWarnings {
        fn default() -> Self {
            Self::V010901(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeControllerPropertiesNVMeSMARTCriticalWarningsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeControllerType {
        #[default]
        #[serde(rename = "Admin")]
        Admin,
        #[serde(rename = "Discovery")]
        Discovery,
        #[serde(rename = "IO")]
        IO,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeSMARTCriticalWarnings {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaInReadOnly")]
        pub media_in_read_only: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OverallSubsystemDegraded"
        )]
        pub overall_subsystem_degraded: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PMRUnreliable")]
        pub pmr_unreliable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerBackupFailed")]
        pub power_backup_failed: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SpareCapacityWornOut"
        )]
        pub spare_capacity_worn_out: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Rates {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsistencyCheckRatePercent"
        )]
        pub consistency_check_rate_percent: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RebuildRatePercent")]
        pub rebuild_rate_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransformationRatePercent"
        )]
        pub transformation_rate_percent: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityReceive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityReceiveRequestBody {
        #[serde(rename = "AllocationLength")]
        pub allocation_length: i64,
        #[serde(rename = "SecurityProtocol")]
        pub security_protocol: i64,
        #[serde(rename = "SecurityProtocolSpecific")]
        pub security_protocol_specific: i64,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityReceiveResponse {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Data")]
        pub data: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecuritySend {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecuritySendRequestBody {
        #[serde(rename = "Data")]
        pub data: String,
        #[serde(rename = "SecurityProtocol")]
        pub security_protocol: i64,
        #[serde(rename = "SecurityProtocolSpecific")]
        pub security_protocol_specific: i64,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageController {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::storage_controller::v1_9_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSummary")]
        pub cache_summary: Option<crate::storage_controller::v1_9_1::CacheSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControllerRates")]
        pub controller_rates: Option<crate::storage_controller::v1_9_1::Rates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::storage_controller::v1_9_1::StorageControllerDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::storage_controller::v1_9_1::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::storage_controller::v1_9_1::StorageControllerMetrics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeControllerProperties"
        )]
        pub nvme_controller_properties:
            Option<crate::storage_controller::v1_9_1::NVMeControllerProperties>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedGbps")]
        pub speed_gbps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedControllerProtocols"
        )]
        pub supported_controller_protocols: Option<Vec<crate::protocol::Protocol>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedDeviceProtocols"
        )]
        pub supported_device_protocols: Option<Vec<crate::protocol::Protocol>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedRAIDTypes")]
        pub supported_raid_types:
            Option<Vec<crate::storage_controller::v1_9_1::StorageControllerSupportedRAIDTypes>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageControllerDescription {
        V000001(crate::storage_controller::v1_9_1::StorageControllerDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for StorageControllerDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageControllerDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageControllerMetrics {
        V000001(crate::storage_controller::v1_9_1::StorageControllerMetricsN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for StorageControllerMetrics {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageControllerMetricsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageControllerSupportedRAIDTypes {
        V000001(crate::storage_controller::v1_9_1::StorageControllerSupportedRAIDTypesN1),
        VolumeRAIDType(crate::swordfish::volume::RAIDType),
    }
    impl Default for StorageControllerSupportedRAIDTypes {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageControllerSupportedRAIDTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
pub mod v1_10_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ANAAccessState {
        #[default]
        #[serde(rename = "Inaccessible")]
        Inaccessible,
        #[serde(rename = "NonOptimized")]
        NonOptimized,
        #[serde(rename = "Optimized")]
        Optimized,
        #[serde(rename = "PersistentLoss")]
        PersistentLoss,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ANACharacteristics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessState")]
        pub access_state: Option<crate::storage_controller::v1_10_0::ANACharacteristicsAccessState>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Volume")]
        pub volume: Option<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ANACharacteristicsAccessState {
        V011000(crate::storage_controller::v1_10_0::ANAAccessState),
        V000001(crate::storage_controller::v1_10_0::ANACharacteristicsAccessStateN1),
    }
    impl Default for ANACharacteristicsAccessState {
        fn default() -> Self {
            Self::V011000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ANACharacteristicsAccessStateN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::storage_controller::v1_10_0::OemActions>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageController.AttachNamespaces"
        )]
        pub storage_controller_attach_namespaces:
            Option<crate::storage_controller::v1_10_0::AttachNamespaces>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageController.DetachNamespaces"
        )]
        pub storage_controller_detach_namespaces:
            Option<crate::storage_controller::v1_10_0::DetachNamespaces>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageController.SecurityReceive"
        )]
        pub storage_controller_security_receive:
            Option<crate::storage_controller::v1_10_0::SecurityReceive>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "#StorageController.SecuritySend"
        )]
        pub storage_controller_security_send:
            Option<crate::storage_controller::v1_10_0::SecuritySend>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AttachDetachNamespacesResponse {
        #[serde(rename = "AttachedVolumes")]
        pub attached_volumes: Vec<crate::odata_v4::IdRef>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AttachedVolumes@odata.count"
        )]
        pub attached_volumes_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AttachNamespaces {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct AttachNamespacesRequestBody {
        #[serde(rename = "Namespaces")]
        pub namespaces: Vec<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CacheSummary {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PersistentCacheSizeMiB"
        )]
        pub persistent_cache_size_mib: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(rename = "TotalCacheSizeMiB")]
        pub total_cache_size_mib: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DetachNamespaces {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct DetachNamespacesRequestBody {
        #[serde(rename = "Namespaces")]
        pub namespaces: Vec<crate::odata_v4::IdRef>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Links {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AttachedVolumes")]
        pub attached_volumes: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AttachedVolumes@odata.count"
        )]
        pub attached_volumes_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Batteries")]
        pub batteries: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Batteries@odata.count"
        )]
        pub batteries_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Endpoints")]
        pub endpoints: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Endpoints@odata.count"
        )]
        pub endpoints_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctions"
        )]
        pub network_device_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkDeviceFunctions@odata.count"
        )]
        pub network_device_functions_odata_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeDiscoveredSubsystems"
        )]
        pub nvme_discovered_subsystems: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeDiscoveredSubsystems@odata.count"
        )]
        pub nvme_discovered_subsystems_odata_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::resource::Oem>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeFunctions")]
        pub pcie_functions: Option<Vec<crate::odata_v4::IdRef>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PCIeFunctions@odata.count"
        )]
        pub pcie_functions_odata_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct MPFProperties {
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsSupervisor")]
        pub is_supervisor: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeControllerAttributes {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ReportsNamespaceGranularity"
        )]
        pub reports_namespace_granularity: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReportsUUIDList")]
        pub reports_uuid_list: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "Supports128BitHostId"
        )]
        pub supports128_bit_host_id: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsEnduranceGroups"
        )]
        pub supports_endurance_groups: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsExceedingPowerOfNonOperationalState"
        )]
        pub supports_exceeding_power_of_non_operational_state: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportsNVMSets")]
        pub supports_nvm_sets: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsPredictableLatencyMode"
        )]
        pub supports_predictable_latency_mode: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsReadRecoveryLevels"
        )]
        pub supports_read_recovery_levels: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsReservations"
        )]
        pub supports_reservations: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsSQAssociations"
        )]
        pub supports_sq_associations: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportsTrafficBasedKeepAlive"
        )]
        pub supports_traffic_based_keep_alive: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeControllerProperties {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocatedCompletionQueues"
        )]
        pub allocated_completion_queues: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AllocatedSubmissionQueues"
        )]
        pub allocated_submission_queues: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ANACharacteristics")]
        pub ana_characteristics: Option<
            Vec<crate::storage_controller::v1_10_0::NVMeControllerPropertiesANACharacteristics>,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControllerType")]
        pub controller_type:
            Option<crate::storage_controller::v1_10_0::NVMeControllerPropertiesControllerType>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "DiscoveryTransportServiceId"
        )]
        pub discovery_transport_service_id: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "MaxAttachedNamespaces"
        )]
        pub max_attached_namespaces: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxQueueSize")]
        pub max_queue_size: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeControllerAttributes"
        )]
        pub nvme_controller_attributes: Option<
            crate::storage_controller::v1_10_0::NVMeControllerPropertiesNVMeControllerAttributes,
        >,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeSMARTCriticalWarnings"
        )]
        pub nvme_smart_critical_warnings: Option<
            crate::storage_controller::v1_10_0::NVMeControllerPropertiesNVMeSMARTCriticalWarnings,
        >,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NVMeVersion")]
        pub nvme_version: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeControllerPropertiesANACharacteristics {
        V011000(crate::storage_controller::v1_10_0::ANACharacteristics),
        V000001(crate::storage_controller::v1_10_0::NVMeControllerPropertiesANACharacteristicsN1),
    }
    impl Default for NVMeControllerPropertiesANACharacteristics {
        fn default() -> Self {
            Self::V011000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeControllerPropertiesANACharacteristicsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeControllerPropertiesControllerType {
        V011000(crate::storage_controller::v1_10_0::NVMeControllerType),
        V000001(crate::storage_controller::v1_10_0::NVMeControllerPropertiesControllerTypeN1),
    }
    impl Default for NVMeControllerPropertiesControllerType {
        fn default() -> Self {
            Self::V011000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeControllerPropertiesControllerTypeN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeControllerPropertiesNVMeControllerAttributes {
        V011000(crate::storage_controller::v1_10_0::NVMeControllerAttributes),
        V000001(
            crate::storage_controller::v1_10_0::NVMeControllerPropertiesNVMeControllerAttributesN1,
        ),
    }
    impl Default for NVMeControllerPropertiesNVMeControllerAttributes {
        fn default() -> Self {
            Self::V011000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeControllerPropertiesNVMeControllerAttributesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum NVMeControllerPropertiesNVMeSMARTCriticalWarnings {
        V011000(crate::storage_controller::v1_10_0::NVMeSMARTCriticalWarnings),
        V000001(
            crate::storage_controller::v1_10_0::NVMeControllerPropertiesNVMeSMARTCriticalWarningsN1,
        ),
    }
    impl Default for NVMeControllerPropertiesNVMeSMARTCriticalWarnings {
        fn default() -> Self {
            Self::V011000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeControllerPropertiesNVMeSMARTCriticalWarningsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NVMeControllerType {
        #[default]
        #[serde(rename = "Admin")]
        Admin,
        #[serde(rename = "Discovery")]
        Discovery,
        #[serde(rename = "IO")]
        IO,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NVMeSMARTCriticalWarnings {
        #[serde(skip_serializing_if = "Option::is_none", rename = "MediaInReadOnly")]
        pub media_in_read_only: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "OverallSubsystemDegraded"
        )]
        pub overall_subsystem_degraded: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PMRUnreliable")]
        pub pmr_unreliable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PowerBackupFailed")]
        pub power_backup_failed: Option<bool>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SpareCapacityWornOut"
        )]
        pub spare_capacity_worn_out: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Rates {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "ConsistencyCheckRatePercent"
        )]
        pub consistency_check_rate_percent: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RebuildRatePercent")]
        pub rebuild_rate_percent: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TransformationRatePercent"
        )]
        pub transformation_rate_percent: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityReceive {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityReceiveRequestBody {
        #[serde(rename = "AllocationLength")]
        pub allocation_length: i64,
        #[serde(rename = "SecurityProtocol")]
        pub security_protocol: i64,
        #[serde(rename = "SecurityProtocolSpecific")]
        pub security_protocol_specific: i64,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecurityReceiveResponse {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Data")]
        pub data: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecuritySend {
        #[serde(skip_serializing_if = "Option::is_none", rename = "target")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "title")]
        pub title: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SecuritySendRequestBody {
        #[serde(rename = "Data")]
        pub data: String,
        #[serde(rename = "SecurityProtocol")]
        pub security_protocol: i64,
        #[serde(rename = "SecurityProtocolSpecific")]
        pub security_protocol_specific: i64,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct StorageController {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::storage_controller::v1_10_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Assembly")]
        pub assembly: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "AssetTag")]
        pub asset_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CacheSummary")]
        pub cache_summary: Option<crate::storage_controller::v1_10_0::CacheSummary>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ControllerRates")]
        pub controller_rates: Option<crate::storage_controller::v1_10_0::Rates>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<crate::storage_controller::v1_10_0::StorageControllerDescription>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnvironmentMetrics")]
        pub environment_metrics: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FirmwareVersion")]
        pub firmware_version: Option<String>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Identifiers")]
        pub identifiers: Option<Vec<crate::resource::Identifier>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IsLogical")]
        pub is_logical: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Links")]
        pub links: Option<crate::storage_controller::v1_10_0::Links>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Location")]
        pub location: Option<crate::resource::Location>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Manufacturer")]
        pub manufacturer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Measurements")]
        pub measurements: Option<Vec<crate::software_inventory::MeasurementBlock>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Metrics")]
        pub metrics: Option<crate::storage_controller::v1_10_0::StorageControllerMetrics>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Model")]
        pub model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MPFProperties")]
        pub mpf_properties:
            Option<crate::storage_controller::v1_10_0::StorageControllerMPFProperties>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NVMeControllerProperties"
        )]
        pub nvme_controller_properties:
            Option<crate::storage_controller::v1_10_0::NVMeControllerProperties>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeInterface")]
        pub pcie_interface: Option<crate::pcie_device::PCIeInterface>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Ports")]
        pub ports: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SerialNumber")]
        pub serial_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
        pub sku: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SpeedGbps")]
        pub speed_gbps: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedControllerProtocols"
        )]
        pub supported_controller_protocols: Option<Vec<crate::protocol::Protocol>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "SupportedDeviceProtocols"
        )]
        pub supported_device_protocols: Option<Vec<crate::protocol::Protocol>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupportedRAIDTypes")]
        pub supported_raid_types:
            Option<Vec<crate::storage_controller::v1_10_0::StorageControllerSupportedRAIDTypes>>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageControllerDescription {
        V000001(crate::storage_controller::v1_10_0::StorageControllerDescriptionN1),
        ResourceDescription(String),
    }
    impl Default for StorageControllerDescription {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageControllerDescriptionN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageControllerMPFProperties {
        V011000(crate::storage_controller::v1_10_0::MPFProperties),
        V000001(crate::storage_controller::v1_10_0::StorageControllerMPFPropertiesN1),
    }
    impl Default for StorageControllerMPFProperties {
        fn default() -> Self {
            Self::V011000(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageControllerMPFPropertiesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageControllerMetrics {
        V000001(crate::storage_controller::v1_10_0::StorageControllerMetricsN1),
        OdataV4IdRef(crate::odata_v4::IdRef),
    }
    impl Default for StorageControllerMetrics {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageControllerMetricsN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum StorageControllerSupportedRAIDTypes {
        V000001(crate::storage_controller::v1_10_0::StorageControllerSupportedRAIDTypesN1),
        VolumeRAIDType(crate::swordfish::volume::RAIDType),
    }
    impl Default for StorageControllerSupportedRAIDTypes {
        fn default() -> Self {
            Self::V000001(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum StorageControllerSupportedRAIDTypesN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
