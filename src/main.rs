use clap::Parser;
use rcli::{process_csv, process_genpass, Base64Subcommand, Opts, SubCommand};

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
                println!("encode: {:?}", opts);
            }
            Base64Subcommand::Decode(opts) => {
                println!("decode: {:?}", opts);
            }
        },
    }

    Ok(())
}
