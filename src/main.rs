use clap::Parser;
use rs_cli::{process_csv, process_gen_pass, Opts, SubCommand};

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
            process_gen_pass(opts.length, opts.uppercase, opts.lowercase, opts.number, opts.symbol)?;
        }
    }
    Ok(())
}
