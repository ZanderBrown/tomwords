use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate regex;

use regex::Regex;

fn main() {
    let source = File::open("words/words.txt").expect("Failed to open words file");
    let invalid = Regex::new(r"[gkmqvwxzio\W]").expect("Invalid expression");
    let current = BufReader::new(source)
        .lines()
        .map(|l| l.expect("Failed to read word"))
        .filter(|l| !invalid.is_match(&l))
        .max_by(|a, b| a.len().cmp(&b.len()));
    if let Some(word) = current {
        println!("Longest Word: {} ({})", word, word.len());
    } else {
        println!("No word found");
    }
}
