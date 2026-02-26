use std::io::{self, BufReader, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(io::stdin().lock());
    reader.read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();

    let mut out = BufWriter::new(io::stdout().lock());
    writeln!(out, "{}", a + b).unwrap();
}
