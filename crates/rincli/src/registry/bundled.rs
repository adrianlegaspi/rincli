use std::fs;
use std::path::PathBuf;
use crate::errors::RincliError;
use crate::registry::schema::Registry;

#[allow(dead_code)]
pub fn load_registry() -> Result<Registry, RincliError> {
    let paths_to_try = vec![
        PathBuf::from("registry/registry.json"),
        PathBuf::from("../registry/registry.json"),
        PathBuf::from("../../registry/registry.json"),
        PathBuf::from("d:\\Documents\\Github\\rincli\\registry\\registry.json"),
    ];

    for path in paths_to_try {
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let registry: Registry = serde_json::from_str(&content)?;
            return Ok(registry);
        }
    }

    Err(RincliError::RegistryNotFound)
}
