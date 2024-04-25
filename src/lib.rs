// #![deny(unsafe_code, missing_docs, clippy::unwrap_used)]
#![deny(unsafe_code, clippy::unwrap_used)]

mod cli;
mod process;
mod utils;

pub use cli::{
    Base64Subcommand, Cha1305Subcommand, HttpServeOpts, HttpSubcommand, Opts, SubCommand,
    TextSignFormat, TextSubcommand,
};
pub use process::{
    process_csv, process_decode, process_decrypt, process_encode, process_encrypt,
    process_generate_decode, process_generate_encode, process_generate_key, process_genpass,
    process_http_serve, process_text_sign, process_text_verify,
};
