mod base64;
mod csv;
mod genpass;
mod text;

use clap::Parser;
use std::path::Path;

pub use self::{
    base64::{Base64Format, Base64Subcommand},
    csv::OutputFormat,
    text::{TextSignFormat, TextSubcommand},
};

use self::{csv::CsvOpts, genpass::GenPassOpts};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show Csv, or convert Csv to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64Subcommand),
    #[command(subcommand)]
    Text(TextSubcommand),
}

fn verify_file(input: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if input == "-" || Path::new(input).exists() {
        Ok(input.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::verify_file;

    #[test]
    fn test_verify_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("output.json"), Ok("output.json".into()));
        assert_eq!(verify_file("no_output.json"), Err("File does not exist"));
    }
}
