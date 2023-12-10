use std::collections::BTreeMap;

use regex::Regex;

fn solve_part1<T: AsRef<str>>(lines: &[T]) -> u32 {
    lines.iter().map(|l| l.as_ref().split_once("| ").unwrap()).map(|(l, r)| {
        let mut result: f32 = 0.5;
        r.split(' ').for_each(|n| if !n.is_empty() && l.contains(format!(" {} ", n).as_str()) {
            result *= 2.;
        });
        result as u32
    }).sum()
}

fn solve_part2<T: AsRef<str>>(lines: &[T]) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let mut map = BTreeMap::new();
    lines.iter().map(|l| l.as_ref().split_once("| ").unwrap()).map(|(l, r)| {
        let card_id = String::from(re.captures(l).unwrap().get(0).unwrap().as_str()).parse::<u32>().unwrap();
        let instances = map.get(&card_id).unwrap_or(&0) + 1;
        let matching_numbers: u32 = r.split(' ').map(|n| if !n.is_empty() && l.contains(format!(" {} ", n).as_str()) { 1 } else { 0 }).sum();
        for i in card_id + 1..=card_id + matching_numbers {
            match map.get(&i) {
                None => { map.insert(i, instances); }
                Some(count) => { map.insert(i, count + instances); }
            }
        }
        match map.get(&card_id) {
            None => { 1 }
            Some(v) => { v + 1 }
        }
    }).sum()
}

fn main() {
    let lines = common::get_lines_from_file("day04/input.txt");
    let result_part1 = solve_part1(&lines);
    println!("Result part1: {}", result_part1);
    println!("Part1 26218 is correct");
    let result_part2 = solve_part2(&lines);
    println!("Result part2: {}", result_part2);
    println!("Part2 9997537 is correct");
}

#[test]
fn test_part1() {
    let content = String::from("\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
    // .collect::<Vec<&str>>()[..] is needed for the &[T] parameter, &Vec<T> involves a new object
    let result_1 = solve_part1(&content.lines().collect::<Vec<&str>>()[..]);
    assert_eq!(13, result_1);
    let result_2 = solve_part2(&content.lines().collect::<Vec<&str>>()[..]);
    assert_eq!(30, result_2);
}