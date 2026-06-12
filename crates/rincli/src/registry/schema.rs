use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RegistryModel {
    pub alias: String,
    pub display_name: String,
    pub filename: String,
    pub source_repo: String,
    pub model_url: String,
    pub download_url: String,
    pub quant: String,
    pub format: String,
    pub size_bytes: u64,
    pub sha256: String,
    pub license: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Registry {
    pub models: Vec<RegistryModel>,
}
