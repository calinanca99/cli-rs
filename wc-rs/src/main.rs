use clap::Parser;
use std::{fs::File, io::Read};

use wc_rs::{cli::Cli, file::*};

/*
    Replicate the wc behavior without:
    - total line count
    - looking at files
*/
fn main() {
    let mut cli = Cli::parse();

    let mut file_counts = Vec::with_capacity(cli.file.len());

    for file_path in &cli.file {
        let mut file = File::open(file_path).expect("File not found");

        let mut file_count = FileCount::new(file_path.to_string());
        file_count.dir = file.metadata().unwrap().is_dir();

        let mut s = String::new();

        if !file_count.dir {
            file.read_to_string(&mut s).expect("Cannot read file");
        }

        // Handle case when no flags are passed
        if !cli.lines && !cli.words && !cli.bytes {
            cli.lines = true;
            cli.words = true;
            cli.bytes = true;
        }

        if cli.lines {
            match file_count.dir {
                true => {
                    let file_lines = get_lines(&s);
                    file_count.lines = Some(file_lines);
                }
                false => {
                    file_count.lines = Some(0);
                }
            }
        }

        if cli.words {
            match file_count.dir {
                true => {
                    let file_words = get_words(&s);
                    file_count.words = Some(file_words);
                }
                false => {
                    file_count.words = Some(0);
                }
            }
        }

        if cli.bytes {
            match file_count.dir {
                true => {
                    let file_bytes = get_bytes(&s);
                    file_count.bytes = Some(file_bytes);
                }
                false => {
                    file_count.bytes = Some(0);
                }
            }
        }

        file_counts.push(file_count);
    }

    for file_count in &file_counts {
        let mut to_print = String::new();

        if file_count.lines.is_some() {
            to_print.push_str(format!("{} ", file_count.lines.unwrap()).as_str());
        }

        if file_count.words.is_some() {
            to_print.push_str(format!("{} ", file_count.words.unwrap()).as_str());
        }

        if file_count.bytes.is_some() {
            to_print.push_str(format!("{} ", file_count.bytes.unwrap()).as_str());
        }

        to_print.push_str(file_count.path.as_str());

        if file_count.dir {
            println! {"wc-rs: {}: Is a directory", file_count.path};
        }

        println!("{to_print}");
    }

    // dbg!(cli);
}
