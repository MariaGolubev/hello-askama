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

pub struct BlogPost {
    pub title: &'static str,
    pub date: &'static str,
    pub summary: &'static str,
}

#[derive(Template)]
#[template(path = "pages/blog.html")]
struct BlogTemplate {
    posts: Vec<BlogPost>,
}

impl BlogTemplate {
    fn new(posts: Vec<BlogPost>) -> Self {
        Self { posts }
    }
}

#[route(GET "/blog" with AppState)]
pub async fn blog(
    headers: HeaderMap,
    State(app_name): State<Arc<String>>,
) -> Result<impl IntoResponse, AppError> {
    let html = render_page(
        &headers,
        format!("Blog — {}", app_name),
        Page::Blog,
        BlogTemplate::new(vec![
            BlogPost {
                title: "Getting started with Askama",
                date: "2026-03-01",
                summary: "A quick introduction to type-safe HTML templates in Rust.",
            },
            BlogPost {
                title: "Boosting navigation with htmx",
                date: "2026-03-10",
                summary: "How hx-boost turns a multi-page app into a SPA-like experience.",
            },
        ]),
    )?;

    Ok(Html(html))
}
