use anyhow::Result;
use regex::Regex;

use crate::reserved_word::RESERVED_WORD;

const BLUE: u32 = 36;

pub struct Options {
    pub lower: bool,
}

fn coloring(target: String, color_num: u32) -> String {
    let form = format!("\x1b[{}m{}\x1b[0m", color_num, target);
    form
}

fn formatting_equal(input: String) -> Result<String> {
    let regex = Regex::new(r"\s*=\s*")?;
    let output = regex.replace_all(&input, " = ").to_string();
    Ok(output)
}

fn formatting_in(input: String) -> Result<String> {
    let regex = Regex::new(r"(?i)\s+(\x1b\[\d{2}m)?(in)(\x1b\[0m)?\s*")?;
    let output = regex.replace_all(&input, " $1$2$3").to_string();
    Ok(output)
}

#[allow(unused_assignments)]
pub fn formatting(input: String, options: Options) -> Result<String> {
    let mut sql = input;

    for w in RESERVED_WORD.iter() {
        let from = format!(r"(?i)(\s+|^){}((\s+)|(?P<in>\())", w);
        let to = if options.lower {
            format!(" {} ", coloring(w.to_lowercase(), BLUE))
        } else if w == &"FROM" || w == &"SELECT" { // TODO
            format!("\n{}\n", coloring(w.to_uppercase(), BLUE))
        } else {
            format!("{} ", coloring(w.to_uppercase(), BLUE))
        };
        let regex = Regex::new(&from)?;
        sql = regex.replace_all(&sql, format!("{}{}{}", "$1", to, "$in")).to_string()
    }
    sql = formatting_equal(sql)?;
    sql = formatting_in(sql)?;
    Ok(sql)
}
