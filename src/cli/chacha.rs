use super::{verify_file_exists, verify_path};
use clap::Parser;
use std::path::PathBuf;
use std::{fmt, str::FromStr};

#[derive(Debug, Parser)]
pub enum ChachaSubCommand {
    #[command(name = "encrypt", about = "Encrypt by chacha stream cipher")]
    Encrypt(EncryptOpts),
    #[command(name = "decrypt", about = "Decrypt by chacha stream cipher")]
    Decrypt(DecryptOpts),
    #[command(name = "generate", about = "Generate chacha stream cipher key")]
    Generate(GenerateOpts),
}

#[derive(Debug, Parser)]
pub struct EncryptOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file_exists)]
    pub key: String,
    #[arg(short, long)]
    pub format: ChachaFormat,
}

#[derive(Debug, Parser)]
pub struct DecryptOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file_exists)]
    pub key: String,
    #[arg(short, long)]
    pub nonce: String,
    #[arg(short, long)]
    pub format: ChachaFormat,
}

#[derive(Debug, Parser)]
pub struct GenerateOpts {
    #[arg(long)]
    pub format: ChachaFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum ChachaFormat {
    ChaCha20Poly1305,
    XChaCha20Poly1305,
    ChaCha12Poly1305,
    ChaCha8Poly1305,
}

impl From<ChachaFormat> for &'static str {
    fn from(format: ChachaFormat) -> Self {
        match format {
            ChachaFormat::ChaCha20Poly1305 => "chacha20poly1305",
            ChachaFormat::XChaCha20Poly1305 => "xchacha20poly1305",
            ChachaFormat::ChaCha12Poly1305 => "chacha12poly1305",
            ChachaFormat::ChaCha8Poly1305 => "chacha8poly1305",
        }
    }
}

impl FromStr for ChachaFormat {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "chacha20poly1305" => Ok(ChachaFormat::ChaCha20Poly1305),
            "xchacha20poly1305" => Ok(ChachaFormat::XChaCha20Poly1305),
            "chacha12poly1305" => Ok(ChachaFormat::ChaCha12Poly1305),
            "chacha8poly1305" => Ok(ChachaFormat::ChaCha8Poly1305),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for ChachaFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
