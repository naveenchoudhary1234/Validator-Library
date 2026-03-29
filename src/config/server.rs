use serde::{Deserialize, Serialize};
use crate::config::host::Host;
use crate::config::storage::StorageConfig;
use crate::config::network::NetworkConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub hosts: Vec<Host>,
    pub storage: StorageConfig,
    pub network: NetworkConfig,
}