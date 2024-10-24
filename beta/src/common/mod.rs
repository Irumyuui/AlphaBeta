use config::AppConfig;
use errors::{AppError, Result};

pub mod config;
pub mod errors;

// 全局服务
#[derive(Debug, Clone)]
pub struct AppState {
    pub config: AppConfig,
}

pub struct AppStateBuilder {
    config_toml_path: Option<String>,
}

impl Default for AppStateBuilder {
    fn default() -> Self {
        Self {
            config_toml_path: Some("app.toml".to_string()),
        }
    }
}

impl AppStateBuilder {
    pub fn config_toml_path(mut self, path: &str) -> Self {
        self.config_toml_path = Some(path.to_string());
        self
    }

    pub fn build(self) -> Result<AppState, AppError> {
        todo!()
    }
}

impl AppState {}
