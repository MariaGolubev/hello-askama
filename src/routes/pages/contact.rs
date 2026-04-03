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
#[template(path = "pages/contact.html")]
struct ContactTemplate;

impl ContactTemplate {
    fn new() -> Self {
        Self
    }
}

#[route(GET "/contact" with AppState)]
pub async fn contact(
    headers: HeaderMap,
    State(app_name): State<Arc<String>>,
) -> Result<impl IntoResponse, AppError> {
    let html = render_page(
        &headers,
        format!("Contact — {}", app_name),
        Page::Contact,
        ContactTemplate::new(),
    )?;

    Ok(Html(html))
}
