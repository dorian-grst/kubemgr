#[derive(Debug)]
pub struct MergeOptions {
    pub files: Vec<String>,
    pub include_current: bool,
    pub dry_run: bool,
    pub output_path: Option<String>,
}
