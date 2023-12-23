use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum PrivilegeType {
    #[default]
    #[serde(rename = "AdministrateStorage")]
    AdministrateStorage,
    #[serde(rename = "AdministrateSystems")]
    AdministrateSystems,
    #[serde(rename = "ConfigureComponents")]
    ConfigureComponents,
    #[serde(rename = "ConfigureCompositionInfrastructure")]
    ConfigureCompositionInfrastructure,
    #[serde(rename = "ConfigureManager")]
    ConfigureManager,
    #[serde(rename = "ConfigureSelf")]
    ConfigureSelf,
    #[serde(rename = "ConfigureUsers")]
    ConfigureUsers,
    #[serde(rename = "Login")]
    Login,
    #[serde(rename = "NoAuth")]
    NoAuth,
    #[serde(rename = "OperateStorageBackup")]
    OperateStorageBackup,
    #[serde(rename = "OperateSystems")]
    OperateSystems,
}
