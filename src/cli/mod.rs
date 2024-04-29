mod base64;
mod chacha;
mod csv;
mod genpass;
mod http_serve;
mod text;

use crate::CmdExecutor;

use clap::Parser;
use std::path::{Path, PathBuf};

pub use self::{base64::*, chacha::*, csv::*, genpass::*, http_serve::*, text::*};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON.")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate rand password")]
    GenPass(GenpassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text signature")]
    Text(TextSubCommand),
    #[command(subcommand, about = "Chacha20 encryption/decryption")]
    Chacha(ChachaSubCommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

impl CmdExecutor for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::GenPass(opts) => opts.execute().await,
            SubCommand::Base64(cmd) => cmd.execute().await,
            SubCommand::Text(cmd) => cmd.execute().await,
            SubCommand::Chacha(cmd) => cmd.execute().await,
            SubCommand::Http(cmd) => cmd.execute().await,
        }
    }
}

fn verify_file_exists(filepath: &str) -> Result<String, &'static str> {
    if filepath == "-" || Path::new(filepath).exists() {
        Ok(filepath.into())
    } else {
        Err("File does not exists")
    }
}

fn verify_path(filepath: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(filepath);
    if path.exists() && path.is_dir() {
        Ok(path.into())
    } else {
        Err("File does not exists")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file_exists() {
        assert_eq!(verify_file_exists("-"), Ok("-".into()));
        assert_eq!(verify_file_exists("*"), Err("File does not exists"));
        assert_eq!(verify_file_exists("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file_exists("not-exist"), Err("File does not exists"));
    }
}
