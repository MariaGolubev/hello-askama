use std::fmt::Display;

use askama::{FastWritable, Template};
use axum::http::HeaderMap;

use crate::ui::{
    components::nav::{NavTemplate, Page},
    layout::{base::BaseTemplate, stack::Stack},
};

pub fn render_page<T>(
    headers: &HeaderMap,
    title: String,
    current_page: Page,
    content: T,
) -> askama::Result<String>
where
    T: FastWritable + Display,
{
    let stack = Stack::default()
        .add(NavTemplate::new(current_page))
        .add(content);

    if headers.contains_key("hx-request") {
        stack.render()
    } else {
        BaseTemplate::new(title, stack).render()
    }
}