use config_validator::{
    FilesystemBuilder, HostBuilder, HostRole, NetworkInterfaceBuilder,
    NetworkType, ServerConfigBuilder,
};

fn main() {
    println!("=== Builder Pattern Demo ===\n");

    let config = ServerConfigBuilder::new()
        .name("production-cluster")
        .add_host(
            HostBuilder::new()
                .hostname("manager-01")
                .ip_address("192.168.1.10")
                .role(HostRole::Manager)
                .enabled(true)
                .build()
                .unwrap(),
        )
        .add_host(
            HostBuilder::new()
                .hostname("storage-01")
                .ip_address("192.168.1.11")
                .role(HostRole::Storage)
                .enabled(true)
                .build()
                .unwrap(),
        )
        .add_filesystem(
            FilesystemBuilder::new()
                .name("data_fs")
                .mount_point("/mnt/data")
                .size("100G")
                .unwrap()
                .build()
                .unwrap(),
        )
        .default_size("512M")
        .unwrap()
        .add_interface(
            NetworkInterfaceBuilder::new()
                .name("eth0")
                .network_type(NetworkType::Ethernet)
                .speed_gbps(10)
                .build()
                .unwrap(),
        )
        .build();

    match config {
        Ok(c) => {
            println!("✅ Config built successfully: '{}'", c.name);
            println!("   Hosts: {}", c.hosts.len());
            println!("   Filesystems: {}", c.storage.filesystems.len());
            println!("   Interfaces: {}", c.network.interfaces.len());

            // JSON bhi dikhao
            println!("\n=== JSON Output ===");
            let json = serde_json::to_string_pretty(&c).unwrap();
            println!("{}", json);
        }
        Err(e) => eprintln!("❌ Build failed:\n{}", e),
    }

    println!("\n=== Builder with Invalid Data ===");
    let bad = HostBuilder::new()
        .hostname("-bad-host")
        .ip_address("999.x.x.x")
        .role(HostRole::Storage)
        .build();

    match bad {
        Ok(_) => println!("✅ Host built"),
        Err(e) => eprintln!("❌ Host build failed: {}", e),
    }
}