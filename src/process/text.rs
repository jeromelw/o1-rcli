use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::path::PathBuf;
use std::{collections::HashMap, io::Read};

use crate::process_genpass;

use crate::cli::SignatureFormat;

trait TextSigner {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
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

struct Ed25519Signer {
    key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Signer {
    fn new(key: &[u8; 32]) -> Self {
        let key = SigningKey::from_bytes(key);
        Self { key }
    }

    fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let verifying_key: VerifyingKey = (&signing_key).into();
        let mut map = HashMap::new();
        map.insert("ed25519.sk", signing_key.to_bytes().to_vec());
        map.insert("ed25519.vk", verifying_key.to_bytes().to_vec());
        Ok(map)
    }
}

impl Ed25519Verifier {
    fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self { key })
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = sig[..64].try_into()?;
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

pub fn process_sign(reader: &mut dyn Read, key: &[u8], format: SignatureFormat) -> Result<String> {
    let signer: Box<dyn TextSigner> = match format {
        SignatureFormat::Blake3 => Box::new(Blake3::try_new(key)?),

        SignatureFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
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
    let verifier: Box<dyn TextVerifier> = match format {
        SignatureFormat::Blake3 => Box::new(Blake3::try_new(key)?),

        SignatureFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
    };
    let result = verifier.verify(reader, sig)?;
    Ok(result)
}

pub fn process_generate(path: PathBuf, format: SignatureFormat) -> Result<()> {
    let map = match format {
        SignatureFormat::Blake3 => Blake3::generate()?,

        SignatureFormat::Ed25519 => Ed25519Signer::generate()?,
    };
    for (filename, key) in map {
        std::fs::write(path.join(filename), key)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_process_generate() {
    //     let format = SignatureFormat::Ed25519;
    //     let path = PathBuf::from("fixture");
    //     assert!(process_generate(path, format).is_ok());
    // }

    #[test]
    fn test_process_sign() -> Result<()> {
        let mut reader = "hello".as_bytes();
        let format = SignatureFormat::Ed25519;
        let key: &[u8] = include_bytes!("../../fixture/ed25519.sk");
        let ret = process_sign(&mut reader, key, format)?;
        println!("{:?}", ret);
        assert_eq!(ret, "-OfJNdwU5zWXc3sq_ZhCNy966s5P4-HNb7mhQsHCjZfOjvMxiSuk_mQLIG6BhUH6D53rikBM_jSa8LuGfgeUDQ");
        Ok(())
    }

    #[test]
    fn test_process_verify() -> Result<()> {
        let mut reader = "hello".as_bytes();
        let format = SignatureFormat::Ed25519;
        let key: &[u8] = include_bytes!("../../fixture/ed25519.vk");
        let sig = "-OfJNdwU5zWXc3sq_ZhCNy966s5P4-HNb7mhQsHCjZfOjvMxiSuk_mQLIG6BhUH6D53rikBM_jSa8LuGfgeUDQ";
        let sig = URL_SAFE_NO_PAD.decode(sig)?;
        let ret = process_verify(&mut reader, key, &sig, format)?;
        assert!(ret);
        Ok(())
    }
}
