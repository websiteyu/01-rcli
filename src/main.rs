use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_sign,
    process_text_verify, Base64Subcommand, Opts, SubCommand, TextSubcommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.no_uppercase,
                opts.no_lowercase,
                opts.no_number,
                opts.no_symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64Subcommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
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
                process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{:?}", opts);
            }
            TextSubcommand::Verify(opts) => {
                process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{:?}", opts);
            }
        },
    }

    Ok(())
}
