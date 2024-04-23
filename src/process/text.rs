use std::{fs, io::Read, path::Path};

use crate::{cli::TextSignFormat, utils::get_reader};
use anyhow::{Ok, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use super::gen_pass;

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
    // ,
    // {
    //     let key = fs::read(&path)?;
    //     Self::try_new(&key)
    // }
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

#[cfg(test)]
mod tests {
    use crate::process::text::{Ed25519Signer, Ed25519Verifier, TextVerify};

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
}
