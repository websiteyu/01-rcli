use anyhow::{Ok, Result};
use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Key, KeyInit, Nonce};
use std::fs;

use crate::{cli::Base64Format, process_decode, process_generate_encode, utils::get_vec};

pub trait Cha1305Encrypt {
    fn encrypt(&self, input: Vec<u8>) -> Result<Vec<u8>>;
}

pub trait Cha1305Decrypt {
    fn decrypt(&self, input: Vec<u8>) -> Result<Vec<u8>>;
}

pub struct Cha1305Processor {
    cipher: ChaCha20Poly1305,
    nonce: Vec<u8>,
}

impl Cha1305Processor {
    fn new(cipher: ChaCha20Poly1305, nonce: Vec<u8>) -> Self {
        Self { cipher, nonce }
    }

    fn try_new(key: &[u8], nonce: Vec<u8>) -> Result<Self> {
        let key = Key::from_slice(key);
        Ok(Self::new(ChaCha20Poly1305::new(key), nonce))
    }

    fn try_load(key_path: &str, nonce_path: &str) -> Result<Self> {
        let key = fs::read(key_path)?;
        if key.len() < 32 {
            panic!("key must be 32 bytes");
        }
        let key = key[..32].to_vec();
        let nonce = fs::read(nonce_path)?;
        if nonce.len() < 12 {
            panic!("key must be 12 bytes");
        }
        let nonce = key[..12].to_vec();
        Self::try_new(&key, nonce)
    }
}
impl Cha1305Encrypt for Cha1305Processor {
    fn encrypt(&self, input: Vec<u8>) -> Result<Vec<u8>> {
        Ok(self
            .cipher
            .encrypt(Nonce::from_slice(&self.nonce), input.as_ref())
            .expect("encryption failure!"))
    }
}

impl Cha1305Decrypt for Cha1305Processor {
    fn decrypt(&self, input: Vec<u8>) -> Result<Vec<u8>> {
        Ok(self
            .cipher
            .decrypt(Nonce::from_slice(&self.nonce), input.as_ref())
            .expect("encryption failure!"))
    }
}

pub fn process_encrypt(
    input: &str,
    key: &str,
    nonce: &str,
    format: Base64Format,
) -> Result<String> {
    // let nonce = fs::read(nonce)?;
    let buf = get_vec(input)?;

    let encryptor = Cha1305Processor::try_load(key, nonce)?;
    let encrypted = encryptor.encrypt(buf)?;
    process_generate_encode(encrypted, format)
}

pub fn process_decrypt(
    input: &str,
    key: &str,
    nonce: &str,
    format: Base64Format,
) -> Result<Vec<u8>> {
    // let nonce = fs::read(nonce)?;
    println!("process1");
    let input = process_decode(input, format)?;
    println!("process2");
    let encryptor: Cha1305Processor = Cha1305Processor::try_load(key, nonce)?;
    let encrypted = encryptor.decrypt(input)?;
    Ok(encrypted)
}

#[cfg(test)]
mod test {

    use anyhow::Result;

    use crate::{cli::Base64Format, process_generate_decode, process_generate_encode};

    use super::{Cha1305Decrypt, Cha1305Encrypt, Cha1305Processor};

    #[test]
    fn test_process_encrypt() -> Result<()> {
        let processor = Cha1305Processor::try_load(
            "./././/fixtures//cha1305-key.txt",
            "./././/fixtures//cha1305-nonce.txt",
        )?;
        let input = b"hello world!";
        let format = Base64Format::URLSafe;
        let encrypted = processor.encrypt(input.as_ref().to_vec())?;
        let encode_encrypted = process_generate_encode(encrypted, format)?;

        let decode_decrypted =
            process_generate_decode(encode_encrypted.as_bytes().to_vec(), format)?;
        let decrypted = processor.decrypt(decode_decrypted)?;
        assert_eq!(decrypted, input);

        Ok(())
    }
}
