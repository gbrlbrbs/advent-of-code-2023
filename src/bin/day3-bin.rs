
use aoc::{day3::{sum_of_gears, sum_of_parts}, read_lines};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    if let Ok(lines) = read_lines(file) {
        let lines_vec: Vec<String> = lines.flatten().collect();
        let sum_of_parts = sum_of_parts(&lines_vec);
        println!("Sum of parts: {}", sum_of_parts);
        let sum_of_gears = sum_of_gears(&lines_vec);
        println!("Sum of gears: {}", sum_of_gears);
    }
}