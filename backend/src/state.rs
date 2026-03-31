use std::path::PathBuf;

#[derive(Clone)]
pub struct AppState {
    pub frontend_url: String,
    pub log_dir: PathBuf,
    pub plugin_dir: PathBuf,
}
