use super::*;

fn default_format(input: &str) -> Result<String, std::fmt::Error> {
    format(input, Some(80))
}

fn format_20(input: &str) -> Result<String, std::fmt::Error> {
    format(input, Some(20))
}

fn format_inf(input: &str) -> Result<String, std::fmt::Error> {
    format(input, None)
}

macro_rules! t {
    ($name:ident, $input:expr) => {
        #[test]
        fn $name() {
            init_tracing();
            let input = $input.trim_start();
            let default_formatted = default_format(input).unwrap();
            assert_snapshot!(default_formatted);
            let formatted_20 = format_20(input).unwrap();
            assert_snapshot!(formatted_20);
            let formatted_inf = format_inf(input).unwrap();
            assert_snapshot!(formatted_inf);
        }
    };
}

t!(gpt_inline, include_str!("gpt_inline.md"));

t!(gpt1, include_str!("gpt1.md"));

t!(
    hard_breaks,
    r"
Markdown documents may contain hard breaks.\
For example, the one above.

\
Best,\
You Know Who
"
);

t!(
    pulldown_cmark_specs_math,
    include_str!("pulldown_cmark_specs_math.md")
);

t!(
    list_item_start_on_same_line,
    r"
- An unordered list item should not go beyond the line width by its start marker.

1. An ordered list item should not have the first line broken by its start marker.

- [ ] Nor should an empty task list item have the first line broken by its start marker.
- [x] Nor should a checked task list item have the first line broken by its start marker.
"
);

t!(
    yaml_header_then_comment,
    r"---
title: dummy
---

<!-- comment -->
"
);
