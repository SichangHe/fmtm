use std::fmt::Write;

use markdown_fmt::*;
use tracing::*;

pub mod paragraph_formatter;

pub use crate::paragraph_formatter::FmttParagraph;

pub type FmttExternalFormatter =
    FormatterCombination<PreservingBuffer, TrimTo4Indent, TrimTo4Indent, FmttParagraph>;

/// Format a markdown string with the given line width and
/// default configuration.
pub fn format(input: &str, line_width: Option<usize>) -> Result<String, std::fmt::Error> {
    let config = Config {
        max_width: line_width,
        ..Config::sichanghe_opinion()
    };
    format_with_config(input, config)
}

/// Format a markdown string with the given configuration.
pub fn format_with_config(input: &str, config: Config) -> Result<String, std::fmt::Error> {
    debug!(?config);
    <MarkdownFormatter<FmttExternalFormatter>>::with_config_and_external_formatter(config)
        .format(input)
}

#[cfg(test)]
mod tests;
