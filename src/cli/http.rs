use std::path::PathBuf;

use clap::{command, Parser};

use crate::{process_http_serve, CmdExcutor};

use super::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubcommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}

impl CmdExcutor for HttpSubcommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubcommand::Serve(opts) => opts.execute().await,
        }
    }
}

impl CmdExcutor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, self.port).await?;
        Ok(())
    }
}
