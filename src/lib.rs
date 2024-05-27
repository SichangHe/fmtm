use markdown_fmt::*;
use tracing::*;

pub fn format(input: &str, line_width: Option<usize>) -> Result<String, std::fmt::Error> {
    let config = Config {
        max_width: line_width,
        ..Config::sichanghe_opinion()
    };
    debug!(?config);
    FormatterBuilder::with_config(config)
        .build()
        .format_with_paragraph_and_html_block_formatter::<Paragraph, PreservingHtmlBlock>(input)
}

#[cfg(test)]
mod tests;
