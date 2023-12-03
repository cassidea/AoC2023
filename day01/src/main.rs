use std::collections::HashMap;

fn solve_part1(content: &str) -> i32 {
    content.lines().filter(|l| !l.is_empty()).map(|l| {
        let first = l.chars().find(|c| c.is_numeric()).unwrap();
        let last = l.chars().rfind(|c| c.is_numeric()).unwrap_or('\0');

        format!("{}{}", first, last).parse::<i32>().unwrap()
    }).sum()
}

fn solve_part2(content: &str) -> i32 {
    let mut result = String::from(content);
    let mut map = HashMap::new();
    map.insert("one", "1");
    map.insert("two", "2");
    map.insert("three", "3");
    map.insert("four", "4");
    map.insert("five", "5");
    map.insert("six", "6");
    map.insert("seven", "7");
    map.insert("eight", "8");
    map.insert("nine", "9");
    map.insert("zero", "0");

    for s in map.keys() {
        let indices = content.match_indices(s);
        indices.for_each(|(i, _)| result.replace_range(i..i + 1, map.get(s).unwrap()))
    }
    solve_part1(result.as_str())
}

fn main() {
    let content = common::get_lines_as_string("day01/input.txt");
    let result_part1 = solve_part1(&content);
    println!("Result part1: {}", result_part1);
    println!("Part1 54940 is correct");
    let result_part2 = solve_part2(&content);
    println!("Result part2: {}", result_part2);
    println!("Part2 54940 is correct");
}

#[test]
fn test_part1() {
    let content = String::from("1abc2\n\
pqr3stu8vwx\n\
a1b2c3d4e5f\n\
treb7uchet\n\n");
    let result = solve_part1(&content);
    assert_eq!(142, result);
}

#[test]
fn test_part2() {
    let content = String::from("two1nine\n\
eightwothree\n\
abcone2threexyz\n\
xtwone3four\n\
4nineeightseven2\n\
zoneight234\n\
7pqrstsixteen\n\n");
    let result = solve_part2(&content);
    assert_eq!(281, result);
}