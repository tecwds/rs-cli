use clap::Parser;
use rs_cli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    /// 获取命令参数
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
            process_csv(&opts.input, &output, opts.format)?
        }
    }
    Ok(())
}
