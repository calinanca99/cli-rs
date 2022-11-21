use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "wc-rs")]
#[command(version = "1.0")]
#[command(about = "wc-rs: wc (partially) ported to Rust")]
pub struct Cli {
    #[arg(help = "FILEs to read")]
    pub file: Vec<String>,

    #[arg(short = 'c', long = "bytes", help = "Print the byte counts")]
    pub bytes: bool,

    #[arg(short = 'm', long = "chars", help = "Print the character counts")]
    pub chars: bool,

    #[arg(short = 'l', long = "lines", help = "Print the newline counts")]
    pub lines: bool,

    #[arg(short = 'w', long = "words", help = "Print the word counts")]
    pub words: bool,
}
