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
