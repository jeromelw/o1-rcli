mod base64;
mod chacha;
mod csv;
mod genpass;
mod text;

use self::{csv::CsvOpts, genpass::GenpassOpts};
use clap::Parser;
use std::path::{Path, PathBuf};

pub use self::{
    base64::Base64Format, base64::Base64SubCommand, chacha::ChachaFormat, chacha::ChachaSubCommand,
    csv::OutputFormat, text::SignatureFormat, text::TextSubCommand,
};

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
