fn next_number(numbers: &Vec<i32>) -> i32 {
    let left = &numbers[..numbers.len() - 1];
    let right = &numbers[1..];
    let zip = left.iter().zip(right).map(|(a, b)| b - a).collect::<Vec<_>>();

    if zip.windows(2).any(|w| w[0] != w[1]) {
        let last = numbers.last().unwrap();
        let next = next_number(&zip);
        return next + last;
    }
    let r = zip.last().unwrap() + numbers.last().unwrap();
    r
}

fn prev_number(numbers: &Vec<i32>) -> i32 {
    let left = &numbers[..numbers.len() - 1];
    let right = &numbers[1..];
    let zip = left.iter().zip(right).map(|(a, b)| b - a).collect::<Vec<_>>();

    if zip.windows(2).any(|w| w[0] != w[1]) {
        let first = numbers.first().unwrap();
        let next = crate::prev_number(&zip);
        return first - next;
    }
    let r = numbers.first().unwrap() - zip.first().unwrap();
    r
}

fn solve_part<T: AsRef<str>>(lines: &[T], f : &dyn Fn(&Vec<i32>) -> i32) -> i32 {
    lines.iter()
        .map(|l| l.as_ref().split(' ').collect::<Vec<_>>())
        .map(|v| v.iter().map(|n| String::from(*n).parse::<i32>().unwrap()).collect::<Vec<_>>())
        .map(|v| f(&v))
        .sum()
}

fn solve_part1<T: AsRef<str>>(lines: &[T]) -> i32 {
    solve_part(lines, &next_number)
}


fn solve_part2<T: AsRef<str>>(lines: &[T]) -> i32 {
    solve_part(lines, &prev_number)
}


fn main() {
    let lines = common::get_lines_from_file("day09/input.txt");
    let result_part1 = solve_part1(&lines);
    println!("Result part1: {}", result_part1);
    println!("Part1 253638586 is correct");

    let result_part2 = solve_part2(&lines);
    println!("Result part2: {}", result_part2);
    println!("Part2 ------ is correct");
}


#[test]
fn test_part1() {
    let content = String::from("\
0 3 6 9 12 15\n\
1 3 6 10 15 21\n\
10 13 16 21 30 45\
");
    let result_1 = solve_part1(&content.lines().collect::<Vec<_>>());
    assert_eq!(114, result_1);
}

#[test]
fn test_part2() {
    let content = String::from("\
0 3 6 9 12 15\n\
1 3 6 10 15 21\n\
10 13 16 21 30 45\
");
    let result_1 = solve_part2(&content.lines().collect::<Vec<_>>());
    assert_eq!(2, result_1);
}