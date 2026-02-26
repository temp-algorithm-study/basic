use std::io::{self, BufWriter, Write};

fn main() {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    writeln!(out, "Hello World!").unwrap();
}
