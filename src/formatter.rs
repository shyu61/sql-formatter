use anyhow::Result;
use regex::Regex;

// use crate::reserved_word::RESERVED_WORD;

pub struct Options {
    pub lower: bool,
}

#[allow(unused_assignments)]
pub fn formatting(input: String, options: Options) -> Result<String> {
    let mut sql = String::new();
    let select = Regex::new(r"(?i)select")?;

    if options.lower {
        sql = select.replace(&input, "select").to_string();
    } else {
        sql = select.replace(&input, "SELECT").to_string();
    }
    Ok(sql)
}
