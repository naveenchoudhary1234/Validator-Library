#[derive(Debug, Clone)]
pub enum HostRole {
    Manager,
    Storage,
    Client,
    Gateway,
}

#[derive(Debug, Clone)]
pub struct Host {
    pub hostname: String,
    pub ip_address: String,
    pub role: HostRole,
    pub enabled: bool,
}
