use askama::Template;
use axum::{
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse},
};

use axum_typed_routing::route;
use std::sync::Arc;

use crate::{
    AppState, error::AppError, ui::components::Page, ui::layout::render::render_page,
};

#[derive(Template)]
#[template(path = "pages/about.html")]
struct AboutTemplate;

impl AboutTemplate {
    fn new() -> Self {
        Self
    }
}

#[route(GET "/about" with AppState)]
pub async fn about(
    headers: HeaderMap,
    State(app_name): State<Arc<String>>,
) -> Result<impl IntoResponse, AppError> {
    let html = render_page(
        &headers,
        format!("About - {}", app_name),
        Page::About,
        AboutTemplate::new(),
    )?;

    Ok(Html(html))
}
