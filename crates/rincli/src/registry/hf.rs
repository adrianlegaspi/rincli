use serde::Deserialize;
use crate::errors::RincliError;
use crate::registry::schema::RegistryModel;

#[derive(Deserialize, Debug)]
struct HfRepo {
    id: String,
    tags: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct HfFile {
    path: String,
    size: u64,
    lfs: Option<HfLfs>,
}

#[derive(Deserialize, Debug)]
struct HfLfs {
    oid: String,
    size: u64,
}

pub fn search_hf(query: &str) -> Result<Vec<RegistryModel>, RincliError> {
    let search_url = format!(
        "https://huggingface.co/api/models?search={}&filter=gguf&limit=5",
        query
    );
    
    let agent = ureq::builder().build();
    let response: Vec<HfRepo> = agent.get(&search_url)
        .set("User-Agent", "rincli/0.1.0")
        .call()?
        .into_json()?;

    let mut models = Vec::new();

    for repo in response {
        let tree_url = format!("https://huggingface.co/api/models/{}/tree/main", repo.id);
        let files_res: Result<Vec<HfFile>, _> = agent.get(&tree_url)
            .set("User-Agent", "rincli/0.1.0")
            .call()
            .map_err(|e| RincliError::Http(e.to_string()))
            .and_then(|r| r.into_json().map_err(|e| RincliError::Io(e)));

        let files = match files_res {
            Ok(f) => f,
            Err(_) => continue,
        };

        // Find Q4_K_M or other Q4 file
        let mut target_file = None;
        for f in &files {
            let path_lower = f.path.to_lowercase();
            if path_lower.ends_with(".gguf") && path_lower.contains("q4_k_m") {
                target_file = Some(f);
                break;
            }
        }

        if target_file.is_none() {
            for f in &files {
                let path_lower = f.path.to_lowercase();
                if path_lower.ends_with(".gguf") && path_lower.contains("q4") {
                    target_file = Some(f);
                    break;
                }
            }
        }

        if target_file.is_none() {
            for f in &files {
                let path_lower = f.path.to_lowercase();
                if path_lower.ends_with(".gguf") {
                    target_file = Some(f);
                    break;
                }
            }
        }

        if let Some(f) = target_file {
            let parts: Vec<&str> = repo.id.split('/').collect();
            let repo_name = parts.get(1).copied().unwrap_or(&repo.id);
            let display_name = repo_name
                .replace("-GGUF", "")
                .replace("-gguf", "")
                .replace("-", " ")
                .replace("_", " ");
            let alias = repo_name
                .replace("-GGUF", "")
                .replace("-gguf", "")
                .to_lowercase();

            // Extract license from tags
            let mut license = "Unknown".to_string();
            if let Some(ref tags) = repo.tags {
                for tag in tags {
                    if tag.starts_with("license:") {
                        license = tag.replace("license:", "").to_uppercase();
                        break;
                    }
                }
            }

            let size = f.lfs.as_ref().map(|l| l.size).unwrap_or(f.size);
            let sha256 = f.lfs.as_ref().map(|l| l.oid.clone()).unwrap_or_default();

            // Determine quant from filename
            let path_lower = f.path.to_lowercase();
            let quant = if path_lower.contains("q4_k_m") {
                "Q4_K_M".to_string()
            } else if path_lower.contains("q4_0") {
                "Q4_0".to_string()
            } else if path_lower.contains("q4_1") {
                "Q4_1".to_string()
            } else if path_lower.contains("q5_k_m") {
                "Q5_K_M".to_string()
            } else if path_lower.contains("q8_0") {
                "Q8_0".to_string()
            } else {
                "Unknown".to_string()
            };

            models.push(RegistryModel {
                alias,
                display_name,
                filename: f.path.clone(),
                source_repo: repo.id.clone(),
                model_url: format!("https://huggingface.co/{}", repo.id),
                download_url: format!("https://huggingface.co/{}/resolve/main/{}", repo.id, f.path),
                quant,
                format: "GGUF".to_string(),
                size_bytes: size,
                sha256,
                license,
            });
        }
    }

    Ok(models)
}
