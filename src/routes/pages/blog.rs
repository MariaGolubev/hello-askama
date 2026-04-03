use askama::Template;
use axum::{
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse},
};
use axum_typed_routing::route;
use std::sync::Arc;

use crate::{AppState, error::AppError, ui::components::Page, ui::layout::render::render_page};

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
                title: "Integrating Svelte into an Axum service",
                date: "2026-03-18",
                summary: "How I approached wiring a Svelte frontend to a Rust Axum backend in my svelte-inc-dec project.",
            },
            BlogPost {
                title: "GLSL Mandelbrot experiments in a libadwaita app",
                date: "2026-02-26",
                summary: "Notes from building fractal: rendering Mandelbrot sets in a desktop application with C and shaders.",
            },
            BlogPost {
                title: "Building a minimal shell with finite state machines",
                date: "2026-01-30",
                summary: "Lessons learned while implementing minishell in C and organizing command parsing with an FSM.",
            },
        ]),
    )?;

    Ok(Html(html))
}
