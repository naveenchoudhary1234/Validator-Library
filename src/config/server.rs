use crate::config::host::Host;
use crate::config::network::NetworkConfig;
use crate::config::storage::StorageConfig;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub name: String,
    pub hosts: Vec<Host>,
    pub storage: StorageConfig,
    pub network: NetworkConfig,
}
