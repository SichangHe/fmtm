# ForMaT Markdown

A diff-friendly Markdown formatter that breaks lines on
sensible punctuations and words to fit a line width.

This is more useful to use with
Git than formatters like Prettier because the formatting is more consistent,
resulting in smaller diffs.

FMTM uses [FMTT][fmtt] to format Markdown paragraphs:

- Respect line width limit.
- Prioritize splitting on
    1. sentence ends like `.`, then
    1. sub-sentence ends like `,`, then
    1. sub-sentence starts like `(`, and finally
    1. sentence-connection words like `and`.
- Limited support for abbreviations using heuristics.

FMTM preserves code blocks,
and trims leading spaces to a multiple of 4 for display math and HTML blocks.

## Installation

```sh
cargo install fmtm
```

## Usage

```sh
$ fmtm --help
ForMaT Markdown diff-friendly,
breaking lines on sensible punctuations and words to fit a line width.

Like FMTT,
FMTM formats its input to have lines shorter than the line width limit
(if possible).
It reads an input file or StdIn and prints the formatted text to StdOut.
Like FMTT, FMTM only preserves leading spaces, not tabs.


Usage: fmtm [OPTIONS]

Options:
-w, --line-width <LINE_WIDTH>
          Maximum line width limit. Preserve line lengths if set to 0.

          [default: 80]

  -f, --filename <FILENAME>
          Name of input file; if omitted, read from StdIn.

  -c, --change-in-place
          If input file is provided, write output to it.

  -e, --emphasis-marker <EMPHASIS_MARKER>
          Marker for emphasis spans, or preserve the input if set to "".

          [default: *]

  -s, --strong-marker <STRONG_MARKER>
          Marker for strong spans, or preserve the input if set to "".

          [default: **]

  -i, --indentation <INDENTATION>
          Fixed indentation string,
          or calculate to align the beginning of content text in each list if set to "".

          [default: "    "]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Examples

See [`src/tests/**.md`](/src/tests/)
for test inputs and their formatted output in
[`src/tests/snapshots/`](/src/tests/snapshots/).

This documentation itself is also formatted with FMTM.

## Thanks

A fork of @ytmimi's [`markdown-fmt`](https://github.com/ytmimi/markdown-fmt)
powers this formatter's Markdown handling;
it is a proof of concept for `rustfmt`.
Thank you @ytmimi for all the hard work!

## Future work

This formatter is for myself. If you want extra features, feel free to fork.
Pull Requests are welcome, but I will try to keep high code quality.

Items with 🌹 are the ones I want.

## Unexposed configurations

I have implemented these options in the `markdown-fmt` fork,
but have not exposed them in the CLI.

- [ ] Customize unordered list style.
    Currently, it is fixed to `-`, but can be `*` or preserved.
- [ ] Customize ordered list style.
    Currently, it is fixed to `1.`, but can have leading `0`s or be preserved.

## Near-term extensions

These extensions can be implemented simply by adding `struct`s that
implement traits in my `markdown-fmt` fork.
I lean towards calling external CLI applications (shell out)
for formatting these blocks of other languages.

- [ ] Format code blocks.
- [ ] Format HTML blocks 🌹.
- [ ] Format math expressions 🌹.
- [ ] Customize ordered list numbers to auto-increment. @ytmimi wanted this.

## Markdownlint compliance

These functionalities require further tweaking `markdown-fmt` itself.

- [ ] Only use fenced code blocks 🌹.
- [ ] Add `<` `>` around bare URLs 🌹.
- [ ] One blank line around each heading 🌹.
- [ ] One blank line around each code block 🌹.

Functionalities that alter the content would, in principle, not be implemented.
Examples include fixing reversed links and incrementing header levels.

Currently,
it is perfectly fine to format a Markdown document with both FMTM and
Markdownlint, similar to running isort and black,
except the order does not matter.

[fmtt]: https://github.com/SichangHe/fmtt
