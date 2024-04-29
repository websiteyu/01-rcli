use clap::Parser;
use rcli::{CmdExcutor, Opts, SubCommand};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => opts.execute().await,
        SubCommand::GenPass(opts) => opts.execute().await,
        SubCommand::Base64(cmd) => cmd.execute().await,
        SubCommand::Text(cmd) => cmd.execute().await,
        SubCommand::Cha1305(cmd) => cmd.execute().await,
        SubCommand::Http(cmd) => cmd.execute().await,
    }

    // Ok(())
}
