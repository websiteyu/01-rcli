mod base64;
mod cha1305;
mod csv;
mod genpass;
mod http;
mod jwt;
mod text;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

pub use self::{
    base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64Subcommand},
    cha1305::{Cha1305DecryptOpt, Cha1305EncryptOpt, Cha1305Subcommand},
    csv::{CsvOpts, OutputFormat},
    genpass::GenPassOpts,
    http::{HttpServeOpts, HttpSubcommand},
    jwt::{JwtSignOpts, JwtSubcommand, JwtVerifyOpts},
    text::{TextKeyGenerateOpts, TextSignFormat, TextSignOpts, TextSubcommand, TextVerifyOpts},
};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcutor)]
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
    #[command(subcommand, about = "Jwt sign/verify")]
    Jwt(JwtSubcommand),
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
