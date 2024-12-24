use csv::ReaderBuilder;
use std::fs;
use serde_json::Value;
use crate::opts::OutputFormat;

pub fn process_csv(input: &str, output: &str, delimiter: char, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = ReaderBuilder::new().delimiter(u8::try_from(delimiter)?).from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();

    for result in reader.records() {
        let record= result?;
        let value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(value);
    }

    let content = match format {
        OutputFormat::Json => {
            serde_json::to_string_pretty(&ret)?
        }
        OutputFormat::Yaml => {
            serde_yaml::to_string(&ret)?
        }
    };

    fs::write(output, content)?;
    Ok(())
}
