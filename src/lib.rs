// #![deny(unsafe_code, missing_docs, clippy::unwrap_used)]
#![deny(unsafe_code, clippy::unwrap_used)]

mod cli;
mod process;
mod utils;

use anyhow::Result;

pub use cli::{
    Base64Subcommand, Cha1305Subcommand, HttpServeOpts, HttpSubcommand, JwtSignOpts, JwtSubcommand,
    JwtVerifyOpts, Opts, SubCommand, TextSignFormat, TextSubcommand,
};
use enum_dispatch::enum_dispatch;
pub use process::{
    process_csv, process_decode, process_decrypt, process_encode, process_encrypt,
    process_generate_decode, process_generate_encode, process_generate_key, process_genpass,
    process_http_serve, process_text_sign, process_text_verify,
};

use cli::{
    Base64DecodeOpts, Base64EncodeOpts, Cha1305DecryptOpt, Cha1305EncryptOpt, CsvOpts, GenPassOpts,
    TextKeyGenerateOpts, TextSignOpts, TextVerifyOpts,
};

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExcutor {
    async fn execute(self) -> Result<()>;
}
