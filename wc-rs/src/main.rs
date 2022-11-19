use std::fs::File;

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

fn main() {
    let cli = Cli::parse();

    for file_path in &cli.file {
        let file = File::open(file_path).expect("File not found");

        let file_metadata = file.metadata().unwrap();

        if file_metadata.is_dir() {
            println!("wc-rs: {file_path}: Is a directory");
            println!("\t0\t0\t0 {file_path}");
        }
    }
    println!("\t0\t0\t0 total");

    // dbg!(cli);
}
