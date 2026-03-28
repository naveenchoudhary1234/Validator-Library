use crate::types::ByteSize;

#[derive(Debug, Clone)]
pub struct Filesystem {
    pub name: String,
    pub mount_point: String,
    pub size: ByteSize,
}

#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub filesystems: Vec<Filesystem>,
    pub default_size: ByteSize,
}
