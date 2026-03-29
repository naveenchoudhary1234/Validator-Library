use std::fmt;

#[derive(Debug)]
pub enum ValidationError {
    InvalidHostname { hostname: String, reason: String },
    InvalidIpAddress { ip: String, reason: String },
    InvalidFilesystemName { name: String, reason: String },
    InvalidMountPoint { path: String, reason: String },
    InvalidNetworkInterface { name: String, reason: String },
    InvalidByteSize { input: String, reason: String },
    NoManagerHost,
    NoFilesystems,
    EmptyConfiguration { field: String },
    MultipleErrors(Vec<ValidationError>),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidHostname { hostname, reason } => {
                write!(f, "Invalid hostname '{}': {}", hostname, reason)
            }
            ValidationError::InvalidIpAddress { ip, reason } => {
                write!(f, "Invalid IP address '{}': {}", ip, reason)
            }
            ValidationError::InvalidFilesystemName { name, reason } => {
                write!(f, "Invalid filesystem name '{}': {}", name, reason)
            }
            ValidationError::InvalidMountPoint { path, reason } => {
                write!(f, "Invalid mount point '{}': {}", path, reason)
            }
            ValidationError::InvalidNetworkInterface { name, reason } => {
                write!(f, "Invalid network interface '{}': {}", name, reason)
            }
            ValidationError::InvalidByteSize { input, reason } => {
                write!(f, "Invalid byte size '{}': {}", input, reason)
            }
            ValidationError::NoManagerHost => {
                write!(
                    f,
                    "Configuration must have at least one host with Manager role"
                )
            }
            ValidationError::NoFilesystems => {
                write!(f, "Storage configuration must have at least one filesystem")
            }
            ValidationError::EmptyConfiguration { field } => {
                write!(f, "Configuration field '{}' cannot be empty", field)
            }
            ValidationError::MultipleErrors(errors) => {
                let messages: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
                write!(
                    f,
                    "Multiple validation errors:\n  - {}",
                    messages.join("\n  - ")
                )
            }
        }
    }
}

impl std::error::Error for ValidationError {}
