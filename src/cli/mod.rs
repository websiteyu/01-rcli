mod base64;
mod cha1305;
mod csv;
mod genpass;
mod http;
mod text;

use clap::Parser;
use std::path::{Path, PathBuf};

use crate::CmdExcutor;

pub use self::{
    base64::{Base64Format, Base64Subcommand},
    cha1305::Cha1305Subcommand,
    csv::OutputFormat,
    http::{HttpServeOpts, HttpSubcommand},
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
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64Subcommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubcommand),
    #[command(subcommand, about = "Chacha20-poly1305 encrypt/decrypt")]
    Cha1305(Cha1305Subcommand),
    #[command(subcommand, about = "Http server")]
    Http(HttpSubcommand),
}

impl CmdExcutor for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => {
                opts.execute().await?;
            }
            SubCommand::GenPass(opts) => {
                opts.execute().await?;
            }
            SubCommand::Base64(cmd) => cmd.execute().await?,
            SubCommand::Text(_) => todo!(),
            SubCommand::Cha1305(_) => todo!(),
            SubCommand::Http(_) => todo!(),
        }
        Ok(())
    }
}

pub fn verify_file(input: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if input == "-" || Path::new(input).exists() {
        Ok(input.into())
    } else {
        Err("File does not exist")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path not exists or is not a directory")
    }
}

pub fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
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
