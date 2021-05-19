// use std::char::ParseCharError;
// use std::str::FromStr;

use anyhow::Result;
use regex::Regex;
use structopt::clap::arg_enum;

use crate::reserved_words::{
    RESERVED_WORDS,
    ONLY_BEFORE_LINE_FEED_WORDS,
    ONLY_AFTER_LINE_FEED_WORDS,
    NONE_LINE_FEED_WORDS
};

arg_enum! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Colors {
        Red = 31,
        Green = 32,
        Yellow = 33,
        Blue = 34,
        Cyan = 36,
        White
    }
}

// impl FromStr for Colors {
//     type Err = ParseCharError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let color = match s {
//             "red" => Colors::Red,
//             "green" => Colors::Green,
//             "yellow" => Colors::Yellow,
//             "blue" => Colors::Blue,
//             "cyan" => Colors::Cyan,
//             "white" => Colors::White,
//             _ => panic!("Invalid arguments of color"),
//         };
//         Ok(color)
//     }
// }

pub struct Options {
    pub lower: bool,
    pub oneline: bool,
    pub color: Option<Colors>,
}

fn coloring(target: String, color: Colors) -> String {
    if color == Colors::White {
        return target
    }
    let form = format!("\x1b[{}m{}\x1b[0m", color as u32, target);
    form
}

fn line_formatting(target: String, w: &&str, oneline: bool) -> String {
    let form = if oneline {
        target 
    } else if ONLY_BEFORE_LINE_FEED_WORDS.contains(w) {
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
    let colored = bracket_regex.replace_all(&trimed, coloring("$1".to_string(), Colors::Yellow)).to_string();
    Ok(colored)
}

fn trim_first_line(input: String) -> Result<String> {
    let first_line_regex = Regex::new(r"^(\s\n|\s)")?;
    let output = first_line_regex.replace_all(&input, "").to_string();
    Ok(output)
}

#[allow(unused_assignments)]
pub fn formatting(input: String, options: Options) -> Result<String> {
    let mut sql = input;
    let color = options.color.unwrap_or(Colors::Cyan);

    for w in RESERVED_WORDS.iter() {
        let from = format!(r"(?i)((\s+)|(?P<head>^)){}((\s+)|(?P<in>\())", w);
        let to = if options.lower {
            format!(" {} ", line_formatting(coloring(w.to_lowercase(), color), w, options.oneline))
        } else {
            format!(" {} ", line_formatting(coloring(w.to_uppercase(), color), w, options.oneline))
        };
        let regex = Regex::new(&from)?;
        sql = regex.replace_all(&sql, format!("{}{}", to, "$in")).to_string();
    }
    sql = formatting_equal(sql)?;
    sql = formatting_brackets(sql)?;
    sql = trim_first_line(sql)?;
    Ok(sql)
}

#[cfg(test)]
mod tests {
    use crate::formatter::{Colors, formatting, Options};

    #[test]
    fn default_color() {
        let options = Options { lower: false, oneline: true, color: None };
        let sql = "select * from users".to_string();
        let conved_sql = formatting(sql, options).unwrap();

        assert_eq!(conved_sql, "\x1b[36mSELECT\x1b[0m * \x1b[36mFROM\x1b[0m users");
    }

    #[test]
    fn non_color() {
        let options = Options { lower: false, oneline: true, color: Some(Colors::White) };
        let sql = "select * from users".to_string();
        let conved_sql = formatting(sql, options).unwrap();

        assert_eq!(conved_sql, "SELECT * FROM users");
    }

    #[test]
    fn lower() {
        let options = Options { lower: true, oneline: true, color: Some(Colors::White) };
        let sql = "seLEcT * frOM users".to_string();
        let conved_sql = formatting(sql, options).unwrap();

        assert_eq!(conved_sql, "select * from users");
    }

    #[test]
    fn feed_line() {
        let options = Options { lower: false, oneline: false,  color: Some(Colors::White) };
        let sql = "select * from users".to_string();
        let conved_sql = formatting(sql, options).unwrap();

        assert_eq!(conved_sql, "SELECT\n * \nFROM\n users");
    }
}
