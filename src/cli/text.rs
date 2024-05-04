use core::fmt;
use std::{path::PathBuf, str::FromStr};

use crate::{
    process_decrypt, process_encrypt, process_generate_key, process_text_sign, process_text_verify,
    CmdExcutor,
};

use super::{parse_base64_format, verify_file, verify_path, Base64Format};
use anyhow::{Ok, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use tokio::fs;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcutor)]
pub enum TextSubcommand {
    #[command(about = "Sign a message with aprivate/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key")]
    Generate(TextKeyGenerateOpts),
    #[command(about = "Encrypt message")]
    Encrypt(Cha1305EncryptOpt),
    #[command(about = "Decrypt message")]
    Decrypt(Cha1305DecryptOpt),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long)]
    pub sig: String,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            e => Err(anyhow::anyhow!("Invalid format, {}", e)),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExcutor for TextSignOpts {
    async fn execute(self) -> Result<()> {
        let sign = process_text_sign(&self.input, &self.key, self.format)?;
        println!("{}", URL_SAFE_NO_PAD.encode(sign));
        Ok(())
    }
}

impl CmdExcutor for TextVerifyOpts {
    async fn execute(self) -> Result<()> {
        let verify = process_text_verify(&self.input, &self.key, self.format, &self.sig)?;
        println!("{}", verify);
        Ok(())
    }
}

impl CmdExcutor for TextKeyGenerateOpts {
    async fn execute(self) -> Result<()> {
        let key = process_generate_key(&self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                fs::write(name, &key[0]).await?;
            }
            TextSignFormat::Ed25519 => {
                let name = &self.output;
                fs::write(name.join("ed25519.sk"), &key[0]).await?;
                fs::write(name.join("ed25519.pk"), &key[1]).await?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcutor)]
pub enum Cha1305Subcommand {
    #[command(about = "Encrypt message")]
    Encrypt(Cha1305EncryptOpt),
    #[command(about = "Decrypt message")]
    Decrypt(Cha1305DecryptOpt),
}

#[derive(Debug, Parser)]
pub struct Cha1305EncryptOpt {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Cha1305DecryptOpt {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub nonce: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExcutor for Cha1305EncryptOpt {
    async fn execute(self) -> anyhow::Result<()> {
        let encrypted = process_encrypt(&self.input, &self.key, self.format)?;
        println!("{:?}", encrypted);
        Ok(())
    }
}

impl CmdExcutor for Cha1305DecryptOpt {
    async fn execute(self) -> anyhow::Result<()> {
        let decrypted = process_decrypt(&self.input, &self.key, &self.nonce, self.format)?;
        println!("{}", String::from_utf8(decrypted)?);
        Ok(())
    }
}
