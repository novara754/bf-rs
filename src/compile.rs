use std::fs::{File, remove_file};
use std::path::Path;
use std::io::{Read, Write};
use std::process::Command;

fn with_indent(s: &str, n: usize) -> String {
	format!("{}{}", "\t".repeat(n), s)
}

pub fn generate_c(source: &str) -> String {
	let mut code = String::from("
fn main() {
	let mut buckets: [u8; 256] = [0; 256];
	let mut ptr = 0;
	let mut out = String::new();
	#[allow(unused_variables)]
	let stdin = std::io::stdin();
");
	let mut indentation = 1;
	let bytes: Vec<u8> = source.bytes().collect();
	let mut ti = 0;
	while ti < bytes.len() {
		match bytes[ti] {
			b'>' => code.push_str(&with_indent("ptr += 1;\n", indentation)),
			b'<' => code.push_str(&with_indent("ptr -= 1;\n", indentation)),
			b'+' => code.push_str(&with_indent("buckets[ptr] += 1;\n", indentation)),
			b'-' => code.push_str(&with_indent("buckets[ptr] -= 1;\n", indentation)),
			b'.' => code.push_str(&with_indent("out.push(buckets[ptr] as char);\n", indentation)),
			b',' => code.push_str(&with_indent("let buf: [u8; 1] = [0];\nstdin.read(&mut buf).expect(\"to read from stdin\");\nbuckets[ptr] = buf[0];\n", indentation)),
			b'[' => {
				code.push_str(&with_indent("while buckets[ptr] != 0 {\n", indentation));
				indentation += 1;
			},
			b']' => {
				indentation -= 1;
				code.push_str(&with_indent("}\n", indentation));
			}
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
