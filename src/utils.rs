use anyhow::Result;
use std::{fs, fs::File, io::Read};

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

pub fn get_vec(input: &str) -> Result<Vec<u8>> {
    if input == "-" {
        let mut reader = std::io::stdin();
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(buf)
    } else {
        Ok(fs::read(input)?)
    }
}
