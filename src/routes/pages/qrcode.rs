use std::sync::Arc;

use crate::{
    AppState, error::AppError, ui::components::Page, ui::components::QrCodeTemplate,
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
pub struct QrCodeTemplate_ {}

impl askama::filters::HtmlSafe for QrCodeTemplate_ {}

impl QrCodeTemplate_ {
    pub fn new() -> Self {
        Self {}
    }
}

#[route(GET "/qrcode" with AppState)]
pub async fn qrcode(
    headers: HeaderMap,
    State(app_name): State<Arc<String>>,
) -> Result<impl IntoResponse, AppError> {
    let html = render_page(
        &headers,
        format!("QrCode — {}", app_name),
        Page::QrCode,
        QrCodeTemplate_::new(),
    )?;
    Ok(Html(html))
}
