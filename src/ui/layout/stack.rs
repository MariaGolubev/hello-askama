use std::fmt;

use askama::{FastWritable, Template, Values, filters::HtmlSafe};

pub struct StackEnd {
    tag: &'static str,
    class: Option<&'static [&'static str]>,
}

impl StackEnd {
    pub fn new(tag: &'static str) -> Self {
        Self { tag, class: None }
    }
}

impl fmt::Display for StackEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag)
    }
}

impl FastWritable for StackEnd {
    fn write_into<W: fmt::Write + ?Sized>(
        &self,
        dest: &mut W,
        _values: &dyn Values,
    ) -> askama::Result<()> {
        if !self.tag.is_empty() {
            dest.write_str("<")?;
            dest.write_str(self.tag)?;

            if let Some(classes) = self.class {
                dest.write_str(" class=\"")?;
                for (i, class) in classes.iter().enumerate() {
                    if i > 0 {
                        dest.write_str(" ")?;
                    }
                    dest.write_str(class)?;
                }
                dest.write_str("\"")?;
            }
            dest.write_str(">")?;
        }
        Ok(())
    }
}

pub struct StackLayer<T, S> {
    content: T,
    next: S,
}

impl<T, S> StackLayer<T, S> {
    pub fn new(content: T, next: S) -> Self {
        Self { content, next }
    }
}

impl<T, S> fmt::Display for StackLayer<T, S>
where
    T: fmt::Display,
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.next, self.content)
    }
}

impl<T, S> FastWritable for StackLayer<T, S>
where
    T: FastWritable,
    S: FastWritable,
{
    fn write_into<W: fmt::Write + ?Sized>(
        &self,
        dest: &mut W,
        values: &dyn Values,
    ) -> askama::Result<()> {
        self.next.write_into(dest, values)?;
        self.content.write_into(dest, values)
    }
}

impl<T, S> HtmlSafe for StackLayer<T, S>
where
    T: fmt::Display,
    S: fmt::Display,
{
}

pub struct Stack<T> {
    tag: &'static str,
    next: T,
}

impl Stack<StackEnd> {
    pub fn new(tag: &'static str) -> Self {
        Self {
            tag,
            next: StackEnd::new(tag),
        }
    }
}

impl Default for Stack<StackEnd> {
    fn default() -> Self {
        Self::new("")
    }
}

impl<T> Stack<T> {
    pub fn add<U>(self, content: U) -> Stack<StackLayer<U, T>> {
        Stack {
            tag: self.tag,
            next: StackLayer::new(content, self.next),
        }
    }
}

impl<T> fmt::Display for Stack<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.tag, self.next)
    }
}

impl<T> FastWritable for Stack<T>
where
    T: FastWritable,
{
    fn write_into<W: fmt::Write + ?Sized>(
        &self,
        dest: &mut W,
        values: &dyn Values,
    ) -> askama::Result<()> {
        dest.write_str(self.tag)?;
        self.next.write_into(dest, values)
    }
}

impl<T> HtmlSafe for Stack<T> where T: fmt::Display {}

impl<T> Template for Stack<T>
where
    T: FastWritable + fmt::Display,
{
    const SIZE_HINT: usize = 0;

    fn render_into_with_values<W: core::fmt::Write + ?Sized>(
        &self,
        writer: &mut W,
        values: &dyn Values,
    ) -> askama::Result<()> {
        self.next.write_into(writer, values)?;
        if !self.tag.is_empty() {
            writer.write_str("<")?;
            writer.write_str(self.tag)?;
            writer.write_str(">")?;
        }
        Ok(())
    }
}
