use std::fs::File;
use std::io::{stdin, Read};
use std::path::Path;

/// Errors that can occur while interpreting a Brainfuck program.
#[derive(Debug)]
pub enum BrainfuckError {
    BucketOutOfRange,
    ValueOutOfRange,
    UnexpectedLoopEnd,
}

/// Executes a Brainfuck program.
pub fn run(input: &str) -> Result<(), BrainfuckError> {
    use BrainfuckError::*;

    let bytes: Vec<u8> = input.bytes().collect();
    let mut buckets: Vec<u8> = vec![0];
    let mut ptr = 0;
    let mut saved_ti = Vec::new();
    let mut stdin = stdin();

    let mut ti = 0;
    while ti < bytes.len() {
        match bytes[ti] {
            b'>' => {
                if ptr == (buckets.len() - 1) {
                    buckets.push(0);
                }

                ptr += 1;
            },
            b'<' => {
                if ptr == 0 {
                    return Err(BucketOutOfRange);
                }

                ptr -= 1;
            },
            b'+' => {
                if buckets[ptr] == 255 {
                    return Err(ValueOutOfRange);
                }
                buckets[ptr] += 1;
            },
            b'-' => {
                if buckets[ptr] == 0 {
                    return Err(ValueOutOfRange);
                }
                buckets[ptr] -= 1;
            },
            b'.' => print!("{}", buckets[ptr] as char),
            b',' => {
                let mut buf: [u8; 1] = [0];
                stdin.read_exact(&mut buf).expect("to read from stdin");
                buckets[ptr] = buf[0];
            },
            b'[' => {
                if buckets[ptr] == 0 {
                    let mut depth = 0;
                    while bytes[ti] != b']' && depth != 0 {
                        if bytes[ti] == b'[' {
                            depth += 1;
                        } else if bytes[ti] == b']' {
                            depth -= 1;
                        }
                        ti += 1;
                    }
                } else {
                    saved_ti.push(ti);
                }
            },
            b']' => {
                if buckets[ptr] != 0 {
                    match saved_ti.last() {
                        None => return Err(UnexpectedLoopEnd),
                        Some(i) => ti = *i,
                    }
                } else if saved_ti.pop().is_none() {
                    return Err(UnexpectedLoopEnd);
                }
            },
            _ => {},
        }

        ti += 1;
    }
    println!();

    Ok(())
}

/// Reads the contents from a given file and calls run with the contents.
pub fn run_from_file(fp: &Path) -> Result<(), BrainfuckError> {
    let mut file = File::open(fp).expect("to open file");
    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("to read file contents");
    run(&source)
}
