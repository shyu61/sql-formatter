use anyhow::Result;
use atty::{self, Stream};
use std::io::{self, Read};
use structopt::StructOpt;

use sql_formatter::formatter::{formatting, Options, Colors};

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INPUT")]
    input: Option<String>,

    // #[structopt(short = "u", long = "uppper", conflicts_with_all(&["lower"]))]
    // uppper: bool,

    #[structopt(short = "l", long = "lower", conflicts_with_all(&["uppper"]))]
    lower: bool,

    #[structopt(short = "o", long = "oneline")]
    oneline: bool,

    #[structopt(
        short = "c",
        long = "color",
        possible_values = &Colors::variants(),
        case_insensitive = true
    )]
    color: Option<Colors>,
}

fn is_pipe() -> bool {
    !atty::is(Stream::Stdin)
}

fn read_from_stdin() -> Result<String> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn main() -> Result<()> {
    let args = Opt::from_args();
    let sql = args.input;

    if sql.is_none() && !is_pipe() {
        Opt::clap().print_help()?;
        std::process::exit(1);
    }

    let sql = match sql {
        Some(i) => i,
        None => read_from_stdin()?,
    };
    if sql.is_empty() {
        Opt::clap().get_matches().usage();
    }

    let options = Options { lower: args.lower, oneline: args.oneline, color: args.color };
    let conved_sql = formatting(sql, options)?;

    #[allow(clippy::unit_arg)]
    Ok(println!("\n{}", conved_sql))
}
