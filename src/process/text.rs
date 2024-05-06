use std::{fs, io::Read, path::Path};

use crate::{
    cli::{Base64Format, TextSignFormat},
    process_decode, process_generate_encode,
    utils::{get_reader, get_vec},
};
use anyhow::{Ok, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use bincode::{deserialize, serialize};
use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Key, KeyInit, Nonce};
use serde::{Deserialize, Serialize};

use super::gen_pass::{self, genpass_length};

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> Result<bool>;
}

trait KeyLoader {
    fn load<T>(path: T) -> Result<Self>
    where
        T: AsRef<Path>,
        Self: Sized;
}

trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519Signer {
    key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sign)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = gen_pass::genpass_length(32)?;
        Ok(vec![key])
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Blake3 {
        Blake3 { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        Ok(Blake3::new(key))
    }
}

impl KeyLoader for Blake3 {
    fn load<T>(path: T) -> Result<Self>
    where
        T: AsRef<Path>,
        Self: Sized,
    {
        let key = fs::read(&path)?;
        Self::try_new(&key)
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Ed25519Signer {
        Ed25519Signer { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Ed25519Signer::new(key))
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key();
        Ok(vec![sk.to_bytes().to_vec(), pk.to_bytes().to_vec()])
    }
}

impl KeyLoader for Ed25519Signer {
    fn load<T>(path: T) -> Result<Self>
    where
        T: AsRef<Path>,
        Self: Sized,
    {
        let key = fs::read(&path)?;
        Self::try_new(&key)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Ed25519Verifier { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?);
        Ok(Self::new(key?))
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load<T>(path: T) -> Result<Self>
    where
        T: AsRef<Path>,
        Self: Sized,
    {
        let key = fs::read(&path)?;
        Self::try_new(&key)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify(&buf, &sig).is_ok();
        Ok(ret)
    }
}

pub fn process_text_sign(
    input: &str,
    key: &str,
    format: TextSignFormat,
) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sig: &str,
) -> Result<bool> {
    let mut reader = get_reader(input)?;

    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
    };

    Ok(verified)
}

pub fn process_generate_key(format: &TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

pub trait Cha1305Encrypt {
    fn encrypt(&self, input: Vec<u8>) -> Result<Cha1305Resp>;
}

pub trait Cha1305Decrypt {
    fn decrypt(&self, input: Vec<u8>) -> Result<Vec<u8>>;
}

pub struct Cha1305Processor {
    cipher: ChaCha20Poly1305,
    nonce: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cha1305Resp {
    pub message: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl Cha1305Resp {
    pub fn new(message: Vec<u8>, nonce: Vec<u8>) -> Self {
        Self { message, nonce }
    }
}

impl Cha1305Processor {
    fn new(cipher: ChaCha20Poly1305, nonce: Vec<u8>) -> Self {
        Self { cipher, nonce }
    }

    fn try_new(key: &[u8], nonce: Vec<u8>) -> Result<Self> {
        let key = Key::from_slice(key);
        Ok(Self::new(ChaCha20Poly1305::new(key), nonce))
    }

    fn try_load(key_path: &str) -> Result<Self> {
        let key = fs::read(key_path)?;
        if key.len() < 32 {
            panic!("key must be 32 bytes");
        }
        let key = key[..32].to_vec();
        let nonce = genpass_length(12)?;
        Self::try_new(&key, nonce)
    }

    fn try_load_full(key_path: &str, nonce: Vec<u8>) -> Result<Self> {
        let key = fs::read(key_path)?;
        if key.len() < 32 {
            panic!("key must be 32 bytes");
        }
        let key = key[..32].to_vec();
        Self::try_new(&key, nonce)
    }
}
impl Cha1305Encrypt for Cha1305Processor {
    fn encrypt(&self, input: Vec<u8>) -> Result<Cha1305Resp> {
        let input = self
            .cipher
            .encrypt(Nonce::from_slice(&self.nonce), input.as_ref())
            .expect("encryption failure!");
        Ok(Cha1305Resp::new(input, self.nonce.clone()))
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

pub fn process_encrypt(input: &str, key: &str, format: Base64Format) -> Result<String> {
    let buf = get_vec(input)?;

    let encryptor = Cha1305Processor::try_load(key)?;
    let encrypted = encryptor.encrypt(buf)?;
    process_generate_encode(&serialize(&encrypted)?, format)
}

pub fn process_decrypt(input: &str, key: &str, format: Base64Format) -> Result<Vec<u8>> {
    let input = process_decode(input, format)?;
    let resp: Cha1305Resp = deserialize(&input)?;
    println!("encrypted : {:?}", resp);
    let encryptor: Cha1305Processor = Cha1305Processor::try_load_full(key, resp.nonce)?;
    let encrypted = encryptor.decrypt(resp.message)?;
    Ok(encrypted)
}

#[cfg(test)]
mod tests {
    use crate::process::text::{
        Cha1305Decrypt, Cha1305Encrypt, Cha1305Processor, Ed25519Signer, Ed25519Verifier,
        TextVerify,
    };

    use super::{Blake3, KeyLoader, TextSign};
    use anyhow::Result;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let input = b"wangmy@gmail.com";
        let signer = Blake3::load("fixtures/blake3.txt")?;
        let sig = signer.sign(&mut &input[..])?;
        assert!(signer.verify(&mut &input[..], &sig).is_ok());

        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let input = b"wangmy@gmail.com";
        let signer = Ed25519Signer::load("fixtures/ed25519.sk")?;
        let sig = signer.sign(&mut &input[..])?;
        let verifier = Ed25519Verifier::load("fixtures/ed25519.pk")?;
        assert!(verifier.verify(&mut &input[..], &sig).is_ok());

        Ok(())
    }

    #[test]
    fn test_process_encrypt() -> Result<()> {
        let processor = Cha1305Processor::try_load("./././/fixtures//cha1305-key.txt")?;
        let input = b"hello world!";
        let encrypted = processor.encrypt(input.as_ref().to_vec())?;

        let decrypted = processor.decrypt(encrypted.message)?;
        assert_eq!(decrypted, input);

        Ok(())
    }
}
