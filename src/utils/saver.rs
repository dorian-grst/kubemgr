use std::io;
use std::io::Write;
use std::path::PathBuf;

fn confirm_overwrite(path: &str) -> Result<bool, String> {
    print!(
        "⚠ The file '{}' already exists. Do you want to overwrite it? (Y/n): ",
        path
    );
    io::stdout().flush().ok();

    let mut confirmation = String::new();
    io::stdin()
        .read_line(&mut confirmation)
        .map_err(|_| "✕ Failed to read input")?;

    let confirmation = confirmation.trim().to_lowercase();
    Ok(confirmation.is_empty() || confirmation == "y")
}

pub fn validate_output_path(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = PathBuf::from(path);
    if output_path.exists() && !confirm_overwrite(path)? {
        // TODO : Create a custom error type for this
        return Err("✕ Operation cancelled by the user.".into());
    }
    Ok(())
}