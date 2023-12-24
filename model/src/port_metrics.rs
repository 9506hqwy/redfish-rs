use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PortMetrics {
    V010400(crate::port_metrics::v1_4_0::PortMetrics),
    V010300(crate::port_metrics::v1_3_0::PortMetrics),
    V010200(crate::port_metrics::v1_2_0::PortMetrics),
    V010101(crate::port_metrics::v1_1_1::PortMetrics),
    V010002(crate::port_metrics::v1_0_2::PortMetrics),
    OdataV4IdRef(crate::odata_v4::IdRef),
}
pub mod v1_0_2 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::port_metrics::v1_0_2::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccessKeyViolations"
        )]
        pub access_key_violations: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndToEndCRCErrors")]
        pub end_to_end_crc_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkNTE")]
        pub link_nte: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLRRecovery")]
        pub llr_recovery: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MarkedECN")]
        pub marked_ecn: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NonCRCTransientErrors"
        )]
        pub non_crc_transient_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PacketCRCErrors")]
        pub packet_crc_errors: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PacketDeadlineDiscards"
        )]
        pub packet_deadline_discards: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReceivedECN")]
        pub received_ecn: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXStompedECRC")]
        pub rx_stomped_ecrc: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXStompedECRC")]
        pub tx_stomped_ecrc: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PortMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::port_metrics::v1_0_2::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::port_metrics::v1_0_2::GenZ>,
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
pub mod v1_1_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::port_metrics::v1_1_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccessKeyViolations"
        )]
        pub access_key_violations: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndToEndCRCErrors")]
        pub end_to_end_crc_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkNTE")]
        pub link_nte: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLRRecovery")]
        pub llr_recovery: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MarkedECN")]
        pub marked_ecn: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NonCRCTransientErrors"
        )]
        pub non_crc_transient_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PacketCRCErrors")]
        pub packet_crc_errors: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PacketDeadlineDiscards"
        )]
        pub packet_deadline_discards: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReceivedECN")]
        pub received_ecn: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXStompedECRC")]
        pub rx_stomped_ecrc: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXStompedECRC")]
        pub tx_stomped_ecrc: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Networking {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RDMAProtectionErrors"
        )]
        pub rdma_protection_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMAProtocolErrors")]
        pub rdma_protocol_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMARXBytes")]
        pub rdmarx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMARXRequests")]
        pub rdmarx_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXBytes")]
        pub rdmatx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXReadRequests")]
        pub rdmatx_read_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXRequests")]
        pub rdmatx_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXSendRequests")]
        pub rdmatx_send_requests: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RDMATXWriteRequests"
        )]
        pub rdmatx_write_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXBroadcastFrames")]
        pub rx_broadcast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXDiscards")]
        pub rx_discards: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXFalseCarrierErrors"
        )]
        pub rx_false_carrier_errors: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXFrameAlignmentErrors"
        )]
        pub rx_frame_alignment_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXFrames")]
        pub rx_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXMulticastFrames")]
        pub rx_multicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXOversizeFrames")]
        pub rx_oversize_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXPauseXOFFFrames")]
        pub rx_pause_xoff_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXPauseXONFrames")]
        pub rx_pause_xon_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXUndersizeFrames")]
        pub rx_undersize_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXUnicastFrames")]
        pub rx_unicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXFCSErrors")]
        pub rxfcs_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXPFCFrames")]
        pub rxpfc_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBroadcastFrames")]
        pub tx_broadcast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXDiscards")]
        pub tx_discards: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXExcessiveCollisions"
        )]
        pub tx_excessive_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXFrames")]
        pub tx_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXLateCollisions")]
        pub tx_late_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXMulticastFrames")]
        pub tx_multicast_frames: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXMultipleCollisions"
        )]
        pub tx_multiple_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXPauseXOFFFrames")]
        pub tx_pause_xoff_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXPauseXONFrames")]
        pub tx_pause_xon_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXSingleCollisions")]
        pub tx_single_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXUnicastFrames")]
        pub tx_unicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXPFCFrames")]
        pub txpfc_frames: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PortMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::port_metrics::v1_1_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::port_metrics::v1_1_1::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Networking")]
        pub networking: Option<crate::port_metrics::v1_1_1::Networking>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXBytes")]
        pub rx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXErrors")]
        pub rx_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SAS")]
        pub sas: Option<Vec<crate::port_metrics::v1_1_1::SAS>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Transceivers")]
        pub transceivers: Option<Vec<crate::port_metrics::v1_1_1::Transceiver>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBytes")]
        pub tx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXErrors")]
        pub tx_errors: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SAS {
        #[serde(skip_serializing_if = "Option::is_none", rename = "InvalidDwordCount")]
        pub invalid_dword_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LossOfDwordSynchronizationCount"
        )]
        pub loss_of_dword_synchronization_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RunningDisparityErrorCount"
        )]
        pub running_disparity_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Transceiver {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXInputPowerMilliWatts"
        )]
        pub rx_input_power_milli_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupplyVoltage")]
        pub supply_voltage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXBiasCurrentMilliAmps"
        )]
        pub tx_bias_current_milli_amps: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXOutputPowerMilliWatts"
        )]
        pub tx_output_power_milli_watts: Option<f64>,
    }
}
pub mod v1_2_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::port_metrics::v1_2_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FibreChannel {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableFECErrors"
        )]
        pub correctable_fec_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InvalidCRCs")]
        pub invalid_cr_cs: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InvalidTXWords")]
        pub invalid_tx_words: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkFailures")]
        pub link_failures: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LossesOfSignal")]
        pub losses_of_signal: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LossesOfSync")]
        pub losses_of_sync: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXExchanges")]
        pub rx_exchanges: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXSequences")]
        pub rx_sequences: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXBBCreditZero")]
        pub rxbb_credit_zero: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXExchanges")]
        pub tx_exchanges: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXSequences")]
        pub tx_sequences: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBBCreditZero")]
        pub txbb_credit_zero: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXBBCreditZeroDurationMilliseconds"
        )]
        pub txbb_credit_zero_duration_milliseconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBBCredits")]
        pub txbb_credits: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableFECErrors"
        )]
        pub uncorrectable_fec_errors: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccessKeyViolations"
        )]
        pub access_key_violations: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndToEndCRCErrors")]
        pub end_to_end_crc_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkNTE")]
        pub link_nte: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLRRecovery")]
        pub llr_recovery: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MarkedECN")]
        pub marked_ecn: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NonCRCTransientErrors"
        )]
        pub non_crc_transient_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PacketCRCErrors")]
        pub packet_crc_errors: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PacketDeadlineDiscards"
        )]
        pub packet_deadline_discards: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReceivedECN")]
        pub received_ecn: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXStompedECRC")]
        pub rx_stomped_ecrc: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXStompedECRC")]
        pub tx_stomped_ecrc: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Networking {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RDMAProtectionErrors"
        )]
        pub rdma_protection_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMAProtocolErrors")]
        pub rdma_protocol_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMARXBytes")]
        pub rdmarx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMARXRequests")]
        pub rdmarx_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXBytes")]
        pub rdmatx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXReadRequests")]
        pub rdmatx_read_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXRequests")]
        pub rdmatx_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXSendRequests")]
        pub rdmatx_send_requests: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RDMATXWriteRequests"
        )]
        pub rdmatx_write_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXBroadcastFrames")]
        pub rx_broadcast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXDiscards")]
        pub rx_discards: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXFalseCarrierErrors"
        )]
        pub rx_false_carrier_errors: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXFrameAlignmentErrors"
        )]
        pub rx_frame_alignment_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXFrames")]
        pub rx_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXMulticastFrames")]
        pub rx_multicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXOversizeFrames")]
        pub rx_oversize_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXPauseXOFFFrames")]
        pub rx_pause_xoff_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXPauseXONFrames")]
        pub rx_pause_xon_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXUndersizeFrames")]
        pub rx_undersize_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXUnicastFrames")]
        pub rx_unicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXFCSErrors")]
        pub rxfcs_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXPFCFrames")]
        pub rxpfc_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBroadcastFrames")]
        pub tx_broadcast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXDiscards")]
        pub tx_discards: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXExcessiveCollisions"
        )]
        pub tx_excessive_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXFrames")]
        pub tx_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXLateCollisions")]
        pub tx_late_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXMulticastFrames")]
        pub tx_multicast_frames: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXMultipleCollisions"
        )]
        pub tx_multiple_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXPauseXOFFFrames")]
        pub tx_pause_xoff_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXPauseXONFrames")]
        pub tx_pause_xon_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXSingleCollisions")]
        pub tx_single_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXUnicastFrames")]
        pub tx_unicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXPFCFrames")]
        pub txpfc_frames: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PortMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::port_metrics::v1_2_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FibreChannel")]
        pub fibre_channel: Option<crate::port_metrics::v1_2_0::FibreChannel>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::port_metrics::v1_2_0::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Networking")]
        pub networking: Option<crate::port_metrics::v1_2_0::Networking>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXBytes")]
        pub rx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXErrors")]
        pub rx_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SAS")]
        pub sas: Option<Vec<crate::port_metrics::v1_2_0::SAS>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Transceivers")]
        pub transceivers: Option<Vec<crate::port_metrics::v1_2_0::Transceiver>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBytes")]
        pub tx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXErrors")]
        pub tx_errors: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SAS {
        #[serde(skip_serializing_if = "Option::is_none", rename = "InvalidDwordCount")]
        pub invalid_dword_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LossOfDwordSynchronizationCount"
        )]
        pub loss_of_dword_synchronization_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RunningDisparityErrorCount"
        )]
        pub running_disparity_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Transceiver {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXInputPowerMilliWatts"
        )]
        pub rx_input_power_milli_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupplyVoltage")]
        pub supply_voltage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXBiasCurrentMilliAmps"
        )]
        pub tx_bias_current_milli_amps: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXOutputPowerMilliWatts"
        )]
        pub tx_output_power_milli_watts: Option<f64>,
    }
}
pub mod v1_3_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::port_metrics::v1_3_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FibreChannel {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableFECErrors"
        )]
        pub correctable_fec_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InvalidCRCs")]
        pub invalid_cr_cs: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InvalidTXWords")]
        pub invalid_tx_words: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkFailures")]
        pub link_failures: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LossesOfSignal")]
        pub losses_of_signal: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LossesOfSync")]
        pub losses_of_sync: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXExchanges")]
        pub rx_exchanges: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXSequences")]
        pub rx_sequences: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXBBCreditZero")]
        pub rxbb_credit_zero: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXExchanges")]
        pub tx_exchanges: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXSequences")]
        pub tx_sequences: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBBCreditZero")]
        pub txbb_credit_zero: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXBBCreditZeroDurationMilliseconds"
        )]
        pub txbb_credit_zero_duration_milliseconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBBCredits")]
        pub txbb_credits: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableFECErrors"
        )]
        pub uncorrectable_fec_errors: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccessKeyViolations"
        )]
        pub access_key_violations: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndToEndCRCErrors")]
        pub end_to_end_crc_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkNTE")]
        pub link_nte: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLRRecovery")]
        pub llr_recovery: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MarkedECN")]
        pub marked_ecn: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NonCRCTransientErrors"
        )]
        pub non_crc_transient_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PacketCRCErrors")]
        pub packet_crc_errors: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PacketDeadlineDiscards"
        )]
        pub packet_deadline_discards: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReceivedECN")]
        pub received_ecn: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXStompedECRC")]
        pub rx_stomped_ecrc: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXStompedECRC")]
        pub tx_stomped_ecrc: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Networking {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RDMAProtectionErrors"
        )]
        pub rdma_protection_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMAProtocolErrors")]
        pub rdma_protocol_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMARXBytes")]
        pub rdmarx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMARXRequests")]
        pub rdmarx_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXBytes")]
        pub rdmatx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXReadRequests")]
        pub rdmatx_read_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXRequests")]
        pub rdmatx_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXSendRequests")]
        pub rdmatx_send_requests: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RDMATXWriteRequests"
        )]
        pub rdmatx_write_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXBroadcastFrames")]
        pub rx_broadcast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXDiscards")]
        pub rx_discards: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXFalseCarrierErrors"
        )]
        pub rx_false_carrier_errors: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXFrameAlignmentErrors"
        )]
        pub rx_frame_alignment_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXFrames")]
        pub rx_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXMulticastFrames")]
        pub rx_multicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXOversizeFrames")]
        pub rx_oversize_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXPauseXOFFFrames")]
        pub rx_pause_xoff_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXPauseXONFrames")]
        pub rx_pause_xon_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXUndersizeFrames")]
        pub rx_undersize_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXUnicastFrames")]
        pub rx_unicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXFCSErrors")]
        pub rxfcs_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXPFCFrames")]
        pub rxpfc_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBroadcastFrames")]
        pub tx_broadcast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXDiscards")]
        pub tx_discards: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXExcessiveCollisions"
        )]
        pub tx_excessive_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXFrames")]
        pub tx_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXLateCollisions")]
        pub tx_late_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXMulticastFrames")]
        pub tx_multicast_frames: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXMultipleCollisions"
        )]
        pub tx_multiple_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXPauseXOFFFrames")]
        pub tx_pause_xoff_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXPauseXONFrames")]
        pub tx_pause_xon_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXSingleCollisions")]
        pub tx_single_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXUnicastFrames")]
        pub tx_unicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXPFCFrames")]
        pub txpfc_frames: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PortMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::port_metrics::v1_3_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FibreChannel")]
        pub fibre_channel: Option<crate::port_metrics::v1_3_0::FibreChannel>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::port_metrics::v1_3_0::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Networking")]
        pub networking: Option<crate::port_metrics::v1_3_0::Networking>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeErrors")]
        pub pcie_errors: Option<crate::pcie_device::PCIeErrors>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXBytes")]
        pub rx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXErrors")]
        pub rx_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SAS")]
        pub sas: Option<Vec<crate::port_metrics::v1_3_0::SAS>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Transceivers")]
        pub transceivers: Option<Vec<crate::port_metrics::v1_3_0::Transceiver>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBytes")]
        pub tx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXErrors")]
        pub tx_errors: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SAS {
        #[serde(skip_serializing_if = "Option::is_none", rename = "InvalidDwordCount")]
        pub invalid_dword_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LossOfDwordSynchronizationCount"
        )]
        pub loss_of_dword_synchronization_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RunningDisparityErrorCount"
        )]
        pub running_disparity_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Transceiver {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXInputPowerMilliWatts"
        )]
        pub rx_input_power_milli_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupplyVoltage")]
        pub supply_voltage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXBiasCurrentMilliAmps"
        )]
        pub tx_bias_current_milli_amps: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXOutputPowerMilliWatts"
        )]
        pub tx_output_power_milli_watts: Option<f64>,
    }
}
pub mod v1_4_0 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::port_metrics::v1_4_0::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct CXL {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "BackpressureAveragePercentage"
        )]
        pub backpressure_average_percentage: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct FibreChannel {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CorrectableFECErrors"
        )]
        pub correctable_fec_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InvalidCRCs")]
        pub invalid_cr_cs: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InvalidTXWords")]
        pub invalid_tx_words: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkFailures")]
        pub link_failures: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LossesOfSignal")]
        pub losses_of_signal: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LossesOfSync")]
        pub losses_of_sync: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXExchanges")]
        pub rx_exchanges: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXSequences")]
        pub rx_sequences: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXBBCreditZero")]
        pub rxbb_credit_zero: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXExchanges")]
        pub tx_exchanges: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXSequences")]
        pub tx_sequences: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBBCreditZero")]
        pub txbb_credit_zero: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXBBCreditZeroDurationMilliseconds"
        )]
        pub txbb_credit_zero_duration_milliseconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBBCredits")]
        pub txbb_credits: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "UncorrectableFECErrors"
        )]
        pub uncorrectable_fec_errors: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct GenZ {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AccessKeyViolations"
        )]
        pub access_key_violations: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EndToEndCRCErrors")]
        pub end_to_end_crc_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LinkNTE")]
        pub link_nte: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "LLRRecovery")]
        pub llr_recovery: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MarkedECN")]
        pub marked_ecn: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NonCRCTransientErrors"
        )]
        pub non_crc_transient_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PacketCRCErrors")]
        pub packet_crc_errors: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PacketDeadlineDiscards"
        )]
        pub packet_deadline_discards: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ReceivedECN")]
        pub received_ecn: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXStompedECRC")]
        pub rx_stomped_ecrc: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXStompedECRC")]
        pub tx_stomped_ecrc: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Networking {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RDMAProtectionErrors"
        )]
        pub rdma_protection_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMAProtocolErrors")]
        pub rdma_protocol_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMARXBytes")]
        pub rdmarx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMARXRequests")]
        pub rdmarx_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXBytes")]
        pub rdmatx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXReadRequests")]
        pub rdmatx_read_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXRequests")]
        pub rdmatx_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDMATXSendRequests")]
        pub rdmatx_send_requests: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RDMATXWriteRequests"
        )]
        pub rdmatx_write_requests: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXBroadcastFrames")]
        pub rx_broadcast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXDiscards")]
        pub rx_discards: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXFalseCarrierErrors"
        )]
        pub rx_false_carrier_errors: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXFrameAlignmentErrors"
        )]
        pub rx_frame_alignment_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXFrames")]
        pub rx_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXMulticastFrames")]
        pub rx_multicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXOversizeFrames")]
        pub rx_oversize_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXPauseXOFFFrames")]
        pub rx_pause_xoff_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXPauseXONFrames")]
        pub rx_pause_xon_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXUndersizeFrames")]
        pub rx_undersize_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXUnicastFrames")]
        pub rx_unicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXFCSErrors")]
        pub rxfcs_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXPFCFrames")]
        pub rxpfc_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBroadcastFrames")]
        pub tx_broadcast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXDiscards")]
        pub tx_discards: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXExcessiveCollisions"
        )]
        pub tx_excessive_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXFrames")]
        pub tx_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXLateCollisions")]
        pub tx_late_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXMulticastFrames")]
        pub tx_multicast_frames: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXMultipleCollisions"
        )]
        pub tx_multiple_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXPauseXOFFFrames")]
        pub tx_pause_xoff_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXPauseXONFrames")]
        pub tx_pause_xon_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXSingleCollisions")]
        pub tx_single_collisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXUnicastFrames")]
        pub tx_unicast_frames: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXPFCFrames")]
        pub txpfc_frames: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct PortMetrics {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::port_metrics::v1_4_0::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CXL")]
        pub cxl: Option<crate::port_metrics::v1_4_0::CXL>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FibreChannel")]
        pub fibre_channel: Option<crate::port_metrics::v1_4_0::FibreChannel>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "GenZ")]
        pub gen_z: Option<crate::port_metrics::v1_4_0::GenZ>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Networking")]
        pub networking: Option<crate::port_metrics::v1_4_0::Networking>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "PCIeErrors")]
        pub pcie_errors: Option<crate::pcie_device::PCIeErrors>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXBytes")]
        pub rx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RXErrors")]
        pub rx_errors: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SAS")]
        pub sas: Option<Vec<crate::port_metrics::v1_4_0::SAS>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Transceivers")]
        pub transceivers: Option<Vec<crate::port_metrics::v1_4_0::Transceiver>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXBytes")]
        pub tx_bytes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "TXErrors")]
        pub tx_errors: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SAS {
        #[serde(skip_serializing_if = "Option::is_none", rename = "InvalidDwordCount")]
        pub invalid_dword_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "LossOfDwordSynchronizationCount"
        )]
        pub loss_of_dword_synchronization_count: Option<i64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RunningDisparityErrorCount"
        )]
        pub running_disparity_error_count: Option<i64>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Transceiver {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "RXInputPowerMilliWatts"
        )]
        pub rx_input_power_milli_watts: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SupplyVoltage")]
        pub supply_voltage: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXBiasCurrentMilliAmps"
        )]
        pub tx_bias_current_milli_amps: Option<f64>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "TXOutputPowerMilliWatts"
        )]
        pub tx_output_power_milli_watts: Option<f64>,
    }
}
