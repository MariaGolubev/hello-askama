use std::net::{IpAddr, Ipv4Addr};
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

fn host_for_url(ip: IpAddr) -> String {
    match ip {
        IpAddr::V4(ipv4) => ipv4.to_string(),
        IpAddr::V6(ipv6) => format!("[{ipv6}]"),
    }
}

fn resolve_target_url(listen_addr: std::net::SocketAddr) -> String {
    let port = listen_addr.port();

    match local_ip_address::local_ip() {
        Ok(ip) => format!("http://{}:{}", host_for_url(ip), port),
        Err(err) => {
            tracing::warn!("failed to resolve local network ip for qrcode: {err}");
            let fallback_ip = if listen_addr.ip().is_unspecified() {
                IpAddr::V4(Ipv4Addr::LOCALHOST)
            } else {
                listen_addr.ip()
            };
            format!("http://{}:{}", host_for_url(fallback_ip), port)
        }
    }
}

#[route(GET "/qrcode" with AppState)]
pub async fn qrcode(
    headers: HeaderMap,
    State(app_name): State<Arc<String>>,
    State(listen_addr): State<std::net::SocketAddr>,
) -> Result<impl IntoResponse, AppError> {
    let target_url = resolve_target_url(listen_addr);
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
