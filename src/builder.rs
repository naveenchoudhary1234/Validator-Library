use crate::config::{
    Filesystem, Host, HostRole, NetworkConfig, NetworkInterface, NetworkType,
    ServerConfig, StorageConfig,
};
use crate::error::ValidationError;
use crate::types::ByteSize;
use crate::validation::Validate;

// ===========================
// ServerConfig Builder
// ===========================

pub struct ServerConfigBuilder {
    name: Option<String>,
    hosts: Vec<Host>,
    filesystems: Vec<Filesystem>,
    default_size: Option<ByteSize>,
    interfaces: Vec<NetworkInterface>,
}

impl ServerConfigBuilder {
    // Naya builder banao
    pub fn new() -> Self {
        ServerConfigBuilder {
            name: None,
            hosts: Vec::new(),
            filesystems: Vec::new(),
            default_size: None,
            interfaces: Vec::new(),
        }
    }

    // Server ka naam set karo
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    // Ek host add karo
    pub fn add_host(mut self, host: Host) -> Self {
        self.hosts.push(host);
        self
    }

    // Ek filesystem add karo
    pub fn add_filesystem(mut self, filesystem: Filesystem) -> Self {
        self.filesystems.push(filesystem);
        self
    }

    // Default size set karo
    pub fn default_size(mut self, size: &str) -> Result<Self, ValidationError> {
        let parsed = size.parse::<ByteSize>()?;
        self.default_size = Some(parsed);
        Ok(self)
    }

    // Ek network interface add karo
    pub fn add_interface(mut self, interface: NetworkInterface) -> Self {
        self.interfaces.push(interface);
        self
    }

    // Final config banao — validate bhi karo
    pub fn build(self) -> Result<ServerConfig, ValidationError> {
        // name check
        let name = self.name.ok_or_else(|| ValidationError::EmptyConfiguration {
            field: "name".to_string(),
        })?;

        // default_size check
        let default_size = self.default_size.ok_or_else(|| {
            ValidationError::EmptyConfiguration {
                field: "default_size".to_string(),
            }
        })?;

        let config = ServerConfig {
            name,
            hosts: self.hosts,
            storage: StorageConfig {
                filesystems: self.filesystems,
                default_size,
            },
            network: NetworkConfig {
                interfaces: self.interfaces,
            },
        };

        // validate karo before returning
        config.validate()?;

        Ok(config)
    }
}

impl Default for ServerConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ===========================
// Host Builder
// ===========================

pub struct HostBuilder {
    hostname: Option<String>,
    ip_address: Option<String>,
    role: Option<HostRole>,
    enabled: bool,
}

impl HostBuilder {
    pub fn new() -> Self {
        HostBuilder {
            hostname: None,
            ip_address: None,
            role: None,
            enabled: true, // default enabled
        }
    }

    pub fn hostname(mut self, hostname: &str) -> Self {
        self.hostname = Some(hostname.to_string());
        self
    }

    pub fn ip_address(mut self, ip: &str) -> Self {
        self.ip_address = Some(ip.to_string());
        self
    }

