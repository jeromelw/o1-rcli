use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::path::PathBuf;
use std::{collections::HashMap, io::Read};

use crate::process_genpass;

use crate::cli::SignatureFormat;

trait TextSigner {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerifier {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = process_genpass(32, false, false, false, false)?;
        let mut map = HashMap::new();
        map.insert("blake3.key", key.as_bytes().to_vec());
        Ok(map)
    }
}

pub fn process_sign(reader: &mut dyn Read, key: &[u8], format: SignatureFormat) -> Result<String> {
    let signer = match format {
        SignatureFormat::Blake3 => Blake3::try_new(key)?,
        SignatureFormat::Ed25519 => unimplemented!(),
    };
    let sig = signer.sign(reader)?;
    let sig = URL_SAFE_NO_PAD.encode(sig);
    Ok(sig)
}

pub fn process_verify(
    reader: &mut dyn Read,
    key: &[u8],
    sig: &[u8],
    format: SignatureFormat,
) -> Result<bool> {
    let verifier = match format {
        SignatureFormat::Blake3 => Blake3::try_new(key)?,
        SignatureFormat::Ed25519 => unimplemented!(),
    };
    let result = verifier.verify(reader, sig)?;
    Ok(result)
}

pub fn process_generate(path: PathBuf, format: SignatureFormat) -> Result<()> {
    match format {
        SignatureFormat::Blake3 => {
            let map = Blake3::generate()?;
            for (filename, key) in map {
                std::fs::write(path.join(filename), key)?;
            }
        }
        SignatureFormat::Ed25519 => unimplemented!(),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_generate() {
        let format = SignatureFormat::Blake3;
        let path = PathBuf::from("fixture");
        assert!(process_generate(path, format).is_ok());
    }
}
