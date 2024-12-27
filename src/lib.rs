mod cli;
mod process;

pub use process::{process_csv, process_decode, process_encode, process_gen_pass, GenPassProps};

pub use cli::{Base64SubCommand, CsvOpts, GenPassOpts, Opts, OutputFormat, SubCommand};
