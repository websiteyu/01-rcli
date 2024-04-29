use clap::Parser;
use rcli::{CmdExcutor, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    opts.cmd.execute().await
}
