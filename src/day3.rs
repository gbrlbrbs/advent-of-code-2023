use regex::{Regex, Match};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position {x, y}
    }

    pub fn adjacent_vertical(&self, other: &Position) -> bool {
        (self.y >= other.y - 1) && (self.y <= other.y + 1)
    }

    pub fn adjacent_horizontal(&self, other: &Position) -> bool {
        (self.x >= other.x - 1) && (self.x <= other.x + 1)
    }

    pub fn is_adjacent_to(&self, other: &Position) -> bool {
        self.adjacent_vertical(other) && self.adjacent_horizontal(other)
    }

    pub fn from_match_vec(m: &Match, y: i32) -> Vec<Position> {
        m.range().into_iter().filter(|x| *x != m.end()).map(|x| Position{x: x as i32, y}).collect()
    }

    pub fn from_match_single(m: &Match, y: i32) -> Position {
        Position::from_match_vec(m, y)[0]
    }

    pub fn create_adjacents(&self) -> Vec<Position> {
        let mut adj: Vec<Position> = vec![];
        for x in self.x-1..self.x+2 {
            for y in self.y-1..self.y+2 {
                adj.push(Position{x, y});
            }
        }
        adj
    }
}

pub struct Symbol {
    pub position: Position,
    pub character: String
}

#[derive(Debug, Clone, Default)]
pub struct Part {
    pub positions: Vec<Position>,
    pub number: i32
}

#[derive(Debug, Clone, Default)]
pub struct Gear {
    parts: Vec<Part>,
    pub position: Position
}

impl Gear {
    pub fn new(m: &Match, y: i32) -> Self {
        Gear {
            position: Position::from_match_single(m, y),
            parts: vec![]
        }
    }

    pub fn is_adjacent_to(&self, part: &Part) -> bool {
        let adjacents = self.position.create_adjacents();
        adjacents.iter().any(|p| part.any_position_equal(p))
    }

    pub fn add_part(&mut self, part: Part) {
        self.parts.push(part);
    }

    pub fn is_valid_gear(&self) -> bool {
        self.parts.len() == 2
    }

    pub fn gear_ratio(&self) -> Option<i32> {
        if self.is_valid_gear() {
            return Some(self.parts[0].number * self.parts[1].number)
        }
        None
    }
}

impl Part {
    pub fn from_match(m: &Match, y: i32) -> Self {
        Part{ 
            positions: Position::from_match_vec(&m, y), 
            number: m.as_str().parse::<i32>().unwrap() 
        }
    }

    pub fn any_position_equal(&self, p: &Position) -> bool {
        self.positions.iter().any(|x| p == x)
    }
}

pub fn sum_of_parts(lines: &Vec<String>) -> i32
{
    let mut part_matches: Vec<Part> = vec![];
    let mut symbol_adjacents: Vec<Position> = vec![];
    let number_regex = Regex::new(r"\d+").unwrap();
    let symbol_regex = Regex::new(r"[^\d.]").unwrap();
    for (y, line) in lines.iter().enumerate() {
        let parts_in_line: Vec<Part> = number_regex
            .find_iter(&line)
            .map(|x| Part::from_match(&x, y as i32))
            .collect();
        let symb_match_in_line= symbol_regex.find_iter(&line);
        for m in symb_match_in_line {
            let p = Position::from_match_single(&m, y as i32);
            symbol_adjacents.extend(p.create_adjacents());
        }
        part_matches.extend(parts_in_line);
    }

    let mut sum_of_part_numbers = 0;
    for part in part_matches.iter() {
        if symbol_adjacents
            .iter()
            .any(|x| part.any_position_equal(x)) {
                sum_of_part_numbers += part.number;
        }
    }
    sum_of_part_numbers
}

pub fn sum_of_gears(lines: &Vec<String>) -> i32 {
    let number_regex = Regex::new(r"\d+").unwrap();
    let gear_regex = Regex::new(r"\*").unwrap();
    let mut gears: Vec<Gear> = vec![];
    let mut parts: Vec<Part> = vec![];
    for (y, line) in lines.iter().enumerate() {
        let parts_in_line: Vec<Part> = number_regex
            .find_iter(&line)
            .map(|x| Part::from_match(&x, y as i32))
            .collect();
        parts.extend(parts_in_line);
        let gears_in_line: Vec<Gear> = gear_regex
            .find_iter(&line)
            .map(|x| Gear::new(&x, y as i32))
            .collect();
        gears.extend(gears_in_line);
    }
    let mut sum_of_gears = 0;
    for gear in gears.iter_mut() {
        let adj_parts: Vec<&Part> = parts
            .iter()
            .filter(|x| gear.is_adjacent_to(x))
            .collect();
        for part in adj_parts.into_iter() {
            gear.add_part(part.clone())
        }
        if let Some(g) = gear.gear_ratio() {
            sum_of_gears += g;
        }
    }
    sum_of_gears
}
