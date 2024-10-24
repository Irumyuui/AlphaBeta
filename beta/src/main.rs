use beta::{log_init, web};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log_init();

    // let app_state = AppState {};

    let app = web::get_routers();

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Server staring on addr: {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
