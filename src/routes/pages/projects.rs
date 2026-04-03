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

pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Template)]
#[template(path = "pages/projects.html")]
struct ProjectsTemplate {
    projects: Vec<Project>,
}

impl askama::filters::HtmlSafe for ProjectsTemplate {}

impl ProjectsTemplate {
    fn new(projects: Vec<Project>) -> Self {
        Self { projects }
    }
}

#[route(GET "/projects" with AppState)]
pub async fn projects(
    headers: HeaderMap,
    State(app_name): State<Arc<String>>,
) -> Result<impl IntoResponse, AppError> {
    let html = render_page(
        &headers,
        format!("Projects — {}", app_name),
        Page::Projects,
        ProjectsTemplate::new(vec![
            Project {
                name: "hello-askama",
                description: "A demo web app built with Rust, Axum, Askama and htmx.",
            },
            Project {
                name: "axum-typed-routing",
                description: "Type-safe routing for Axum with compile-time path checking.",
            },
        ]),
    )?;
    Ok(Html(html))
}
