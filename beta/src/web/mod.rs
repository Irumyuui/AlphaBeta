use axum::{http::StatusCode, response::IntoResponse, Json, Router};
use serde_json::json;

// 每个 mod 都需要一个 get_routers 参数代表需要的中间件

pub fn get_routers() -> Router {
    Router::new().fallback(default_api_failed_handler)
}

async fn default_api_failed_handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "msg": "404 not found",
        })),
    )
}
