use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    println!(
        "{}",
        Regex::new(r"[gkmqvwxzio]")
            .map(|invalid| {
                File::open("words/words.txt")
                    .map(|source| {
                        BufReader::new(source)
                            .lines()
                            .map(|l| l.expect("Failed to read word"))
                            .filter(|l| !invalid.is_match(&l))
                            .max_by(|a, b| a.len().cmp(&b.len()))
                            .map(|word| format!("Longest Word: {} ({})", word, word.len()))
                            .unwrap_or(String::from("No word found"))
                    })
                    .unwrap_or(String::from("Failed to open words file"))
            })
            .unwrap_or(String::from("Invalid expression"))
    );
}
