use bf::compile::compile_from_file;
use bf::interpret::{run_from_file, BrainfuckError};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "bf")]
struct Config {
    /// Generate a native executable binary
    #[structopt(short, long)]
    compile: bool,
    /// Files to process
    #[structopt(name = "INPUT", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> Result<(), BrainfuckError> {
    let config = Config::from_args();

    for file in config.files {
        if config.compile {
            if let Err(e) = compile_from_file(file.as_path()) {
                eprintln!("fatal error during compilation: {:?}.", e);
            }
        } else if let Err(e) = run_from_file(file.as_path()) {
            eprintln!("fatal error in {}: {:?}.", file.display(), e);
        }
    }

    Ok(())
}
