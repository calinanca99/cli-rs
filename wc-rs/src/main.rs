use clap::Parser;
use std::{fs::File, io::Read};

use wc_rs::{cli::Cli, file::*};

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
            set_count(&mut file_count, &s, EnumCount::Lines);
        }

        if cli.words {
            set_count(&mut file_count, &s, EnumCount::Words);
        }

        if cli.bytes {
            set_count(&mut file_count, &s, EnumCount::Bytes);
        }

        file_counts.push(file_count);
    }

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;

    for file_count in &file_counts {
        let mut to_print = String::new();

        if file_count.lines.is_some() {
            let lines = file_count.lines.unwrap();
            total_lines += lines;
            add_to_print(&mut to_print, lines);
        }

        if file_count.words.is_some() {
            let words = file_count.words.unwrap();
            total_words += words;
            add_to_print(&mut to_print, words);
        }

        if file_count.bytes.is_some() {
            let bytes = file_count.bytes.unwrap();
            total_bytes += bytes;
            add_to_print(&mut to_print, bytes);
        }

        to_print.push_str(file_count.path.as_str());

        if file_count.dir {
            println! {"wc-rs: {}: Is a directory", file_count.path};
        }

        println!("{to_print}");
    }

    if cli.file.len() > 1 {
        let mut to_print_total = String::new();

        if cli.lines {
            add_to_print(&mut to_print_total, total_lines);
        }

        if cli.words {
            add_to_print(&mut to_print_total, total_words);
        }

        if cli.bytes {
            add_to_print(&mut to_print_total, total_bytes);
        }

        to_print_total.push_str("total");

        println!("{to_print_total}");
    }
}

fn add_to_print(to_print: &mut String, count: usize) {
    to_print.push_str(format!("{} ", count).as_str())
}
