use crate::error::ValidationError;

pub fn validate_hostname(hostname: &str) -> Result<(), ValidationError> {
    if hostname.is_empty() || hostname.len() > 253 {
        return Err(ValidationError::InvalidHostname {
            hostname: hostname.to_string(),
            reason: "hostname must be between 1 and 253 characters".to_string(),
        });
    }

    if hostname.starts_with('-') || hostname.ends_with('-') {
        return Err(ValidationError::InvalidHostname {
            hostname: hostname.to_string(),
            reason: "hostname cannot start or end with a hyphen".to_string(),
        });
    }

    if !hostname.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err(ValidationError::InvalidHostname {
            hostname: hostname.to_string(),
            reason: "hostname can only contain alphanumeric characters and hyphens".to_string(),
        });
    }

    Ok(())
}

pub fn validate_ip_address(ip: &str) -> Result<(), ValidationError> {
    let parts: Vec<&str> = ip.split('.').collect();

    if parts.len() != 4 {
        return Err(ValidationError::InvalidIpAddress {
            ip: ip.to_string(),
            reason: "IPv4 address must have exactly 4 octets".to_string(),
        });
    }

    for part in &parts {
        match part.parse::<u8>() {
            Ok(_) => {}
            Err(_) => {
                return Err(ValidationError::InvalidIpAddress {
                    ip: ip.to_string(),
                    reason: format!("'{}' is not a valid octet (0-255)", part),
                });
            }
        }
    }

    Ok(())
}

pub fn validate_filesystem_name(name: &str) -> Result<(), ValidationError> {
    if name.is_empty() || name.len() > 64 {
        return Err(ValidationError::InvalidFilesystemName {
            name: name.to_string(),
            reason: "filesystem name must be between 1 and 64 characters".to_string(),
        });
    }

    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(ValidationError::InvalidFilesystemName {
            name: name.to_string(),
            reason: "filesystem name can only contain alphanumeric characters and underscores"
                .to_string(),
        });
    }

    Ok(())
}

pub fn validate_mount_point(path: &str) -> Result<(), ValidationError> {
    if !path.starts_with('/') {
        return Err(ValidationError::InvalidMountPoint {
            path: path.to_string(),
            reason: "mount point must start with '/'".to_string(),
        });
    }

    Ok(())
}

pub fn validate_network_interface_name(name: &str) -> Result<(), ValidationError> {
    // Pattern: one or more letters followed by one or more digits (eth0, ib1, enp0s3)
    let letter_end = name
        .find(|c: char| !c.is_alphabetic())
        .unwrap_or(name.len());

    if letter_end == 0 {
        return Err(ValidationError::InvalidNetworkInterface {
            name: name.to_string(),
            reason: "interface name must start with letters".to_string(),
        });
    }

    let rest = &name[letter_end..];

    if rest.is_empty() || !rest.chars().next().unwrap().is_ascii_digit() {
        return Err(ValidationError::InvalidNetworkInterface {
            name: name.to_string(),
            reason: "interface name must have digits after the letters (e.g., eth0, ib1)"
                .to_string(),
        });
    }

    Ok(())
}
