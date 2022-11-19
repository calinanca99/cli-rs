use std::{fs::File, io::Read};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "wc-rs")]
#[command(version = "1.0")]
#[command(about = "wc-rs: wc (partially) ported to Rust")]

struct Cli {
    #[arg(help = "FILEs to read")]
    file: Vec<String>,

    #[arg(short = 'c', long = "bytes", help = "Print the byte counts")]
    bytes: bool,

    #[arg(short = 'm', long = "chars", help = "Print the character counts")]
    chars: bool,

    #[arg(short = 'l', long = "lines", help = "Print the newline counts")]
    lines: bool,

    #[arg(short = 'w', long = "words", help = "Print the word counts")]
    words: bool,
}

fn get_bytes(path: &str) -> usize {
    let mut file = File::open(path).expect("File not found");
    let mut s = String::new();

    let bytes = file.read_to_string(&mut s).expect("Cannot read from file");

    bytes
}

fn main() {
    let cli = Cli::parse();

    let lines = 0;
    let words = 0;
    let mut bytes = 0;

    for file_path in &cli.file {
        let file = File::open(file_path).expect("File not found");

        let file_metadata = file.metadata().unwrap();

        if file_metadata.is_dir() {
            println!("wc-rs: {file_path}: Is a directory");
            println!("0 0 0 {file_path}");
        } else {
            let file_lines = 0;
            let file_words = 0;
            let file_bytes = get_bytes(file_path);

            bytes += get_bytes(file_path);

            println!("{file_lines} {file_words} {file_bytes} {file_path}");
        }
    }

    if cli.file.len() > 1 {
        println!("{lines} {words} {bytes} total");
    }

    // dbg!(cli);
}
