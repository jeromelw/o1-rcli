use clap::Parser;

use crate::CmdExecutor;

#[derive(Debug, Parser)]
pub struct GenpassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = false)]
    pub no_uppercase: bool,

    #[arg(long, default_value_t = false)]
    pub no_lowercace: bool,

    #[arg(long, default_value_t = false)]
    pub no_number: bool,

    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
}

impl CmdExecutor for GenpassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        crate::process_genpass(
            self.length,
            self.no_uppercase,
            self.no_lowercace,
            self.no_number,
            self.no_symbol,
        )?;
        Ok(())
    }
}
