use std::ops::Range;

use regex::Regex;

fn get_regex() -> Regex {
    Regex::new(r"\d+").unwrap()
}

fn get_maps<T: AsRef<str>>(lines: &[T]) -> Vec<Vec<(Range<u64>, u64)>> {
    let re = get_regex();
    let iter = lines.iter();
    let map_info = iter.as_slice();

    let mut maps = vec![];
    let mut current_map: Vec<(Range<u64>, u64)> = vec![];
    for line in map_info.iter() {
        let line = line.as_ref();
        if line.is_empty() { continue; }
        if line.contains("map") {
            maps.push(current_map);
            current_map = vec![];
            continue;
        }
        let info = &re.find_iter(&line).map(|m| String::from(m.as_str()).parse::<u64>().unwrap()).collect::<Vec<u64>>()[..];
        let (dest, source, length) = (info[0], info[1], info[2]);
        current_map.push(((source..source + length), dest));
    }
    // current_map.sort_by_key(|(r, _)| r.start);
    maps.push(current_map);
    maps
}

fn seed_to_location(s: &u64, maps: &Vec<Vec<(Range<u64>, u64)>>) -> u64 {
    let mut t = *s;
    maps.iter().filter(|m| !m.is_empty()).for_each(|m| {
        // println!("Using {:?}", m);
        // print!("Mapped {} to ", t);
        for (r, d) in m {
            if r.contains(&t) {
                let diff = t - r.start;
                t = d + diff;
                break;
            }
        }
        // println!("{}", t);
    });
    // println!("--------");
    t
}

fn solve_part1<T: AsRef<str>>(seeds: &Vec<u64>, lines: &[T]) -> u64 {
    let maps = get_maps(lines);

    // println!("{:?}", maps);

    seeds.iter().map(|s| {
        seed_to_location(s, &maps)
    }).min().unwrap_or(0)
}


fn solve_part2<T: AsRef<str>>(seeds: &[u64], lines: &[T]) -> u64 {
    // TODO: Reverse lookup implementieren
    let maps = get_maps(lines);
    (0..seeds.len()).step_by(2).map(|i|
        (seeds[i]..=seeds[i] + seeds[i + 1])
    )
        .inspect(|r| println!("{:?} with length of {}", r, r.end() - r.start()))
        .map(|r| r.map(|s| seed_to_location(&s, &maps)).min().unwrap())
        .min().unwrap()
}

fn get_seeds(lines: &Vec<String>) -> Vec<u64> {
    let re = get_regex();
    let seeds = re.find_iter(&lines[0].as_ref()).map(|m| String::from(m.as_str()).parse::<u64>().unwrap()).collect::<Vec<u64>>();
    seeds
}

fn main() {
    let lines = common::get_lines_from_file("day05/input.txt");
    let seeds = get_seeds(&lines);
    let result_part1 = solve_part1(&seeds, &lines[1..]);
    println!("Result part1: {}", result_part1);
    println!("Part1 240320250 is correct");


    let result_part2 = solve_part2(&seeds, &lines[1..]);
    println!("Result part2: {}", result_part2);
    println!("Part2 ------ is correct");
}


#[test]
fn test_part1() {
    let lines = common::get_lines_from_file("test_input.txt");
    let seeds = get_seeds(&lines);
    // let result_1 = solve_part1(&seeds, &lines[1..]);
    // assert_eq!(35, result_1);
    let result_2 = solve_part2(&seeds, &lines[1..]);
    assert_eq!(46, result_2);
}

#[test]
fn test_single() {
    let lines = common::get_lines_from_file("test_input.txt");
    let mut seeds = vec![];
    seeds.push(34);
    let result_1 = solve_part1(&seeds, &lines[1..]);
    assert_eq!(35, result_1);
}