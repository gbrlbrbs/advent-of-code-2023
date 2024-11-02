use aoc::{day2::{check_game, find_minimum_set, CubeSet}, read_lines};
use std::env; 

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    if let Ok(lines) = read_lines(file) {
        let cs = CubeSet {
            reds: 12,
            greens: 13,
            blues: 14
        };
        let mut id_sum = 0;
        let mut sum_powers = 0;
        for line in lines.flatten() {
            if let Some(id) = check_game(&line, &cs) {
                id_sum += id;
            }
            sum_powers += find_minimum_set(&line);
        }
        println!("sum of ids: {}", id_sum);
        println!("sum of powers: {}", sum_powers);
    }
}