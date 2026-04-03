use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::ui::layout::base::BaseTemplate;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("internal server error")]
    Render(#[from] askama::Error),
    #[error("not found: {path}")]
    NotFound { path: String },
}

#[derive(Template)]
#[template(path = "errors/content.html")]
struct ErrorContentTemplate {
    status: u16,
    message: String,
}

impl askama::filters::HtmlSafe for ErrorContentTemplate {}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();
        let status = match self {
            AppError::Render(err) => {
                tracing::error!("template rendering error: {err}");
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::NotFound { .. } => axum::http::StatusCode::NOT_FOUND,
        };

        let template = BaseTemplate {
            title: format!("Error {}", status.as_u16()),
            content: ErrorContentTemplate {
                status: status.as_u16(),
                message,
            },
        };

        match template.render() {
            Ok(html) => (status, Html(html)).into_response(),
            Err(err) => {
                tracing::error!("error template rendering failed: {err}");
                (status, status.to_string()).into_response()
            }
        }
    }
}
