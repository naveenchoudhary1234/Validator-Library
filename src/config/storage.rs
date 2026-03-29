use serde::{Deserialize, Serialize};
use crate::types::ByteSize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filesystem {
    pub name: String,
    pub mount_point: String,
    pub size: ByteSize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub filesystems: Vec<Filesystem>,
    pub default_size: ByteSize,
}