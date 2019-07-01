use std::fs::{File, remove_file};
use std::path::Path;
use std::io::{Read, Write};
use std::process::Command;

pub fn generate_c(source: &str) -> String {
	let mut code = String::from("
fn main() {
	let mut buckets: [u8; 256] = [0; 256];
	let mut ptr = 0;
	let mut out = String::new();
	#[allow(unused_variables)]
	let stdin = std::io::stdin();
	");

	let bytes: Vec<u8> = source.bytes().collect();
	let mut ti = 0;
	while ti < bytes.len() {
		match bytes[ti] {
			b'>' => code.push_str("ptr += 1;\n"),
			b'<' => code.push_str("ptr -= 1;\n"),
			b'+' => code.push_str("buckets[ptr] += 1;\n"),
			b'-' => code.push_str("buckets[ptr] -= 1;\n"),
			b'.' => code.push_str("out.push(buckets[ptr] as char);\n"),
			b',' => code.push_str("let buf: [u8; 1] = [0];\nstdin.read(&mut buf).expect(\"to read from stdin\");\nbuckets[ptr] = buf[0];\n"),
			b'[' => code.push_str("while buckets[ptr] != 0 {\n"),
			b']' => code.push_str("} \n"),
			_ => {},
		}

		ti += 1;
	}
	code.push_str("
	println!(\"{}\", out);
}
	");

	code
}

pub fn compile(source: &str, path: &Path) -> std::io::Result<()> {
	let code_path = path.with_extension("rs");

	let code = generate_c(source);
	let mut file = File::create(&code_path)?;
	file.write_all(code.as_bytes())?;

	if cfg!(target_os = "windows") {
		Command::new("cmd")
			.arg("/C")
			.arg(format!("rustc -O {}", code_path.display()))
			.spawn()?;
	} else {
		Command::new("sh")
			.arg("-c")
			.arg(format!("rustc -O {}", code_path.display()))
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
