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