    pub fn role(mut self, role: HostRole) -> Self {
        self.role = Some(role);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn build(self) -> Result<Host, ValidationError> {
        let hostname = self.hostname.ok_or_else(|| ValidationError::EmptyConfiguration {
            field: "hostname".to_string(),
        })?;

        let ip_address = self.ip_address.ok_or_else(|| ValidationError::EmptyConfiguration {
            field: "ip_address".to_string(),
        })?;

        let role = self.role.ok_or_else(|| ValidationError::EmptyConfiguration {
            field: "role".to_string(),
        })?;

        let host = Host {
            hostname,
            ip_address,
            role,
            enabled: self.enabled,
        };

        host.validate()?;

        Ok(host)
    }
}

impl Default for HostBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ===========================
// Filesystem Builder
// ===========================

pub struct FilesystemBuilder {
    name: Option<String>,
    mount_point: Option<String>,
    size: Option<ByteSize>,
}

impl FilesystemBuilder {
    pub fn new() -> Self {
        FilesystemBuilder {
            name: None,
            mount_point: None,
            size: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn mount_point(mut self, mount_point: &str) -> Self {
        self.mount_point = Some(mount_point.to_string());
        self
    }

    pub fn size(mut self, size: &str) -> Result<Self, ValidationError> {
        let parsed = size.parse::<ByteSize>()?;
        self.size = Some(parsed);
        Ok(self)
    }

    pub fn build(self) -> Result<Filesystem, ValidationError> {
        let name = self.name.ok_or_else(|| ValidationError::EmptyConfiguration {
            field: "filesystem name".to_string(),
        })?;

        let mount_point =
            self.mount_point.ok_or_else(|| ValidationError::EmptyConfiguration {
                field: "mount_point".to_string(),
            })?;

        let size = self.size.ok_or_else(|| ValidationError::EmptyConfiguration {
            field: "size".to_string(),
        })?;

        let fs = Filesystem {
            name,
            mount_point,
            size,
        };

        fs.validate()?;

        Ok(fs)
    }
}

impl Default for FilesystemBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ===========================
// NetworkInterface Builder
// ===========================

pub struct NetworkInterfaceBuilder {
    name: Option<String>,
    network_type: Option<NetworkType>,
    speed_gbps: Option<u32>,
}

impl NetworkInterfaceBuilder {
    pub fn new() -> Self {
        NetworkInterfaceBuilder {
            name: None,
            network_type: None,
            speed_gbps: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn network_type(mut self, network_type: NetworkType) -> Self {
        self.network_type = Some(network_type);
        self
    }

    pub fn speed_gbps(mut self, speed: u32) -> Self {
        self.speed_gbps = Some(speed);
        self
    }

    pub fn build(self) -> Result<NetworkInterface, ValidationError> {
        let name = self.name.ok_or_else(|| ValidationError::EmptyConfiguration {
            field: "interface name".to_string(),
        })?;

        let network_type =
            self.network_type.ok_or_else(|| ValidationError::EmptyConfiguration {
                field: "network_type".to_string(),
            })?;

        let speed_gbps =
            self.speed_gbps.ok_or_else(|| ValidationError::EmptyConfiguration {
                field: "speed_gbps".to_string(),
            })?;

        let iface = NetworkInterface {
            name,
            network_type,
            speed_gbps,
        };

        iface.validate()?;

        Ok(iface)
    }
}

impl Default for NetworkInterfaceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_valid_config() {
        let host = HostBuilder::new()
            .hostname("manager-01")
            .ip_address("192.168.1.10")
            .role(HostRole::Manager)
            .enabled(true)
            .build()
            .unwrap();

        let fs = FilesystemBuilder::new()
            .name("data_fs")
            .mount_point("/mnt/data")
            .size("100G")
            .unwrap()
            .build()
            .unwrap();

        let iface = NetworkInterfaceBuilder::new()
            .name("eth0")
            .network_type(NetworkType::Ethernet)
            .speed_gbps(10)
            .build()
            .unwrap();

        let config = ServerConfigBuilder::new()
            .name("production-cluster")
            .add_host(host)
            .add_filesystem(fs)
            .default_size("512M")
            .unwrap()
            .add_interface(iface)
            .build()
            .unwrap();

        assert_eq!(config.name, "production-cluster");
        assert_eq!(config.hosts.len(), 1);
        assert_eq!(config.storage.filesystems.len(), 1);
        assert_eq!(config.network.interfaces.len(), 1);
    }

    #[test]
    fn test_build_missing_name() {
        let result = ServerConfigBuilder::new()
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_build_invalid_host() {
        let result = HostBuilder::new()
            .hostname("-bad-host")
            .ip_address("192.168.1.10")
            .role(HostRole::Manager)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_build_invalid_ip() {
        let result = HostBuilder::new()
            .hostname("manager-01")
            .ip_address("999.999.999.999")
            .role(HostRole::Manager)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_build_invalid_filesystem() {
        let result = FilesystemBuilder::new()
            .name("bad-name!")
            .mount_point("/mnt/data")
            .size("100G")
            .unwrap()
            .build();
        assert!(result.is_err());
    }
}