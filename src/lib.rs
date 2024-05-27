use std::fmt::Write;

use markdown_fmt::*;
use tracing::*;

pub mod paragraph_formatter;

pub use crate::paragraph_formatter::FmttParagraph;

/// Format a markdown string with the given line width and
/// default configuration.
pub fn format(input: &str, line_width: Option<usize>) -> Result<String, std::fmt::Error> {
    let config = Config {
        max_width: line_width,
        ..Config::sichanghe_opinion()
    };
    debug!(?config);
    FormatterBuilder::with_config(config)
        .build()
        .format_with_paragraph_and_html_block_formatter::<FmttParagraph, PreservingHtmlBlock>(input)
}

/// Format a markdown string with the given configuration.
pub fn format_with_config(input: &str, config: Config) -> Result<String, std::fmt::Error> {
    debug!(?config);
    FormatterBuilder::with_config(config)
        .build()
        .format_with_paragraph_and_html_block_formatter::<FmttParagraph, PreservingHtmlBlock>(input)
}

#[cfg(test)]
mod tests;
