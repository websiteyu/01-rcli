// #![deny(unsafe_code, missing_docs, clippy::unwrap_used)]
#![deny(unsafe_code, clippy::unwrap_used)]

mod cli;
mod process;
mod utils;

pub use cli::{Base64Subcommand, Opts, SubCommand, TextSignFormat, TextSubcommand};
pub use process::{
    process_csv, process_decode, process_encode, process_genpass, process_text_sign,
    process_text_verify,
};
