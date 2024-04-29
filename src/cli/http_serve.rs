use std::path::PathBuf;

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::CmdExecutor;

use super::verify_path;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum HttpSubCommand {
    #[command(about = "Signature input by private key")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path)]
    pub dirtory: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        crate::process_http_serve(self.dirtory, self.port).await
    }
}
