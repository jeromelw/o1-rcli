use crate::{process_jwt_sign, process_jwt_verify, CmdExecutor};
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]

pub enum JwtSubCommand {
    #[command(about = "Sign by sub and  aud and exp")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a token")]
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

impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = process_jwt_sign(self.sub, self.aud, self.exp)?;
        println!("Token: {}", token);

        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub token: String,
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verified = process_jwt_verify(self.token)?;
        if verified {
            println!("Token verified");
        } else {
            println!("Token not verified");
        }
        Ok(())
    }
}
