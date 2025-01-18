use once_cell::sync::Lazy;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use image_processor::{
    config::CONFIG,
    error::{AppError, Result},
    updown::startup,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let config = Lazy::force(&CONFIG)
        .as_ref()
        .map_err(|err| AppError::Startup(err.to_string()))?;

    println!("Hello world {:?}", config);

    // set the address
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), config.port);
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|_err| AppError::TcpBind)?;

    let app = startup::create_app()?;

    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| AppError::Startup(e.to_string()))?;

    Ok(())
}
