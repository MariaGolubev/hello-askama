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
#[template(path = "pages/index.html")]
struct IndexTemplate {
    message: &'static str,
}

impl IndexTemplate {
    fn new(message: &'static str) -> Self {
        Self { message }
    }
}

#[route(GET "/" with AppState)]
pub async fn index(
    headers: HeaderMap,
    State(app_name): State<Arc<String>>,
) -> Result<impl IntoResponse, AppError> {
    let html = render_page(
        &headers,
        app_name.to_string(),
        Page::Home,
        IndexTemplate::new("Hello, Askama!"),
    )?;

    Ok(Html(html))
}
