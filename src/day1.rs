// aho-corasick algorithm
use aho_corasick::{AhoCorasick, Match};
use std::collections::HashMap;

pub fn build_aho_corasick() -> AhoCorasick {
    let patterns = &["1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven", "8", "eight", "9", "nine"];
    AhoCorasick::builder()
        .build(patterns)
        .unwrap()
}

pub fn match_text(text: &str, ac: &AhoCorasick) -> Result<i32, &'static str>
{
    let matches: Vec<Match> = ac.find_overlapping_iter(text).collect();
    let first = match matches.first() {
        Some(val) => parse_val(&text[val.start()..val.end()]),
        None => return Err("first not found")
    };
    let last = match matches.last() {
        Some(val) => parse_val(&text[val.start()..val.end()]),
        None => return Err("last not found")
    };
    Ok(10 * first + last)
}

fn parse_val(val: &str) -> i32
{
    let is_numeric = val.parse::<i32>().is_ok();
    if is_numeric {
        return val.parse::<i32>().unwrap();
    }
    let numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    numbers[val]
}