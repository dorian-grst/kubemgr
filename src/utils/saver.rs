use crate::utils::errors::KubeMergeError;
use std::io;
use std::io::Write;
use std::path::PathBuf;

fn confirm_overwrite(path: &str) -> Result<bool, KubeMergeError> {
    print!(
        "⚠ The file '{}' already exists. Do you want to overwrite it? (Y/n): ",
        path
    );
    io::stdout()
        .flush()
        .map_err(|e| KubeMergeError::WriteError(e.to_string()))?;

    let mut confirmation = String::new();
    io::stdin()
        .read_line(&mut confirmation)
        .map_err(|e| KubeMergeError::WriteError(e.to_string()))?;

    let confirmation = confirmation.trim().to_lowercase();
    Ok(confirmation.is_empty() || confirmation == "y")
}

pub fn validate_output_path(path: &str) -> Result<(), KubeMergeError> {
    let output_path = PathBuf::from(path);
    if output_path.exists() && !confirm_overwrite(path)? {
        return Err(KubeMergeError::UserCancelled(
            "Operation cancelled by user".to_string(),
        ));
    }
    Ok(())
}
