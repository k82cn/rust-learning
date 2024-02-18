use std::collections::HashMap;

use serde::Serialize;
use serde_derive::Serialize;

#[derive(Serialize)]
struct SiteConfig {
    pub ib_fabrics: IBFabricConfig,
}

#[derive(Serialize)]
struct IBFabric {
    pub name: String,
}

#[derive(Serialize)]
struct IBFabricConfig {
    pub max_partition_per_tenant: i32,
    pub enabled: bool,
    pub fabrics: Vec<IBFabric>,
}

fn main() {
    let ib_fabrics = IBFabricConfig {
        max_partition_per_tenant: 3,
        enabled: true,
        fabrics: vec![
            IBFabric {
                name: "default".to_string(),
            },
            IBFabric {
                name: "storage".to_string(),
            },
            IBFabric {
                name: "compute".to_string(),
            },
        ],
    };
    let toml = toml::to_string(&SiteConfig { ib_fabrics }).unwrap();
    println!("{}", toml);
}
