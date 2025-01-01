use std::io::Read;

use base64::{engine::general_purpose::{STANDARD, URL_SAFE}, Engine};

use crate::cli::Base64Format;

pub fn process_encode(
    input: &str,
    format: Base64Format
) -> anyhow::Result<()> {
    let content = parse_from_file(input)?;

    let encode = match format {
        Base64Format::Standard => {
            STANDARD.encode(content)
        }
        Base64Format::UrlSafe => {
            URL_SAFE.encode(content)
        }
    };

    println!("{}", encode);

    Ok(())
}

pub fn process_decode(
    input: &str,
    format: Base64Format
) -> anyhow::Result<()> {
    let content = parse_from_file(input)?;

    let content = content.replace("\n", "").replace("\r", "").trim().to_string();

    let decode = match format {
        Base64Format::Standard => {
            // println!("{:?}", STANDARD.decode(&content));
            STANDARD.decode(&content)?
        }
        Base64Format::UrlSafe => {
            // println!("{:?}", URL_SAFE.decode(&content));
            URL_SAFE.decode(&content)?
        },
    };

    println!("{}", String::from_utf8(decode)?);
    Ok(())
}

fn parse_from_file(filename: &str) -> anyhow::Result<String> {
    let mut file = std::fs::File::open(filename).expect("File won't be open failed");
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(content)
}
