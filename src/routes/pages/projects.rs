use askama::Template;
use axum::{
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse},
};
use axum_typed_routing::route;
use std::sync::Arc;

use crate::{AppState, error::AppError, ui::components::Page, ui::layout::render::render_page};

pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub url: &'static str,
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
                name: "webserv",
                description: "HTTP server project focused on networking fundamentals and protocol handling.",
                url: "https://github.com/MariaGolubev/webserv",
            },
            Project {
                name: "svelte-inc-dec",
                description: "Integration of a Svelte interface into a Rust Axum web service.",
                url: "https://github.com/MariaGolubev/svelte-inc-dec",
            },
            Project {
                name: "fractal",
                description: "A libadwaita desktop application that draws Mandelbrot sets in a GLSL shader.",
                url: "https://github.com/MariaGolubev/fractal",
            },
            Project {
                name: "minishell",
                description: "A minimal Unix-like shell in C, focused on core shell behavior and parser design.",
                url: "https://github.com/MariaGolubev/minishell",
            },
            Project {
                name: "hello_codam",
                description: "A GTK4/libadwaita application project in C.",
                url: "https://github.com/MariaGolubev/hello_codam",
            },
            Project {
                name: "Cube3d",
                description: "Implementation of DDA ray tracing algorithm in C.",
                url: "https://github.com/MariaGolubev/Cube3d",
            },
        ]),
    )?;
    Ok(Html(html))
}
