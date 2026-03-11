use std::sync::Arc;

use axum::{
    extract::{FromRef, State},
    response::IntoResponse,
};
use axum_typed_routing::{TypedRouter, route};
use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod config;
mod shutdown;

#[route(GET "/" with AppState)]
async fn index(State(app_name): State<Arc<String>>) -> impl IntoResponse {
    format!("Hello from {app_name}!")
}

#[derive(FromRef, Clone)]
pub struct AppState {
    pub app_name: Arc<String>,
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
            let state = AppState {
                app_name: std::sync::Arc::new("hello-askama".to_string()),
            };

            let app = axum::Router::new()
                .typed_route(index)
                .with_state(state)
                .layer(tower_http::trace::TraceLayer::new_for_http())
                .layer(tower_http::timeout::TimeoutLayer::with_status_code(
                    axum::http::StatusCode::REQUEST_TIMEOUT,
                    config.request_timeout,
                ));

            let listener = tokio::net::TcpListener::bind(config.socket_addr())
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
