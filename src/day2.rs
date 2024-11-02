use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy)]
pub struct CubeSet {
    pub blues: i32,
    pub greens: i32,
    pub reds: i32
}

impl FromStr for CubeSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubeset = CubeSet::default();

        for part in s.split(", ") {
            let mut words = part.split_whitespace();
            if let Some(number_str) = words.next() {
                let number = number_str.parse::<i32>()
                    .map_err(|_| format!("failed to parse number!"))?;
                if let Some(color) = words.next() {
                    match color {
                        "blue" => cubeset.blues = number,
                        "green" => cubeset.greens = number,
                        "red" => cubeset.reds = number,
                        _ => return Err(format!("unknown color: {}", color))
                    }
                }  
            }
        }

        Ok(cubeset)
    }
}

pub fn parse_line(line: &str) -> (i32, Vec<CubeSet>)
{
    // split on colon, get id
    let colon_split: Vec<&str> = line.splitn(2, ": ").collect();
    let game_id = colon_split[0]
        .splitn(2, " ")
        .filter(|x| x.parse::<i32>().is_ok())
        .next()
        .map(|x| x.parse::<i32>().unwrap())
        .unwrap();
        
    // get rest, strip whitespace at start and end and split on semicolon
    let game: Vec<CubeSet> = colon_split[1]
        .split("; ")
        .map(|x| CubeSet::from_str(x).unwrap())
        .collect();

    (game_id, game)
}

pub fn check_game(line: &str, set_to_check: &CubeSet) -> Option<i32>
{
    let (game_id, game) = parse_line(line);
    let all_sets_valid = game
        .into_iter()
        .map(|x| check_set(&x, set_to_check))
        .reduce(|acc, el| acc && el)
        .unwrap();
    
    if all_sets_valid { Some(game_id) } else { None }
}

fn check_set(s: &CubeSet, cmp: &CubeSet) -> bool
{
    (s.reds <= cmp.reds) && (s.greens <= cmp.greens) && (s.blues <= cmp.blues)
}

pub fn find_minimum_set(line: &str) -> i32 {
    let (_, game) = parse_line(line);
    // need to get the max of each for the minimum set
    let red_cubes = max_by_field!(game, reds);
    let green_cubes = max_by_field!(game, greens);
    let blue_cubes = max_by_field!(game, blues);
    red_cubes * green_cubes * blue_cubes
}