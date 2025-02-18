use crate::utils::errors::KubeMergeError;
use kube::config::Kubeconfig;

#[derive(Debug)]
pub struct KubeconfigContent {
    pub content: String,
}

/// Core merging logic that works with raw YAML content
pub fn merge_kubeconfig_contents(configs: &[KubeconfigContent]) -> Result<String, KubeMergeError> {
    if configs.is_empty() {
        return Err(KubeMergeError::NoContent(
            "No kubeconfig content provided".to_string(),
        ));
    }

    let mut merged_config: Option<Kubeconfig> = None;

    for config in configs {
        let current_config: Kubeconfig = serde_yaml::from_str(&config.content)
            .map_err(|e| KubeMergeError::ParseError(e.to_string()))?;

        match merged_config {
            None => merged_config = Some(current_config),
            Some(ref mut merged) => {
                merged.clusters.extend(current_config.clusters);
                merged.auth_infos.extend(current_config.auth_infos);
                merged.contexts.extend(current_config.contexts);
                if merged.current_context.is_none() {
                    merged.current_context = current_config.current_context;
                }
            }
        }
    }

    let merged = merged_config.ok_or_else(|| {
        KubeMergeError::NoContent("No valid kubeconfig content found".to_string())
    })?;

    serde_yaml::to_string(&merged).map_err(|e| {
        KubeMergeError::ParseError(format!("Failed to serialize merged config: {}", e))
    })
}
