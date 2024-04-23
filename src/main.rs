use std::fs;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_generate_key, process_genpass,
    process_text_sign, process_text_verify, Base64Subcommand, Opts, SubCommand, TextSignFormat,
    TextSubcommand,
};
use zxcvbn::zxcvbn;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };
            let result = process_csv(&opts.input, output, opts.format);
            println!("{}", result.is_ok())
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.no_uppercase,
                opts.no_lowercase,
                opts.no_number,
                opts.no_symbol,
            )?;
            println!("{}", password);

            let estimate = zxcvbn(&password, &[])?;
            eprintln!("Password strength: {}", estimate.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Encode(opts) => {
                let encode = process_encode(&opts.input, opts.format)?;
                println!("{}", encode);
            }
            Base64Subcommand::Decode(opts) => {
                let decode = process_decode(&opts.input, opts.format)?;
                let decode = String::from_utf8(decode)?;
                println!("{}", decode);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubcommand::Sign(opts) => {
                // match opts.format {
                //     TextSignFormat::Blake3 => {
                //         process_text_sign(&opts.input, &opts.key, opts.format)?;
                //     }
                //     TextSignFormat::Ed25519 => todo!(),
                // }
                let sign = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", URL_SAFE_NO_PAD.encode(sign));
            }
            TextSubcommand::Verify(opts) => {
                let verify = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{}", verify);
            }
            TextSubcommand::Generate(opts) => {
                let key = process_generate_key(&opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
                println!("{:?}", key);
            }
        },
    }

    Ok(())
}
