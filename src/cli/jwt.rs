use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{
    process::{process_jwt_sign, process_jwt_verify},
    CmdExcutor,
};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcutor)]
pub enum JwtSubcommand {
    #[command(about = "Sign a message with aprivate/shared key")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(long)]
    pub sub: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long)]
    pub exp: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub token: String,
}

impl CmdExcutor for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let signed = process_jwt_sign(self.sub, self.aud, self.exp)?;
        println!("{}", signed);
        Ok(())
    }
}

impl CmdExcutor for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token_data = process_jwt_verify(&self.token);
        println!("{:?}", token_data);
        Ok(())
    }
}
