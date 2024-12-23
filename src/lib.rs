mod opts;
mod process;

pub use opts::{Opts, CsvOpts, SubCommand};
pub use process::process_csv;
pub use process::process_gen_pass;