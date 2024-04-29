use std::{fmt, path::PathBuf, str::FromStr};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{get_content, get_reader, process_generate, process_sign, process_verify, CmdExecutor};

use super::{verify_file_exists, verify_path};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]

pub enum TextSubCommand {
    #[command(about = "Signature input by private key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signature by public key")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate key")]
    Generate(TextGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file_exists)]
    pub key: String,
    #[arg(short, long, value_parser = parse_signature_format)]
    pub format: SignatureFormat,
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let encode = process_sign(&mut reader, &key, self.format)?;
        println!("Signature result: {}", encode);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file_exists)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(long, value_parser = parse_signature_format)]
    pub format: SignatureFormat,
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let decoded = URL_SAFE_NO_PAD.decode(&self.sig)?;
        let verified = process_verify(&mut reader, &key, &decoded, self.format)?;
        if verified {
            println!("Signature verified");
        } else {
            println!("Signature not verified");
        }
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextGenerateOpts {
    #[arg(long, value_parser = parse_signature_format, default_value = "blake3")]
    pub format: SignatureFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

impl CmdExecutor for TextGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_generate(self.output, self.format)
    }
}

fn parse_signature_format(format: &str) -> Result<SignatureFormat, anyhow::Error> {
    format.parse()
}

#[derive(Debug, Clone, Copy)]
pub enum SignatureFormat {
    Blake3,
    Ed25519,
}

impl From<SignatureFormat> for &'static str {
    fn from(format: SignatureFormat) -> Self {
        match format {
            SignatureFormat::Blake3 => "blake3",
            SignatureFormat::Ed25519 => "ed25519",
        }
    }
}

impl FromStr for SignatureFormat {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "blake3" => Ok(SignatureFormat::Blake3),
            "ed25519" => Ok(SignatureFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for SignatureFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
