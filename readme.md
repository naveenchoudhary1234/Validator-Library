# Config Validator

A fast, robust configuration validation library for server management systems written in Rust.

## Features

- Parse and validate server configurations with hosts, storage, and network settings
- Custom `ByteSize` type for human-readable sizes (`"100G"`, `"512M"`, `"1T"`)
- Collect **all validation errors at once** — never stop at the first error
- Friendly, descriptive error messages for every failure
- Strict validation rules — invalid hostnames, bad IPs, missing roles all caught
- Minimal dependencies (only `thiserror` for ergonomic errors)

## Building

```bash
cargo build --release
```

## Running Tests

```bash
cargo test
```

Run with detailed output:

```bash
cargo test -- --nocapture
```

Run a specific test:

```bash
cargo test test_valid_config
```

Run only ByteSize tests:

```bash
cargo test byte_size
```

## Usage

```bash
# Run the demo example
cargo run --example demo

# Check for lint warnings
cargo clippy

# Format code
cargo fmt
```

## Example Usage

```rust
use config_validator::{
    Filesystem, Host, HostRole, NetworkConfig, NetworkInterface,
    NetworkType, ServerConfig, StorageConfig, Validate,
};

fn main() {
    let config = ServerConfig {
        name: "production-cluster".to_string(),
        hosts: vec![
            Host {
                hostname: "manager-01".to_string(),
                ip_address: "192.168.1.10".to_string(),
                role: HostRole::Manager,
                enabled: true,
            },
        ],
        storage: StorageConfig {
            filesystems: vec![Filesystem {
                name: "data_fs".to_string(),
                mount_point: "/mnt/data".to_string(),
                size: "100G".parse().unwrap(),
            }],
            default_size: "512M".parse().unwrap(),
        },
        network: NetworkConfig {
            interfaces: vec![NetworkInterface {
                name: "eth0".to_string(),
                network_type: NetworkType::Ethernet,
                speed_gbps: 10,
            }],
        },
    };

    match config.validate() {
        Ok(()) => println!("Configuration is valid!"),
        Err(e) => eprintln!("Validation failed:\n{}", e),
    }
}
```

## Sample Output

### Valid Configuration

```
=== Testing Valid Configuration ===
✅ Configuration is valid!
```

### Invalid Configuration

```
=== Testing Invalid Configuration ===
❌ Validation failed:
Multiple validation errors:
  - Invalid hostname '-invalid-host': cannot start or end with a hyphen
  - Invalid IP address '999.999.999.999': '999' is not a valid octet (0-255)
  - Configuration must have at least one host with Manager role
  - Invalid filesystem name 'bad-name!': filesystem name can only contain alphanumeric characters and underscores
  - Invalid mount point 'no_slash': mount point must start with '/'
```

## Project Structure

```
config-validator/
├── Cargo.toml
├── README.md
├── examples/
│   └── demo.rs
└── src/
    ├── lib.rs              — public API, re-exports
    ├── error.rs            — ValidationError enum with display messages
    ├── config/
    │   ├── mod.rs          — config module exports
    │   ├── server.rs       — ServerConfig struct
    │   ├── host.rs         — Host struct + HostRole enum
    │   ├── storage.rs      — StorageConfig + Filesystem structs
    │   └── network.rs      — NetworkConfig + NetworkInterface + NetworkType
    ├── types/
    │   ├── mod.rs          — types module exports
    │   └── byte_size.rs    — ByteSize (parse, display, ordering)
    └── validation/
        ├── mod.rs          — Validate impl for all config types
        ├── traits.rs       — Validate trait definition
        └── rules.rs        — validation helper functions
```

## Validation Rules

| Field | Rule |
|-------|------|
| `Host.hostname` | Alphanumeric + hyphens only, 1–253 chars, cannot start or end with hyphen |
| `Host.ip_address` | Valid IPv4 format — 4 octets each 0–255 (e.g. `192.168.1.1`) |
| `Filesystem.name` | Alphanumeric + underscores only, 1–64 chars |
| `Filesystem.mount_point` | Must start with `/` |
| `NetworkInterface.name` | Letters followed by digits (e.g. `eth0`, `ib1`, `enp0s3`) |
| `ServerConfig` | Must have at least one host with `Manager` role |
| `StorageConfig` | Must have at least one filesystem |

## ByteSize Format

Human-readable sizes are parsed and stored internally as bytes (`u64`).

**Supported suffixes:**

| Suffix | Multiplier | Example |
|--------|------------|---------|
| `B` | 1 | `2048B` |
| `K` | 1,024 | `1024K` |
| `M` | 1,048,576 | `512M` |
| `G` | 1,073,741,824 | `100G` |
| `T` | 1,099,511,627,776 | `1T` |

**Parsing rules:**
- Case-insensitive: `"100g"` = `"100G"`
- Size must be non-zero: `"0G"` is invalid
- Invalid formats rejected: `""`, `"abc"`, `"-100G"`, `"100X"`, `"G100"`

## Error Types

| Error | When it occurs |
|-------|---------------|
| `InvalidHostname` | Invalid characters, length, or hyphen placement |
| `InvalidIpAddress` | Not a valid IPv4 address |
| `InvalidFilesystemName` | Invalid characters or out of length range |
| `InvalidMountPoint` | Does not start with `/` |
| `InvalidNetworkInterface` | Does not match `letters+digits` pattern |
| `InvalidByteSize` | Unknown unit, missing value, or zero size |
| `NoManagerHost` | No host has the `Manager` role |
| `NoFilesystems` | Storage config has zero filesystems |
| `EmptyConfiguration` | Required field like `name` or `hosts` is empty |
| `MultipleErrors` | Multiple errors collected and reported together |

## Dependencies

```toml
[dependencies]
thiserror = "1.0"
```

- **thiserror** — ergonomic error type derivation

---

