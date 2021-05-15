use anyhow::Result;
use regex::Regex;

use crate::reserved_word::RESERVED_WORD;

pub struct Options {
    pub lower: bool,
}

fn formatting_equal(input: String) -> Result<String> {
    let regex = Regex::new(r"\s*=\s*")?;
    let output = regex.replace_all(&input, " = ").to_string();
    Ok(output)
}

fn formatting_in(input: String) -> Result<String> {
    let regex = Regex::new(r"(?i)(in)\s*")?;
    let output = regex.replace_all(&input, "$1").to_string();
    Ok(output)
}

#[allow(unused_assignments)]
pub fn formatting(input: String, options: Options) -> Result<String> {
    let mut sql = input;

    for w in RESERVED_WORD.iter() {
        let from = format!(r"(?i){} ", w);
        let to = if options.lower {
            format!("{} ", w.to_lowercase())
        } else if w == &"FROM" || w == &"SELECT" { // TODO
            format!("\n{}\n", w.to_uppercase())
        } else {
            format!("{} ", w.to_uppercase())
        };
        let regex = Regex::new(&from)?;
        sql = regex.replace_all(&sql, to).to_string()
    }
    sql = formatting_equal(sql)?;
    sql = formatting_in(sql)?;
    Ok(sql)
}
