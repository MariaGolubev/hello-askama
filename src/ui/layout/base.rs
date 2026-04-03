use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate<T>
where
    T: Template + askama::filters::HtmlSafe,
{
    pub title: String,
    pub content: T,
}

impl<T> BaseTemplate<T>
where
    T: Template + askama::filters::HtmlSafe,
{
    pub fn new(title: String, content: T) -> Self {
        Self { title, content }
    }
}
