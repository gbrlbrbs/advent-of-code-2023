use std::collections::HashSet;
use regex::Regex;


pub fn parse_line(line: &str) -> i32 {
    let colon_split: Vec<&str> = line.splitn(2, ": ").collect();
    let num_regex = Regex::new(r"\d+").unwrap();
    let game: Vec<&str> = colon_split[1]
        .split("| ")
        .collect();
    let winning_numbers: HashSet<i32> = num_regex
        .find_iter(&game[0])
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect();
    let drawn_numbers: HashSet<i32> = num_regex
        .find_iter(&game[1])
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect();
    let intersect: Vec<&i32> = winning_numbers.intersection(&drawn_numbers).collect();
    if intersect.len() > 0 {
        return i32::pow(2, (intersect.len() - 1) as u32);
    }
    0
}