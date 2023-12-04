use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug)]
struct Number {
    value: u32,
    string: String,
    index: usize,
    symbol_index: (usize, usize),
}

impl Number {
    fn of(string: String, index: usize) -> Self {
        Self { value: string.parse::<u32>().unwrap(), string, index, symbol_index: (usize::MAX, usize::MAX) }
    }

    fn symbol_index(&mut self, si: (usize, usize)) {
        self.symbol_index = si;
    }
}

fn get_numbers_from_line(line: &str) -> Vec<Number> {
    let mut result = vec![];
    let mut index: usize = usize::MAX;
    for (x, c) in line.chars().enumerate() {
        if c.is_numeric() {
            if index == usize::MAX {
                index = x;
            }
            if x + 1 == line.len() {
                result.push(Number::of(String::from(&line[index..=x]), index));
                index = usize::MAX;
            }
            continue;
        }
        if index != usize::MAX {
            result.push(Number::of(String::from(&line[index..x]), index));
            index = usize::MAX;
        }
    }
    result
}

fn has_symbol(range: Range<usize>, line: &str) -> Option<usize> {
    for i in range.into_iter() {
        let c = line.chars().nth(i).unwrap_or('.');
        if c != '.' && !c.is_numeric() {
            return Some(i);
        }
    }
    None
}

fn solve_part1<T: AsRef<str>>(lines: Vec<T>) -> (u32, Vec<Number>) {
    let mut result = (0u32, vec![]);
    lines.iter().enumerate().for_each(|(y, l)| {
        let line = l.as_ref();
        let numbers = get_numbers_from_line(line);
        for mut number in numbers {
            let x = number.index;
            let mut symbol_index = (usize::MAX, usize::MAX);
            let start = if x > 0 { x - 1 } else { 0 };
            let end_index = x + number.string.len() + 1;
            let end = if end_index < line.len() { end_index } else { line.len() };
            // check above
            if y > 0 {
                if let Some(i) = has_symbol(Range { start, end }, lines[y - 1].as_ref()) {
                    symbol_index = (i, y - 1);
                }
            }
            //check line
            if let Some(i) = has_symbol(Range { start, end }, line) {
                symbol_index = (i, y);
            }
            //check below
            if y + 1 < lines.len() {
                if let Some(i) = has_symbol(Range { start, end }, lines[y + 1].as_ref()) {
                    symbol_index = (i, y + 1);
                }
            }

            if symbol_index.0 < usize::MAX {
                number.symbol_index(symbol_index);
                result.0 += number.value;
                result.1.push(number);
            }
        }
    });
    result
}

fn solve_part2(numbers: &[Number]) -> u32 {
    let mut map = HashMap::new();
    numbers.iter().map(|n| (n.symbol_index, n.value)).for_each(|(si, v)| {
        map.entry(si).or_insert_with(Vec::new).push(v);
    });
    map.iter().filter_map(|(_si, values)| if values.len() > 1 {
        Some(values.iter().product::<u32>())
    } else { None }).sum::<u32>()
}

fn main() {
    let lines = common::get_lines_from_file("day03/input.txt");
    let non_empty_lines: Vec<_> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    let result_part1 = solve_part1(non_empty_lines);
    println!("Result part1: {}", result_part1.0);
    println!("Part1 521515 is correct");
    let result_part2 = solve_part2(&result_part1.1);
    println!("Result part2: {}", result_part2);
    println!("Part2 69527306 is correct");
}

#[test]
fn test_part1() {
    let content = String::from("\
467..114..\n\
...*......\n\
..35..633.\n\
......#...\n\
617*......\n\
.....+.58.\n\
..592.....\n\
......755.\n\
...$.*....\n\
.664.598..");
    let result = solve_part1(content.lines().collect());
    assert_eq!(4361, result.0);
    let result2 = solve_part2(&result.1);
    assert_eq!(467835, result2);
}