use fmtt::{format, Hanging};

use super::*;

/// A [`ParagraphFormatter`] that uses the FMTT to format Markdown paragraphs.
/// If the line width is not specified, do not format.
pub struct FmttParagraph {
    line_width: Option<usize>,
    buffer: String,
}

impl Write for FmttParagraph {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.push_str(s);
        Ok(())
    }
}

impl ParagraphFormatter for FmttParagraph {
    fn new(max_width: Option<usize>, capacity: usize) -> Self {
        Self {
            line_width: max_width,
            buffer: String::with_capacity(capacity),
        }
    }

    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    fn into_buffer(self) -> String {
        match self.line_width {
            Some(line_width) => {
                format(&self.buffer, line_width, Hanging::Hang, &Default::default()).join("")
            }
            None => self.buffer,
        }
    }
}
