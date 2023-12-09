struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn from_lines<T: AsRef<str>>(lines: &[T]) -> Vec<Self> {
        let times = Self::numbers_from_line(lines.get(0).unwrap().as_ref());
        let records = Self::numbers_from_line(lines.get(1).unwrap().as_ref());
        times.iter().zip(&records).map(|(t, d)| Self { time: *t, record: *d }).collect::<Vec<Self>>()
    }

    fn numbers_from_line(line: &str) -> Vec<u64> {
        line.split(' ').filter_map(|token| match String::from(token).parse::<u64>() {
            Ok(n) => { Some(n) }
            Err(_) => { None }
        }).collect::<Vec<u64>>()
    }

    fn calc_dist(&self, ms: u64) -> u64 {
        (self.time - ms) * ms
    }
    fn records(&self) -> u64 {
        let mut ms = self.time / 2;
        let max_dist = self.calc_dist(ms);
        let mut dist = max_dist;
        while dist > self.record {
            ms -= 1;
            dist = self.calc_dist(ms);
        }
        ms += 1; // last run is shorter than record
        self.time - 2 * ms + 1
    }
}

fn solve<T: AsRef<str>>(lines: &[T]) -> u64 {
    let races = Race::from_lines(lines);
    races.iter().map(|r| r.records()).product()
}

fn main() {
    let lines = common::get_lines_from_file("day06/input.txt");
    let result_part1 = solve(&lines);
    println!("Result part1: {}", result_part1);
    println!("Part1 503424 is correct");

    let lines = lines.iter().map(|l| l.replace(' ', "").replace(':', ": ")).collect::<Vec<_>>();
    println!("{:?}", lines);
    let result_part2 = solve(&lines);
    println!("Result part2: {}", result_part2);
    println!("Part2 32607562 is correct");
}

#[test]
fn test_part1() {
    let content = String::from("Time:      7  15   30\nDistance:  9  40  200");
    let result = solve(&content.lines().collect::<Vec<_>>());
    assert_eq!(288, result);
}

#[test]
fn test_part2() {
    let content = String::from("Time: 71530\nDistance:  940200");
    let result = solve(&content.lines().collect::<Vec<_>>());
    assert_eq!(71503, result);
}
