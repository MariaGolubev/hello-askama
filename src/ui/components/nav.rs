use askama::Template;
use strum::IntoEnumIterator;
use strum::{Display, EnumIter};

#[derive(EnumIter, Display, Debug, PartialEq)]
pub enum Page {
    Home,
    Blog,
    Projects,
    Contact,
    About,
    QrCode,
}

impl Page {
    pub fn url(&self) -> &'static str {
        match self {
            Page::Home => "/",
            Page::Blog => "/blog",
            Page::Projects => "/projects",
            Page::Contact => "/contact",
            Page::About => "/about",
            Page::QrCode => "/qrcode",
        }
    }
}

#[derive(Template)]
#[template(path = "partials/nav.html")]
pub struct NavTemplate {
    pub current_path: Page,
}

impl NavTemplate {
    pub fn new(current_path: Page) -> Self {
        Self { current_path }
    }
}

impl askama::filters::HtmlSafe for NavTemplate {}