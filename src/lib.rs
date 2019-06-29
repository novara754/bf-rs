use std::fs::File;
use std::io::{Read};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
	IncrPtr,
	DecrPtr,
	IncrData,
	DecrData,
	Output,
	Input,
	LoopBegin,
	LoopEnd,
	Empty,
	Error(u8),
}

#[derive(Debug)]
pub enum BrainfuckError {
	BucketOutOfRange,
	ValueOutOfRange,
	UnexpectedLoopEnd,
}

pub fn parse(source: &str) -> Vec<Token> {
	let bytes = source.as_bytes();
	bytes.into_iter().map(|byte| {
		match byte {
			b'>' => Token::IncrPtr,
			b'<' => Token::DecrPtr,
			b'+' => Token::IncrData,
			b'-' => Token::DecrData,
			b'.' => Token::Output,
			b',' => Token::Input,
			b'[' => Token::LoopBegin,
			b']' => Token::LoopEnd,
			b if b.is_ascii_whitespace() => Token::Empty,
			b => Token::Error(*b),
		}
	}).filter(|t| t != &Token::Empty).collect()
}

pub fn exec(tokens: Vec<Token>) -> Result<(), BrainfuckError> {
	let mut buckets: Vec<u8> = vec![0];
	let mut ptr = 0;
	let mut saved_ti = Vec::new();

	let mut ti = 0;
	while ti < tokens.len() {
		let token = tokens[ti];

		match token {
			Token::IncrPtr => {
				if ptr == (buckets.len() - 1) {
					buckets.push(0);
				}

				ptr += 1;
			},
			Token::DecrPtr => {
				if ptr == 0 {
					return Err(BrainfuckError::BucketOutOfRange);
				}

				ptr -= 1;
			},
			Token::IncrData => {
				if buckets[ptr] == 255 {
					return Err(BrainfuckError::ValueOutOfRange);
				}
				buckets[ptr] += 1;
			},
			Token::DecrData => {
				if buckets[ptr] == 0 {
					return Err(BrainfuckError::ValueOutOfRange);
				}
				buckets[ptr] -= 1;
			}
			Token::Output => print!("{}", buckets[ptr] as char),
			Token::LoopBegin =>	saved_ti.push(ti),
			Token::LoopEnd => {
				if buckets[ptr] != 0 {
					match saved_ti.last() {
						None => return Err(BrainfuckError::UnexpectedLoopEnd),
						Some(i) => ti = *i,
					}
				} else {
					if let None = saved_ti.pop() {
						return Err(BrainfuckError::UnexpectedLoopEnd);
					}
				}
			},
			// TODO: Implement support for stdin.
			_ => unimplemented!(),
		}

		ti += 1;
	}

	Ok(())
}

pub fn run(source: &str) -> Result<(), BrainfuckError> {
	let tokens = parse(source);
	exec(tokens)
}

pub fn run_from_file(fp: &str) -> Result<(), BrainfuckError> {
	let mut file = File::open(fp).expect("to open file");
	let mut source = String::new();
	file.read_to_string(&mut source).expect("to read file contents");
	run(&source)
}
