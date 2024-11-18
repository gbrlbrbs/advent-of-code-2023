use std::env;
use aoc::{day4::parse_line, read_lines};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    if let Ok(lines) = read_lines(file) {
        
        let mut sum_points = 0;
        for line in lines.flatten() {
            sum_points += parse_line(&line); 
        }
        println!("sum of points: {}", sum_points);
    }
}