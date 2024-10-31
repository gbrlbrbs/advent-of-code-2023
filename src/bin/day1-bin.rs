use aoc::{day1::{match_text, build_aho_corasick}, read_lines};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    if let Ok(lines) = read_lines(file) {
        let ac = build_aho_corasick();
        let mut final_value = 0;
        for line in lines.flatten() {
            final_value += match_text(&line, &ac).unwrap();
        }
        println!("Final value: {}", final_value);
    }
}