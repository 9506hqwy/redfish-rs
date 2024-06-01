pub mod acceleration_function;
pub mod acceleration_function_collection;
pub mod account_service;
pub mod address_pool;
pub mod address_pool_collection;
pub mod aggregate;
pub mod aggregate_collection;
pub mod aggregation_service;
pub mod aggregation_source;
pub mod aggregation_source_collection;
pub mod allow_deny;
pub mod allow_deny_collection;
pub mod application;
pub mod application_collection;
pub mod assembly;
pub mod battery;
pub mod battery_collection;
pub mod battery_metrics;
pub mod bios;
pub mod boot_option;
pub mod boot_option_collection;
pub mod cable;
pub mod cable_collection;
pub mod certificate;
pub mod certificate_collection;
pub mod certificate_locations;
pub mod certificate_service;
pub mod chassis;
pub mod chassis_collection;
pub mod circuit;
pub mod circuit_collection;
pub mod component_integrity;
pub mod component_integrity_collection;
pub mod composition_reservation;
pub mod composition_reservation_collection;
pub mod composition_service;
pub mod computer_system;
pub mod computer_system_collection;
pub mod connection;
pub mod connection_collection;
pub mod connection_method;
pub mod connection_method_collection;
pub mod container;
pub mod container_collection;
pub mod container_image;
pub mod container_image_collection;
pub mod control;
pub mod control_collection;
pub mod coolant_connector;
pub mod coolant_connector_collection;
pub mod cooling_loop;
pub mod cooling_loop_collection;
pub mod cooling_unit;
pub mod cooling_unit_collection;
pub mod cxl_logical_device;
pub mod cxl_logical_device_collection;
pub mod drive;
pub mod drive_collection;
pub mod drive_metrics;
pub mod endpoint;
pub mod endpoint_collection;
pub mod endpoint_group;
pub mod endpoint_group_collection;
pub mod environment_metrics;
pub mod ethernet_interface;
pub mod ethernet_interface_collection;
pub mod event;
pub mod event_destination;
pub mod event_destination_collection;
pub mod event_service;
pub mod external_account_provider;
pub mod external_account_provider_collection;
pub mod fabric;
pub mod fabric_adapter;
pub mod fabric_adapter_collection;
pub mod fabric_collection;
pub mod facility;
pub mod facility_collection;
pub mod fan;
pub mod fan_collection;
pub mod filter;
pub mod filter_collection;
pub mod graphics_controller;
pub mod graphics_controller_collection;
pub mod heater;
pub mod heater_collection;
pub mod heater_metrics;
pub mod host_interface;
pub mod host_interface_collection;
pub mod ip_addresses;
pub mod job;
pub mod job_collection;
pub mod job_service;
pub mod json_schema_file;
pub mod json_schema_file_collection;
pub mod key;
pub mod key_collection;
pub mod key_policy;
pub mod key_policy_collection;
pub mod key_service;
pub mod leak_detection;
pub mod leak_detector;
pub mod leak_detector_collection;
pub mod license;
pub mod license_collection;
pub mod license_service;
pub mod log_entry;
pub mod log_entry_collection;
pub mod log_service;
pub mod log_service_collection;
pub mod manager;
pub mod manager_account;
pub mod manager_account_collection;
pub mod manager_collection;
pub mod manager_diagnostic_data;
pub mod manager_network_protocol;
pub mod manifest;
pub mod media_controller;
pub mod media_controller_collection;
pub mod memory;
pub mod memory_chunks;
pub mod memory_chunks_collection;
pub mod memory_collection;
pub mod memory_domain;
pub mod memory_domain_collection;
pub mod memory_metrics;
pub mod memory_region;
pub mod memory_region_collection;
pub mod message;
pub mod message_registry_file;
pub mod message_registry_file_collection;
pub mod metric_definition;
pub mod metric_definition_collection;
pub mod metric_report;
pub mod metric_report_collection;
pub mod metric_report_definition;
pub mod metric_report_definition_collection;
pub mod network_adapter;
pub mod network_adapter_collection;
pub mod network_adapter_metrics;
pub mod network_device_function;
pub mod network_device_function_collection;
pub mod network_device_function_metrics;
pub mod network_interface;
pub mod network_interface_collection;
pub mod network_port;
pub mod network_port_collection;
pub mod odata;
pub mod odata_v4;
pub mod operating_config;
pub mod operating_config_collection;
pub mod operating_system;
pub mod outbound_connection;
pub mod outbound_connection_collection;
pub mod outlet;
pub mod outlet_collection;
pub mod outlet_group;
pub mod outlet_group_collection;
pub mod pcie_device;
pub mod pcie_device_collection;
pub mod pcie_function;
pub mod pcie_function_collection;
pub mod pcie_slots;
pub mod physical_context;
pub mod port;
pub mod port_collection;
pub mod port_metrics;
pub mod power;
pub mod power_distribution;
pub mod power_distribution_collection;
pub mod power_distribution_metrics;
pub mod power_domain;
pub mod power_domain_collection;
pub mod power_equipment;
pub mod power_subsystem;
pub mod power_supply;
pub mod power_supply_collection;
pub mod power_supply_metrics;
pub mod privileges;
pub mod processor;
pub mod processor_collection;
pub mod processor_metrics;
pub mod protocol;
pub mod pump;
pub mod pump_collection;
pub mod redundancy;
pub mod registered_client;
pub mod registered_client_collection;
pub mod reservoir;
pub mod reservoir_collection;
pub mod resource;
pub mod resource_block;
pub mod resource_block_collection;
pub mod role;
pub mod role_collection;
pub mod route_entry;
pub mod route_entry_collection;
pub mod route_set_entry;
pub mod route_set_entry_collection;
pub mod schedule;
pub mod secure_boot;
pub mod secure_boot_database;
pub mod secure_boot_database_collection;
pub mod security_policy;
pub mod sensor;
pub mod sensor_collection;
pub mod serial_interface;
pub mod serial_interface_collection;
pub mod service_conditions;
pub mod service_root;
pub mod session;
pub mod session_collection;
pub mod session_service;
pub mod signature;
pub mod signature_collection;
pub mod simple_storage;
pub mod simple_storage_collection;
pub mod software_inventory;
pub mod software_inventory_collection;
pub mod storage;
pub mod storage_collection;
pub mod storage_controller;
pub mod storage_controller_collection;
pub mod storage_controller_metrics;
pub mod switch;
pub mod switch_collection;
pub mod switch_metrics;
pub mod swordfish;
pub mod task;
pub mod task_collection;
pub mod task_service;
pub mod telemetry_service;
pub mod thermal;
pub mod thermal_equipment;
pub mod thermal_metrics;
pub mod thermal_subsystem;
pub mod triggers;
pub mod triggers_collection;
pub mod trusted_component;
pub mod trusted_component_collection;
pub mod update_service;
pub mod usb_controller;
pub mod usb_controller_collection;
pub mod vcat_entry;
pub mod vcat_entry_collection;
pub mod virtual_media;
pub mod virtual_media_collection;
pub mod vlan_network_interface;
pub mod vlan_network_interface_collection;
pub mod zone;
pub mod zone_collection;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GetOdata200Response {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "value")]
    pub value: Vec<crate::GetOdata200ResponseValue>,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GetOdata200ResponseValue {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RedfishError {
    #[serde(rename = "error")]
    pub error: crate::RedfishErrorError,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RedfishErrorError {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "@Message.ExtendedInfo"
    )]
    pub message_extended_info: Option<Vec<crate::message::v1_1_3::Message>>,
}
