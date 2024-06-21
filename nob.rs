// original nobuild from tsoding

use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;
use std::process::Command;

pub struct NobStringBuilder {
    items: Vec<u8>,
}

impl NobStringBuilder {
    pub fn new() -> Self {
        NobStringBuilder { items: Vec::new() }
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }
}

pub fn nob_read_entire_file<P: AsRef<Path>>(path: P, sb: &mut NobStringBuilder) -> io::Result<()> {
    let mut file = File::open(path)?;
    file.read_to_end(&mut sb.items)?;
    Ok(())
}

pub fn nob_log(level: &str, message: &str, file: &str) {
    println!("[{}] {}: {}", level, message, file);
}

const RUSTLEX_ID: i32 = 1;
const RUSTLEX_INTLIT: i32 = 1;

pub fn is_unconcatable(token: i32) -> bool {
    token == RUSTLEX_ID || token == RUSTLEX_INTLIT
}

pub struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
    token: i32,
    where_firstchar: usize,
    where_lastchar: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Lexer {
            input,
            pos: 0,
            token: 0,
            where_firstchar: 0,
            where_lastchar: 0,
        }
    }

    pub fn get_token(&mut self) -> bool {
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }

        if self.pos >= self.input.len() {
            return false;
        }

        self.where_firstchar = self.pos;

        if self.input[self.pos].is_ascii_alphabetic() || self.input[self.pos] == b'_' {
            while self.pos < self.input.len()
                && (self.input[self.pos].is_ascii_alphanumeric() || self.input[self.pos] == b'_')
            {
                self.pos += 1;
            }
            self.token = RUSTLEX_ID;
        } else if self.input[self.pos].is_ascii_digit() {
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
            self.token = RUSTLEX_INTLIT;
        } else {
            self.token = self.input[self.pos] as i32;
            self.pos += 1;
        }

        self.where_lastchar = self.pos - 1;
        true
    }
}

pub fn format_tokens(output_path: &str, input_path: &str) -> bool {
    let mut sb = NobStringBuilder::new();

    if nob_read_entire_file(input_path, &mut sb).is_err() {
        return false;
    }

    let output = File::create(output_path);
    let mut output = match output {
        Ok(file) => BufWriter::new(file),
        Err(err) => {
            nob_log(
                "NOB_ERROR",
                &format!("Could not create file {}: {}", output_path, err),
                "",
            );
            return false;
        }
    };

    let mut lexer = Lexer::new(&sb.items);

    let mut x = 8;
    let mut prev_token = 0;

    while lexer.get_token() {
        let n = lexer.where_lastchar - lexer.where_firstchar + 1;
        if is_unconcatable(prev_token) && is_unconcatable(lexer.token) {
            write!(output, " ").unwrap();
            x += 1;
        }
        prev_token = lexer.token;
        write!(
            output,
            "{}",
            std::str::from_utf8(&sb.items[lexer.where_firstchar..=lexer.where_lastchar]).unwrap()
        )
        .unwrap();
        x += n;
        if x >= 88 {
            write!(output, "\n").unwrap();
            x = 0;
        }
    }
    if output.flush().is_err() {
        return false;
    }
    nob_log("NOB_INFO", "Generated", output_path);
    true
}

pub fn compile_quine_blob(output_path: &str, input_path: &str) -> bool {
    let mut sb = NobStringBuilder::new();

    if nob_read_entire_file(input_path, &mut sb).is_err() {
        return false;
    }

    let output = File::create(output_path);
    let mut output = match output {
        Ok(file) => BufWriter::new(file),
        Err(err) => {
            nob_log(
                "NOB_ERROR",
                &format!("Could not create file {}: {}", output_path, err),
                "",
            );
            return false;
        }
    };

    for i in 0..sb.count() {
        if sb.items[i] == b'?' {
            for j in 0..sb.count() {
                match sb.items[j] {
                    b'\n' => write!(output, "\\n\"\n\"").unwrap(),
                    b'\\' => write!(output, "\\\\").unwrap(),
                    b'"' => write!(output, "\\\"").unwrap(),
                    _ => write!(output, "{}", sb.items[j] as char).unwrap(),
                }
            }
        } else {
            write!(output, "{}", sb.items[i] as char).unwrap();
        }
    }

    if output.flush().is_err() {
        return false;
    }

    nob_log("NOB_INFO", "Generated", output_path);
    true
}

fn main() {
    if let Err(..) = fs::create_dir_all("./build/") {}
    if !format_tokens("./build/qlock-formatted.rs", "qlock.rs") {}
    if !compile_quine_blob("./build/qlock-blob.rs", "./build/qlock-formatted.rs") {}
    if !build_exe("./build/quine", "quine.rs") {}
    if !build_exe("./build/clock", "clock.rs") {}
    if !build_exe("./build/qlock", "./build/qlock-blob.rs") {}
}

fn build_exe(output_path: &str, input_path: &str) -> bool {
    let output = Command::new("rustc")
        .arg(input_path)
        .arg("-o")
        .arg(output_path)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                true
            } else {
                eprintln!("failed to compile {}: {:?}", input_path, output);
                false
            }
        }
        Err(..) => false,
    }
}
