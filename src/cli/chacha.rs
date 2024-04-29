use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

use crate::{
    get_content, get_reader, process_chacha_generate, process_decrypt, process_encrypt, CmdExecutor,
};

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

impl CmdExecutor for ChachaSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            ChachaSubCommand::Encrypt(opts) => {
                opts.execute().await?;
            }
            ChachaSubCommand::Decrypt(opts) => {
                opts.execute().await?;
            }
            ChachaSubCommand::Generate(opts) => {
                opts.execute().await?;
            }
        }
        Ok(())
    }
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

impl CmdExecutor for EncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let (ret, nonce) = process_encrypt(&mut reader, &key, self.format)?;
        let ret = URL_SAFE_NO_PAD.encode(ret);

        let nonce = URL_SAFE_NO_PAD.encode(nonce);
        println!("Encrypted: {}", ret);
        println!("Nonce: {}", nonce);
        Ok(())
    }
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

impl CmdExecutor for DecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let nonce = URL_SAFE_NO_PAD.decode(&self.nonce)?;
        let result = process_decrypt(&mut reader, &key, &nonce, self.format)?;
        println!("Decrypted: {:?}", String::from_utf8(result)?);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct GenerateOpts {
    #[arg(long)]
    pub format: ChachaFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

impl CmdExecutor for GenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_chacha_generate(self.output, self.format)
    }
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
