// let given_string = String::from("This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.");

use std::collections::BTreeMap;
use std::io;
use std::io::BufRead;

fn main() {
    let mut counts: BTreeMap<String, isize> = BTreeMap::new();
    let stdin = io::stdin();
    for line_result in stdin.lock().lines() {
        match line_result {
            Ok(line) => {
                let lowercase_line = line.to_lowercase();
                let words = lowercase_line
                    .split(|c: char| !(c.is_alphabetic() || c == '\''))
                    .filter(|s| !s.is_empty());
                for word in words {
                    *(counts.entry(word.to_string()).or_insert(0)) += 1;
                }
            }
            Err(e) => {
                panic!("Error parsing stdin: {:?}", e);
            }
        }
    }
    for (key, value) in counts.iter() {
        println!("{} {}", key, value);
    }
}
