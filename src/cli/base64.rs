use core::fmt;
use std::str::FromStr;

use crate::{process_decode, process_encode, CmdExcutor};

use super::{parse_base64_format, verify_file};

use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64Subcommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a string to base64")]
    Decode(Base64DecodeOpts),
}

impl CmdExcutor for Base64Subcommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64Subcommand::Encode(opts) => opts.execute().await,
            Base64Subcommand::Decode(opts) => opts.execute().await,
        }
    }
}

impl CmdExcutor for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encode = process_encode(&self.input, self.format)?;
        println!("{}", encode);
        Ok(())
    }
}

impl CmdExcutor for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decode = process_decode(&self.input, self.format)?;
        let decode = String::from_utf8(decode)?;
        println!("{}", decode);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    URLSafe,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::URLSafe),
            v => anyhow::bail!("Unsupported format: {}", v),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::URLSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
