use std::fs::{File, remove_file};
use std::path::Path;
use std::io::{Read, Write};
use std::process::Command;

pub fn generate_c(source: &str) -> String {
	let mut c_code = String::from("
#include <stdio.h>
#include <stdint.h>

int main(void) {
	uint8_t buckets[256] = {0};
	uint8_t *ptr = buckets;
	");

	let bytes: Vec<u8> = source.bytes().collect();
	let mut ti = 0;
	while ti < bytes.len() {
		match bytes[ti] {
			b'>' => c_code.push_str("ptr++;\n"),
			b'<' => c_code.push_str("ptr--;\n"),
			b'+' => c_code.push_str("(*ptr)++;\n"),
			b'-' => c_code.push_str("(*ptr)--;\n"),
			b'.' => c_code.push_str("printf(\"%c\", *ptr);\n"),
			b',' => c_code.push_str("*ptr = fgetc(stdin);\n"),
			b'[' => c_code.push_str("while (*ptr != 0) {\n"),
			b']' => c_code.push_str("} \n"),
			_ => {},
		}

		ti += 1;
	}
	c_code.push_str("
	printf(\"\\n\");
	return 0;
}
	");

	c_code
}

pub fn compile(source: &str, path: &Path) -> std::io::Result<()> {
	let c_path = path.with_extension("c");

	let code = generate_c(source);
	let mut file = File::create(&c_path)?;
	file.write_all(code.as_bytes())?;

	if cfg!(target_os = "windows") {
		Command::new("cmd")
			.arg("/C")
			.arg(format!("gcc -o {} {}", path.with_extension("").display(), c_path.display()))
			.spawn()?;
	} else {
		Command::new("sh")
			.arg("-c")
			.arg(format!("gcc -o {} {}", path.with_extension("").display(), c_path.display()))
			.spawn()?;
	}

	// remove_file(c_path)?;

	Ok(())
}

pub fn compile_from_file(fp: &str) -> std::io::Result<()> {
	let path = Path::new(fp);
	let mut file = File::open(fp)?;
	let mut source = String::new();
	file.read_to_string(&mut source)?;
	compile(&source, path)
}
