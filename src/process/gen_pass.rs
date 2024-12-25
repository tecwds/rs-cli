use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHIJKLNMOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklnmopqastuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

#[derive(Debug, Clone)]
pub struct GenPassProps {
    pub length: u8,
    pub upper: bool,
    pub lower: bool,
    pub number: bool,
    pub symbol: bool,
}


pub fn process_gen_pass(
    props: &GenPassProps,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut passwd = Vec::new();
    let mut chars = Vec::new();

    if props.upper {
        chars.extend_from_slice(UPPER);
        passwd.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if props.lower {
        chars.extend_from_slice(LOWER);
        passwd.push(*LOWER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if props.number {
        chars.extend_from_slice(NUMBER);
        passwd.push(*NUMBER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if props.symbol {
        chars.extend_from_slice(SYMBOL);
        passwd.push(*SYMBOL.choose(&mut rng).expect("UPPER won't be empty"));
    }

    for _ in 0..(props.length as usize - passwd.len()) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        passwd.push(*c);
    }

    passwd.shuffle(&mut rng);

    println!("{}", String::from_utf8(passwd)?);

    Ok(())
}
