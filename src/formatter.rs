use anyhow::Result;
use regex::Regex;

use crate::reserved_words::{
    RESERVED_WORDS,
    ONLY_BEFORE_LINE_FEED_WORDS,
    ONLY_AFTER_LINE_FEED_WORDS,
    NONE_LINE_FEED_WORDS
};

const BLUE: u32 = 36;
const YELLOW: u32 = 33;

pub struct Options {
    pub lower: bool,
}

fn coloring(target: String, color_num: u32) -> String {
    let form = format!("\x1b[{}m{}\x1b[0m", color_num, target);
    form
}

fn line_formatting(target: String, w: &&str) -> String {
    let form = if ONLY_BEFORE_LINE_FEED_WORDS.contains(w) {
        format!("\n{}", target)
    } else if ONLY_AFTER_LINE_FEED_WORDS.contains(w) {
        format!("{}\n", target)
    } else if NONE_LINE_FEED_WORDS.contains(w) {
        target
    } else {
        format!("\n{}\n", target)
    };
    form
}

fn formatting_equal(input: String) -> Result<String> {
    let regex = Regex::new(r"\s*=\s*")?;
    let output = regex.replace_all(&input, " = ").to_string();
    Ok(output)
}

fn formatting_brackets(input: String) -> Result<String> {
    let space_regex = Regex::new(r"\s*\(")?;
    let trimed = space_regex.replace_all(&input, "(").to_string();

    let bracket_regex = Regex::new(r"(\(|\))")?;
    let colored = bracket_regex.replace_all(&trimed, coloring("$1".to_string(), YELLOW)).to_string();
    Ok(colored)
}

#[allow(unused_assignments)]
pub fn formatting(input: String, options: Options) -> Result<String> {
    let mut sql = input;

    println!("{}", sql);

    for w in RESERVED_WORDS.iter() {
        let from = format!(r"(?i)((\s+)|(?P<head>^)){}((\s+)|(?P<in>\())", w);
        let to = if options.lower {
            format!(" {} ", line_formatting(coloring(w.to_lowercase(), BLUE), w))
        } else {
            format!(" {} ", line_formatting(coloring(w.to_uppercase(), BLUE), w))
        };
        let regex = Regex::new(&from)?;
        sql = regex.replace_all(&sql, format!("{}{}", to, "$in")).to_string();
    }
    sql = formatting_equal(sql)?;
    sql = formatting_brackets(sql)?;
    Ok(sql)
}
