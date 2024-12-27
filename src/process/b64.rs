use std::io::Read;

use base64::{engine::general_purpose::URL_SAFE, Engine};

pub fn process_encode(input: &str) -> anyhow::Result<()> {
    let content = parse_from_file(input)?;

    let encoded = URL_SAFE.encode(content);
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str) -> anyhow::Result<()> {
    let decode = input;

    println!("{}", decode);
    Ok(())
}

fn parse_from_file(filename: &str) -> anyhow::Result<String> {
    let mut file = std::fs::File::open(filename).expect("File won't be open failed");
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(content)
}