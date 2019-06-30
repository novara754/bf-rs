extern crate bf;

use std::env;
use bf::interpret::{BrainfuckError, run_from_file};
use bf::compile::compile_from_file;

fn main() -> Result<(), BrainfuckError> {
	if let Some(arg) = env::args().nth(1) {
		match &arg[..] {
			"-c" => {
				if let Some(fp) = env::args().nth(2) {
					if let Err(e) = compile_from_file(&fp) {
						eprintln!("fatal error during compilation: {:?}.", e);
					}
				} else {
					eprintln!("fatal error: no input files");
				}
			},
			fp => {
				if let Err(e) = run_from_file(&fp) {
					eprintln!("fatal error in {}: {:?}.", fp, e);
				}
			}
		}
	} else {
		eprintln!("fatal error: no input files");
	}

	Ok(())
}
