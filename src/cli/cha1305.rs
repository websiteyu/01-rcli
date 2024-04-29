use crate::{process_decrypt, process_encrypt, CmdExcutor};

use super::{parse_base64_format, verify_file, Base64Format};
use clap::Parser;

#[derive(Debug, Parser)]
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
    #[arg(short, long, value_parser = verify_file)]
    pub nonce: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Cha1305DecryptOpt {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, value_parser = verify_file)]
    pub nonce: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExcutor for Cha1305Subcommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Cha1305Subcommand::Encrypt(opts) => opts.execute().await,
            Cha1305Subcommand::Decrypt(opts) => opts.execute().await,
        }
    }
}

impl CmdExcutor for Cha1305EncryptOpt {
    async fn execute(self) -> anyhow::Result<()> {
        let encrypted = process_encrypt(&self.input, &self.key, &self.nonce, self.format)?;
        println!("{}", encrypted);
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
