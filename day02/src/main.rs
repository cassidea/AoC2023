use std::cmp::max;
use csv::Error;

struct Cubes {
    red: i32,
    blue: i32,
    green: i32,
}

impl Cubes {
    fn from_pull(pull: &str) -> Self {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        pull.trim().split(", ").for_each(|c| {
            let tokens = c.split(' ').collect::<Vec<&str>>();
            match tokens[1] {
                "red" => red = String::from(tokens[0]).parse::<i32>().unwrap(),
                "blue" => blue = String::from(tokens[0]).parse::<i32>().unwrap(),
                "green" => green = String::from(tokens[0]).parse::<i32>().unwrap(),
                _ => panic!("Unknown color {}", tokens[1])
            }
        });
        Self { red, blue, green }
    }

    fn from_pulls(pulls: &str) -> Result<Vec<Self>, Error> {
        let mut reader = csv::ReaderBuilder::new().has_headers(false).delimiter(b';').from_reader(pulls.as_bytes());
        let record = reader.records().next().unwrap()?;
        return Ok(record.iter().map(Cubes::from_pull).collect());
    }
}

fn get_cubes(line: &str) -> (i32, Vec<Cubes>) {
    let (game, pulls) = line.split_once(": ").unwrap();
    let (_, id) = game.split_once(' ').unwrap();
    (String::from(id).parse::<i32>().unwrap(), Cubes::from_pulls(pulls).unwrap())
}

fn solve_part1<T: AsRef<str>>(lines: impl Iterator<Item=T>) -> i32 {
    let mut result = 0;
    lines.for_each(|line| {
        let (id, cubes) = get_cubes(line.as_ref());
        let mut valid = true;
        for c in cubes {
            if c.blue > CUBES.blue || c.red > CUBES.red || c.green > CUBES.green {
                valid = false;
            }
        }
        if valid {
            result += id;
        }
    });
    result
}

fn solve_part2<T: AsRef<str>>(lines: impl Iterator<Item=T>) -> i32 {
    let mut result = 0;
    lines.for_each(|line| {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        let (_, cubes) = get_cubes(line.as_ref());
        for c in cubes {
            red = max(c.red, red);
            blue = max(c.blue, blue);
            green = max(c.green, green);
        }
        result += red * blue * green;
    });
    result
}

const CUBES: Cubes = Cubes { red: 12, green: 13, blue: 14 };
fn main() {
    let lines = common::get_lines_from_file("day02/input.txt");
    let non_empty_lines: Vec<_> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    let result_part1 = solve_part1(non_empty_lines.iter());
    println!("Result part1: {}", result_part1);
    println!("Part1 2369 is correct");
    let result_part2 = solve_part2(non_empty_lines.iter());
    println!("Result part2: {}", result_part2);
    println!("Part2 66363 is correct");
}

#[test]
fn test_part1() {
    let content = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    let result = solve_part1(content.lines());
    assert_eq!(8, result);
}

#[test]
fn test_part2() {
    let content = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    let result = solve_part2(content.lines());
    assert_eq!(2286, result);
}