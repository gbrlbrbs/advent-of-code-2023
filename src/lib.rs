#[macro_use]
mod macros;

pub mod day1;
pub mod day2;
pub mod day3;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}