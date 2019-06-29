extern crate bf;

use std::env;
use bf::{BrainfuckError, run_from_file};

fn main() -> Result<(), BrainfuckError> {
	if let Some(fp) = env::args().nth(1) {
		run_from_file(&fp)?;
	} else {
		eprintln!("fatal error: no input files");
	}

	Ok(())
}
