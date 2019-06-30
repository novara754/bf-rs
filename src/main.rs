extern crate bf;

use std::env;
use bf::{BrainfuckError, run_from_file};

fn main() -> Result<(), BrainfuckError> {
	if let Some(fp) = env::args().nth(1) {
		if let Err(e) = run_from_file(&fp) {
			eprintln!("fatal error in {}: {:?}.", fp, e);
		}
	} else {
		eprintln!("fatal error: no input files");
	}

	Ok(())
}
