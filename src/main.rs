use std::{fs::File, io::*, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use fmtm::*;
use markdown_fmt::Config;
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = App::parse();
    let input = if let Some(filename) = &app.filename {
        read_all(File::open(filename).with_context(|| format!("Cannot open {filename:?}"))?)
            .with_context(|| format!("Cannot read {filename:?}"))?
    } else {
        read_all(stdin()).context("Cannot read StdIn")?
    };

    let max_width = match app.line_width {
        0 => None,
        w => Some(w),
    };
    let fixed_indentation = match app.indentation {
        m if m.is_empty() => None,
        m => Some(m.into()),
    };
    let config = Config {
        max_width,
        fixed_emphasis_marker: set_none_or_leak(app.emphasis_marker),
        fixed_strong_marker: set_none_or_leak(app.strong_marker),
        fixed_indentation,
        ..Config::sichanghe_opinion()
    };
    let formatted = format_with_config(&input, config).context("Cannot format")?;

    if let (true, Some(filename)) = (app.change_in_place, &app.filename) {
        File::create(filename)
            .with_context(|| format!("Cannot create {filename:?}"))?
            .write_all(formatted.as_bytes())
            .with_context(|| format!("Cannot write to {filename:?}"))?;
    } else {
        stdout()
            .write_all(formatted.as_bytes())
            .context("Cannot write to StdOut")?;
    }

    Ok(())
}

fn read_all(from: impl Read) -> Result<String> {
    let mut input = Vec::with_capacity(4096);
    let mut from = BufReader::new(from);
    from.read_to_end(&mut input)?;
    Ok(String::from_utf8(input)?)
}

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = r#"
ForMaT Markdown diff-friendly,
breaking lines on sensible punctuations and words to fit a line width.

Like FMTT,
FMTM formats its input to have lines shorter than the line width limit
(if possible).
It reads an input file or StdIn and prints the formatted text to StdOut.
Like FMTT, FMTT only preserves leading spaces, not tabs.
"#
)]
struct App {
    #[arg(
        short = 'w',
        long,
        default_value = "80",
        help = "Maximum line width limit. Preserve line lengths if set to 0."
    )]
    line_width: usize,

    #[arg(short, long, help = "Name of input file; if omitted, read from StdIn.")]
    filename: Option<PathBuf>,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "If input file is provided, write output to it."
    )]
    change_in_place: bool,

    #[arg(
        short,
        long,
        default_value = "*",
        help = r#"Marker for emphasis spans, or preserve the input if set to ""."#
    )]
    emphasis_marker: String,

    #[arg(
        short,
        long,
        default_value = "**",
        help = r#"Marker for strong spans, or preserve the input if set to ""."#
    )]
    strong_marker: String,

    #[arg(
        short,
        long,
        default_value = "    ",
        help = r#"Fixed indentation string, or preserve the input if set to ""."#
    )]
    indentation: String,
}

fn set_none_or_leak(s: String) -> Option<&'static str> {
    match s.is_empty() {
        true => None,
        false => Some(&*s.leak()),
    }
}
