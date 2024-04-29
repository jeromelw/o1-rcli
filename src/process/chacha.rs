use crate::ChachaFormat;
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305 as OtherChaCha20Poly1305,
};
use std::{collections::HashMap, io::Read};

trait ChachaEncrypt {
    fn encrypt(&self, reader: &mut dyn Read) -> Result<(Vec<u8>, Vec<u8>)>;
}

trait ChachaDecrypt {
    fn decrypt(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<Vec<u8>>;
}

impl ChachaEncrypt for ChaCha20Poly1305 {
    fn encrypt(&self, reader: &mut dyn Read) -> Result<(Vec<u8>, Vec<u8>)> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let cipher = OtherChaCha20Poly1305::new((&self.key[..32]).into());
        let nonce = OtherChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ret = cipher.encrypt(&nonce, buf.as_ref());
        let ret = match ret {
            Err(_) => return Err(anyhow::anyhow!("Encrypt failed")),
            Ok(ret) => ret,
        };
        Ok((ret, nonce[0..12].to_vec()))
    }
}

impl ChachaDecrypt for ChaCha20Poly1305 {
    fn decrypt(&self, reader: &mut dyn Read, nonce: &[u8]) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let cipher = OtherChaCha20Poly1305::new((&self.key[..32]).into());
        let buf = URL_SAFE_NO_PAD.decode(&buf)?;
        let ret = cipher.decrypt((&nonce[..12]).into(), buf.as_ref());
        let ret = match ret {
            Err(_) => return Err(anyhow::anyhow!("Decrypt failed")),
            Ok(ret) => ret,
        };
        Ok(ret)
    }
}

struct ChaCha20Poly1305 {
    key: [u8; 32],
}

impl ChaCha20Poly1305 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = OtherChaCha20Poly1305::generate_key(&mut OsRng);
        let mut map = HashMap::new();
        map.insert("ChaCha20Poly1305.txt", key.to_vec());
        Ok(map)
    }
}

pub fn process_encrypt(
    reader: &mut dyn Read,
    key: &[u8],
    format: ChachaFormat,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let encryptor: Box<dyn ChachaEncrypt> = match format {
        ChachaFormat::ChaCha20Poly1305 => Box::new(ChaCha20Poly1305::try_new(key)?),
        ChachaFormat::XChaCha20Poly1305
        | ChachaFormat::ChaCha12Poly1305
        | ChachaFormat::ChaCha8Poly1305 => todo!(),
    };
    let (ret, nonce) = encryptor.encrypt(reader)?;

    Ok((ret, nonce))
}

pub fn process_decrypt(
    reader: &mut dyn Read,
    key: &[u8],
    nonce: &[u8],
    format: ChachaFormat,
) -> Result<Vec<u8>> {
    let decrypter: Box<dyn ChachaDecrypt> = match format {
        ChachaFormat::ChaCha20Poly1305 => Box::new(ChaCha20Poly1305::try_new(key)?),
        ChachaFormat::XChaCha20Poly1305
        | ChachaFormat::ChaCha12Poly1305
        | ChachaFormat::ChaCha8Poly1305 => todo!(),
    };
    let result = decrypter.decrypt(reader, nonce)?;
    Ok(result)
}

pub fn process_chacha_generate(path: std::path::PathBuf, format: ChachaFormat) -> Result<()> {
    let map = match format {
        ChachaFormat::ChaCha20Poly1305 => ChaCha20Poly1305::generate()?,
        ChachaFormat::XChaCha20Poly1305
        | ChachaFormat::ChaCha12Poly1305
        | ChachaFormat::ChaCha8Poly1305 => todo!(),
    };
    for (filename, key) in map {
        std::fs::write(path.join(filename), key)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encrypt_decrypt() -> Result<()> {
        let format = ChachaFormat::ChaCha20Poly1305;
        let mut reader = "hello".as_bytes();
        let key: &[u8] = include_bytes!("../../fixture/ChaCha20Poly1305.txt");
        let (ret, nonce) = process_encrypt(&mut reader, key, format)?;
        let ret = URL_SAFE_NO_PAD.encode(ret);
        let mut read = ret.as_bytes();
        let decrypt_ret = process_decrypt(&mut read, key, &nonce, format);

        let decrypt_ret = if let Ok(ret) = decrypt_ret {
            String::from_utf8(ret)?
        } else {
            "Fatal error".to_string()
        };

        assert_eq!("hello", decrypt_ret);
        Ok(())
    }
}
