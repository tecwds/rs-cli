use base64::{engine::general_purpose::URL_SAFE, Engine};

pub fn process_encode(input: &str) -> anyhow::Result<()> {
    let encoded = URL_SAFE.encode(input);
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str) -> anyhow::Result<()> {
    let decode = input;
    println!("{}", decode);
    Ok(())
}
