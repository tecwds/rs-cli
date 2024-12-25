mod base64;
mod csv;
mod gen;

pub use self::csv::CsvOpts;
pub use self::csv::OutputFormat;
pub use base64::Base64SubCommand;

pub use gen::GenPassOpts;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rs-csv", version = "0.1.0", author = "tecwds", about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV or convert CSV to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),
}

/// 验证输入的文件是否存在
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}

/// 验证输入的数字长度
fn verify_input_passwd_len(length: &str) -> Result<u8, String> {
    let len = length.parse::<u8>();

    if len.is_err() {
        return Err(format!("{} is too large", length));
    }

    let len = len.unwrap();

    if len < 4 {
        return Err(format!("{} is too small", length));
    }

    Ok(len)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            verify_input_file("not-exists"),
            Err("File does not exist".into())
        );
    }
}
