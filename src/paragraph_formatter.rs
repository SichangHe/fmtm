use std::iter::once;

use fmtt::{format, Hanging};

use super::*;

/// A [`ParagraphFormatter`] that uses the FMTT to format Markdown paragraphs.
/// If the line width is not specified, do not format.
pub struct FmttParagraph {
    line_width: Option<usize>,
    buffer: String,
    hard_break_points: Vec<usize>,
}

impl Write for FmttParagraph {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let is_hard_break = matches!(
            s, r"\
"
        );
        self.buffer.push_str(s);
        if is_hard_break {
            self.hard_break_points.push(self.buffer.len());
        }
        Ok(())
    }
}

impl ParagraphFormatter for FmttParagraph {
    fn new(max_width: Option<usize>, capacity: usize) -> Self {
        Self {
            line_width: max_width,
            buffer: String::with_capacity(capacity),
            hard_break_points: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    fn into_buffer(self) -> String {
        let Self {
            line_width,
            buffer,
            hard_break_points,
        } = self;
        let chunk_border_indices = hard_break_points.iter().copied();
        match line_width {
            Some(line_width) => {
                let paragraph_starts = Default::default();
                once(0)
                    .chain(chunk_border_indices.clone())
                    .zip(chunk_border_indices.chain(once(buffer.len())))
                    .map(|(start, end)| {
                        format(
                            &buffer[start..end],
                            line_width,
                            Hanging::Hang,
                            &paragraph_starts,
                        )
                        .join("")
                    })
                    .collect::<String>()
                // format(&self.buffer, line_width, Hanging::Hang, &Default::default()).join("")
            }
            None => buffer,
        }
    }
}
