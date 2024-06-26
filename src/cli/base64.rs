use std::{fmt, str::FromStr};

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::CmdExecutor;

use super::verify_file_exists;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        crate::process_encode(&self.input, self.format)
    }
}

impl CmdExecutor for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        crate::process_decode(&self.input, self.format)
    }
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
