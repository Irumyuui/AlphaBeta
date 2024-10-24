use std::{ops::Deref, sync::Arc};

use serde::Deserialize;
use tokio::io::AsyncReadExt;

use super::errors::{AppError, Result};

#[derive(Debug, Clone)]
pub struct AppConfig {
    inner: Arc<AppConfigInner>,
}

impl Deref for AppConfig {
    type Target = AppConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// 从 app.json 或者 app.toml 读取
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfigInner {
    pub port: Option<u16>,
    pub db_url: String,
}

impl AppConfig {
    pub async fn from_toml_file<P: AsRef<std::path::Path>>(path: P) -> Result<AppConfig, AppError> {
        Ok(AppConfig {
            inner: Arc::new(AppConfigInner::from_toml_file(path).await?),
        })
    }
}

impl AppConfigInner {
    async fn from_toml_file<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<AppConfigInner, AppError> {
        let mut file = tokio::fs::File::open(path)
            .await
            .map_err(|_| AppError::ConfigFileCouldNotOpen)?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .await
            .map_err(|e| AppError::ConfigFileReadFailed(e.to_string()))?;
        drop(file);

        AppConfigInner::from_toml(content.as_str()).map(|mut conf| {
            if conf.port.is_none() {
                conf.port = Some(8080);
            }
            conf
        })
    }

    fn from_toml(content: &str) -> Result<AppConfigInner, AppError> {
        toml::from_str::<AppConfigInner>(content)
            .map_err(|e| AppError::ConfigFileReadFailed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::AppConfigInner;

    #[test]
    fn test_read_from_app_toml() {
        let toml_file = r#"
            port = 8080
            db_url = "mongodb://mongodb"
        "#;

        let config_inner = AppConfigInner::from_toml(toml_file).unwrap();
        assert_eq!(config_inner.port, Some(8080));
        assert_eq!(config_inner.db_url, "mongodb://mongodb");
    }
}
