use axum::response::IntoResponse;

pub type Result<T, E> = anyhow::Result<T, E>;

#[derive(Debug, Clone, thiserror::Error)]
pub enum AppError {
    #[error("Config file could not open")]
    ConfigFileCouldNotOpen,

    #[error("Config file could not read: {0}")]
    ConfigFileReadFailed(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
