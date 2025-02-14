use anyhow::{Context, Result};
use kube::config::Kubeconfig;

#[derive(Debug)]
pub struct KubeconfigContent {
    pub content: String,
}

/// Core merging logic that works with raw YAML content
pub fn merge_kubeconfig_contents(configs: &[KubeconfigContent]) -> Result<String> {
    if configs.is_empty() {
        return Err(anyhow::anyhow!("No kubeconfig content provided"));
    }

    let mut merged_config : Option<Kubeconfig> = None;

    for config in configs {
        let current_config: Kubeconfig =
            serde_yaml::from_str(&config.content).context("Failed to parse kubeconfig content")?;

        match merged_config {
            None => merged_config = Some(current_config),
            Some(ref mut merged) => {
                // Merge clusters
                merged.clusters.extend(current_config.clusters);
                // Merge auth-infos (users)
                merged.auth_infos.extend(current_config.auth_infos);
                // Merge contexts
                merged.contexts.extend(current_config.contexts);
                // Keep the current context from the first file if not explicitly set
                if merged.current_context.is_none() {
                    merged.current_context = current_config.current_context;
                }
            }
        }
    }

    let merged =
        merged_config.ok_or_else(|| anyhow::anyhow!("No valid kubeconfig content found"))?;
    serde_yaml::to_string(&merged).context("Failed to serialize merged config")
}
