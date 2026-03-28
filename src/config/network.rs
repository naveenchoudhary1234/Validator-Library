#[derive(Debug, Clone)]
pub enum NetworkType {
    Ethernet,
    Infiniband,
    OmniPath,
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub network_type: NetworkType,
    pub speed_gbps: u32,
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub interfaces: Vec<NetworkInterface>,
}
