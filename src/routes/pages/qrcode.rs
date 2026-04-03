use std::sync::Arc;

use crate::{
    AppState, error::AppError, ui::components::Page, ui::components::qrcode::build_qrcode_svg,
    ui::layout::render::render_page,
};
use askama::Template;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::Html;
use axum::response::IntoResponse;
use axum_typed_routing::route;

#[derive(Template)]
#[template(path = "pages/qrcode.html")]
pub struct QrCodePageTemplate {
    qr_svg: String,
}

impl askama::filters::HtmlSafe for QrCodePageTemplate {}

impl QrCodePageTemplate {
    pub fn new(qr_svg: String) -> Self {
        Self { qr_svg }
    }
}

fn resolve_target_url(headers: &HeaderMap, listen_addr: std::net::SocketAddr) -> String {
    if let Some(host_header) = headers.get("host") {
        if let Ok(host) = host_header.to_str() {
            if !host.is_empty() {
                return format!("http://{host}");
            }
        }
    }

    format!("http://{listen_addr}")
}

#[route(GET "/qrcode" with AppState)]
pub async fn qrcode(
    headers: HeaderMap,
    State(app_name): State<Arc<String>>,
    State(listen_addr): State<std::net::SocketAddr>,
) -> Result<impl IntoResponse, AppError> {
    let target_url = resolve_target_url(&headers, listen_addr);
    let qr_svg = match build_qrcode_svg(&target_url) {
        Ok(svg) => svg,
        Err(err) => {
            tracing::error!("failed to render qrcode svg: {err}");
            "<p class=\"muted\">QR code is temporarily unavailable</p>".to_string()
        }
    };

    let html = render_page(
        &headers,
        format!("QrCode — {}", app_name),
        Page::QrCode,
        QrCodePageTemplate::new(qr_svg),
    )?;
    Ok(Html(html))
}
