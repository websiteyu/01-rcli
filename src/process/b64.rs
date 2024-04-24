use std::io::Read;

use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::{cli::Base64Format, utils::get_reader};

pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    process_generate_encode(buf, format)
}

pub fn process_generate_encode(input: Vec<u8>, format: Base64Format) -> Result<String> {
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&input),
        Base64Format::URLSafe => URL_SAFE_NO_PAD.encode(&input),
    };
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::URLSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    Ok(decoded)
}

pub fn process_generate_decode(input: Vec<u8>, format: Base64Format) -> Result<Vec<u8>> {
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&input),
        Base64Format::URLSafe => URL_SAFE_NO_PAD.decode(&input),
    };
    Ok(decoded?)
}

#[cfg(test)]
mod tests {
    use crate::{process_decode, process_encode};

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = super::Base64Format::Standard;
        assert!(process_encode(input, format).is_ok())
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = super::Base64Format::Standard;
        assert!(process_decode(input, format).is_ok())
    }
}
