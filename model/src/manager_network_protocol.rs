pub mod v1_9_1 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Actions {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Oem")]
        pub oem: Option<crate::manager_network_protocol::v1_9_1::OemActions>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EngineId {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ArchitectureId")]
        pub architecture_id: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnterpriseSpecificMethod"
        )]
        pub enterprise_specific_method: Option<String>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "PrivateEnterpriseId"
        )]
        pub private_enterprise_id: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct HTTPSProtocol {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Certificates")]
        pub certificates: Option<crate::odata_v4::IdRef>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProtocolEnabled")]
        pub protocol_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ManagerNetworkProtocol {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Actions")]
        pub actions: Option<crate::manager_network_protocol::v1_9_1::Actions>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHCPv6")]
        pub dhc_pv6: Option<crate::manager_network_protocol::v1_9_1::Protocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "DHCP")]
        pub dhcp: Option<crate::manager_network_protocol::v1_9_1::Protocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FQDN")]
        pub fqdn: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HostName")]
        pub host_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HTTP")]
        pub http: Option<crate::manager_network_protocol::v1_9_1::Protocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "HTTPS")]
        pub https: Option<crate::manager_network_protocol::v1_9_1::HTTPSProtocol>,
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "IPMI")]
        pub ipmi: Option<crate::manager_network_protocol::v1_9_1::Protocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "KVMIP")]
        pub kvmip: Option<crate::manager_network_protocol::v1_9_1::Protocol>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NTP")]
        pub ntp: Option<crate::manager_network_protocol::v1_9_1::NTPProtocol>,
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
        #[serde(skip_serializing_if = "Option::is_none", rename = "Proxy")]
        pub proxy: Option<crate::manager_network_protocol::v1_9_1::ProxyConfiguration>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RDP")]
        pub rdp: Option<crate::manager_network_protocol::v1_9_1::Protocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RFB")]
        pub rfb: Option<crate::manager_network_protocol::v1_9_1::Protocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SNMP")]
        pub snmp: Option<crate::manager_network_protocol::v1_9_1::SNMPProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSDP")]
        pub ssdp: Option<crate::manager_network_protocol::v1_9_1::SSDProtocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "SSH")]
        pub ssh: Option<crate::manager_network_protocol::v1_9_1::Protocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Status")]
        pub status: Option<crate::resource::Status>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Telnet")]
        pub telnet: Option<crate::manager_network_protocol::v1_9_1::Protocol>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "VirtualMedia")]
        pub virtual_media: Option<crate::manager_network_protocol::v1_9_1::Protocol>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct NTPProtocol {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NetworkSuppliedServers"
        )]
        pub network_supplied_servers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NTPServers")]
        pub ntp_servers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProtocolEnabled")]
        pub protocol_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum NotifyIPv6Scope {
        #[default]
        #[serde(rename = "Link")]
        Link,
        #[serde(rename = "Organization")]
        Organization,
        #[serde(rename = "Site")]
        Site,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct OemActions {}
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Protocol {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProtocolEnabled")]
        pub protocol_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct ProxyConfiguration {
        #[serde(skip_serializing_if = "Option::is_none", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ExcludeAddresses")]
        pub exclude_addresses: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Password")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "PasswordSet")]
        pub password_set: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProxyAutoConfigURI")]
        pub proxy_auto_config_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProxyServerURI")]
        pub proxy_server_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
        pub username: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPAuthenticationProtocols {
        #[default]
        #[serde(rename = "Account")]
        Account,
        #[serde(rename = "CommunityString")]
        CommunityString,
        #[serde(rename = "HMAC128_SHA224")]
        HMAC128SHA224,
        #[serde(rename = "HMAC192_SHA256")]
        HMAC192SHA256,
        #[serde(rename = "HMAC256_SHA384")]
        HMAC256SHA384,
        #[serde(rename = "HMAC384_SHA512")]
        HMAC384SHA512,
        #[serde(rename = "HMAC_MD5")]
        HMACMD5,
        #[serde(rename = "HMAC_SHA96")]
        HMACSHA96,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SNMPCommunity {
        #[serde(skip_serializing_if = "Option::is_none", rename = "AccessMode")]
        pub access_mode: Option<crate::manager_network_protocol::v1_9_1::SNMPCommunityAccessMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CommunityString")]
        pub community_string: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPCommunityAccessMode {
        #[default]
        #[serde(rename = "Full")]
        Full,
        #[serde(rename = "Limited")]
        Limited,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum SNMPEncryptionProtocols {
        #[default]
        #[serde(rename = "Account")]
        Account,
        #[serde(rename = "CBC_DES")]
        CBCDES,
        #[serde(rename = "CFB128_AES128")]
        CFB128AES128,
        #[serde(rename = "None")]
        None,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SNMPProtocol {
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "AuthenticationProtocol"
        )]
        pub authentication_protocol:
            Option<crate::manager_network_protocol::v1_9_1::SNMPAuthenticationProtocols>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "CommunityAccessMode"
        )]
        pub community_access_mode:
            Option<crate::manager_network_protocol::v1_9_1::SNMPCommunityAccessMode>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "CommunityStrings")]
        pub community_strings: Option<Vec<crate::manager_network_protocol::v1_9_1::SNMPCommunity>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnableSNMPv1")]
        pub enable_snm_pv1: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnableSNMPv2c")]
        pub enable_snm_pv2c: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnableSNMPv3")]
        pub enable_snm_pv3: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EncryptionProtocol")]
        pub encryption_protocol:
            Option<crate::manager_network_protocol::v1_9_1::SNMPEncryptionProtocols>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EngineId")]
        pub engine_id: Option<crate::manager_network_protocol::v1_9_1::EngineId>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "HideCommunityStrings"
        )]
        pub hide_community_strings: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProtocolEnabled")]
        pub protocol_enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct SSDProtocol {
        #[serde(skip_serializing_if = "Option::is_none", rename = "NotifyIPv6Scope")]
        pub notify_ipv6_scope: Option<crate::manager_network_protocol::v1_9_1::NotifyIPv6Scope>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "NotifyMulticastIntervalSeconds"
        )]
        pub notify_multicast_interval_seconds: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "NotifyTTL")]
        pub notify_ttl: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Port")]
        pub port: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ProtocolEnabled")]
        pub protocol_enabled: Option<bool>,
    }
}
