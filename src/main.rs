use clap::Parser;
use rs_cli::{
    process_csv, process_decode, process_encode, process_gen_pass, Base64SubCommand, GenPassProps, Opts, SubCommand
};

fn main() -> anyhow::Result<()> {
    // 获取命令参数
    let opts = Opts::parse();

    match opts.cmd {
        // csv 相关操作
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = &opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };

            // 处理
            process_csv(&opts.input, &output, opts.delimiter, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            process_gen_pass(
                &GenPassProps {
                    length: opts.length,
                    upper: opts.uppercase,
                    lower: opts.lowercase,
                    number: opts.number,
                    symbol: opts.symbol
                }
            )?;
        }
        SubCommand::Base64(base64) => match base64 {
            Base64SubCommand::Encode(opts) => process_encode(&opts.input)?,
            Base64SubCommand::Decode(opts) => process_decode(&opts.input)?,
        },
    }
    Ok(())
}
