mod base64;
mod csv;
mod genpass;

use clap::Parser;
use std::path::Path;

pub use self::{base64::Base64Format, base64::Base64Subcommand, csv::OutputFormat};

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
}

fn verify_input(input: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if input == "-" || Path::new(input).exists() {
        Ok(input.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::verify_input;

    #[test]
    fn test_verify_input() {
        assert_eq!(verify_input("-"), Ok("-".into()));
        assert_eq!(verify_input("output.json"), Ok("output.json".into()));
        assert_eq!(verify_input("no_output.json"), Err("File does not exist"));
    }
}
