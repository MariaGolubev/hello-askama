use std::sync::Arc;

use axum::{extract::FromRef, response::IntoResponse};
use axum_embed::ServeEmbed;
use axum_typed_routing::TypedRouter;
use clap::Parser;
use rust_embed::RustEmbed;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod config;
mod error;
mod routes;
mod shutdown;
mod ui;

use error::AppError;

#[derive(RustEmbed, Clone)]
#[folder = "static/"]
struct StaticAssets;

async fn fallback(uri: axum::http::Uri) -> impl IntoResponse {
    AppError::NotFound {
        path: uri.path().to_string(),
    }
}

#[derive(FromRef, Clone)]
pub struct AppState {
    pub app_name: Arc<String>,
    pub listen_addr: std::net::SocketAddr,
}

fn main() -> anyhow::Result<()> {
    let config = config::Config::parse();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new(&config.log_filter))
        .init();

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.worker_threads)
        .enable_all()
        .build()?
        .block_on(async {
            let listen_addr = config.socket_addr();
            let state = AppState {
                app_name: std::sync::Arc::new("hello-askama".to_string()),
                listen_addr,
            };

            let app = axum::Router::new()
                .typed_route(routes::pages::index)
                .typed_route(routes::pages::blog)
                .typed_route(routes::pages::projects)
                .typed_route(routes::pages::contact)
                .typed_route(routes::pages::about)
                .typed_route(routes::pages::qrcode)
                .nest_service("/static", ServeEmbed::<StaticAssets>::new())
                .with_state(state)
                .fallback(fallback)
                .layer(tower_http::trace::TraceLayer::new_for_http())
                .layer(tower_http::timeout::TimeoutLayer::with_status_code(
                    axum::http::StatusCode::REQUEST_TIMEOUT,
                    config.request_timeout,
                ));
            // .layer(tower_http::compression::CompressionLayer::new());

            let listener = tokio::net::TcpListener::bind(listen_addr)
                .await
                .and_then(|listener| {
                    tracing::info!("listening on http://{}", listener.local_addr()?);
                    Ok(listener)
                })?;

            axum::serve(listener, app)
                .with_graceful_shutdown(shutdown::graceful_shutdown())
                .await
        })
        .map_err(Into::into)
}
