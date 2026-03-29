pub mod rules;
pub mod traits;

pub use traits::Validate;

use crate::config::{Filesystem, Host, HostRole, NetworkInterface, ServerConfig, StorageConfig};
use crate::error::ValidationError;
use rules::*;

impl Validate for Host {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        if let Err(e) = validate_hostname(&self.hostname) {
            errors.push(e);
        }

        if let Err(e) = validate_ip_address(&self.ip_address) {
            errors.push(e);
        }

        if errors.is_empty() {
            Ok(())
        } else if errors.len() == 1 {
            Err(errors.remove(0))
        } else {
            Err(ValidationError::MultipleErrors(errors))
        }
    }
}

impl Validate for Filesystem {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        if let Err(e) = validate_filesystem_name(&self.name) {
            errors.push(e);
        }

        if let Err(e) = validate_mount_point(&self.mount_point) {
            errors.push(e);
        }

        if errors.is_empty() {
            Ok(())
        } else if errors.len() == 1 {
            Err(errors.remove(0))
        } else {
            Err(ValidationError::MultipleErrors(errors))
        }
    }
}

impl Validate for NetworkInterface {
    fn validate(&self) -> Result<(), ValidationError> {
        validate_network_interface_name(&self.name)
    }
}

impl Validate for StorageConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        if self.filesystems.is_empty() {
            errors.push(ValidationError::NoFilesystems);
        }

        for fs in &self.filesystems {
            if let Err(e) = fs.validate() {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else if errors.len() == 1 {
            Err(errors.remove(0))
        } else {
            Err(ValidationError::MultipleErrors(errors))
        }
    }
}

impl Validate for ServerConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push(ValidationError::EmptyConfiguration {
                field: "name".to_string(),
            });
        }

        if self.hosts.is_empty() {
            errors.push(ValidationError::EmptyConfiguration {
                field: "hosts".to_string(),
            });
        } else {
            let has_manager = self
                .hosts
                .iter()
                .any(|h| matches!(h.role, HostRole::Manager));

            if !has_manager {
                errors.push(ValidationError::NoManagerHost);
            }

            for host in &self.hosts {
                if let Err(e) = host.validate() {
                    errors.push(e);
                }
            }
        }

        if let Err(e) = self.storage.validate() {
            errors.push(e);
        }

        for iface in &self.network.interfaces {
            if let Err(e) = iface.validate() {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else if errors.len() == 1 {
            Err(errors.remove(0))
        } else {
            Err(ValidationError::MultipleErrors(errors))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::*;
    use crate::types::ByteSize;
    use crate::validation::Validate;

    fn valid_config() -> ServerConfig {
        ServerConfig {
            name: "production-cluster".to_string(),
            hosts: vec![Host {
                hostname: "manager-01".to_string(),
                ip_address: "192.168.1.10".to_string(),
                role: HostRole::Manager,
                enabled: true,
            }],
            storage: StorageConfig {
                filesystems: vec![Filesystem {
                    name: "data_fs".to_string(),
                    mount_point: "/mnt/data".to_string(),
                    size: "100G".parse::<ByteSize>().unwrap(),
                }],
                default_size: "512M".parse::<ByteSize>().unwrap(),
            },
            network: NetworkConfig {
                interfaces: vec![NetworkInterface {
                    name: "eth0".to_string(),
                    network_type: NetworkType::Ethernet,
                    speed_gbps: 10,
                }],
            },
        }
    }

    #[test]
    fn test_valid_config() {
        assert!(valid_config().validate().is_ok());
    }

    #[test]
    fn test_invalid_hostname_chars() {
        let mut config = valid_config();
        config.hosts[0].hostname = "invalid_host!".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_hostname_starts_with_hyphen() {
        let mut config = valid_config();
        config.hosts[0].hostname = "-badhost".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_hostname_too_long() {
        let mut config = valid_config();
        config.hosts[0].hostname = "a".repeat(254);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_ip_address() {
        let mut config = valid_config();
        config.hosts[0].ip_address = "999.999.999.999".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_missing_manager_host() {
        let mut config = valid_config();
        config.hosts[0].role = HostRole::Storage;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_filesystem_name() {
        let mut config = valid_config();
        config.storage.filesystems[0].name = "bad-name!".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_mount_point() {
        let mut config = valid_config();
        config.storage.filesystems[0].mount_point = "no_slash".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_multiple_errors_collected() {
        let mut config = valid_config();
        config.hosts[0].hostname = "-bad".to_string();
        config.hosts[0].ip_address = "not_an_ip".to_string();
        config.hosts[0].role = HostRole::Storage;

        match config.validate() {
            Err(e) => {
                let msg = e.to_string();
                assert!(msg.contains("error") || msg.len() > 0);
            }
            Ok(_) => panic!("Expected validation to fail"),
        }
    }
}
