use std::collections::HashMap;

use crate::Type::{Five, Four, FullHouse, HighCard, Pair, Three, TwoPair};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Type {
    Five = 7,
    Four = 6,
    FullHouse = 5,
    Three = 4,
    TwoPair = 3,
    Pair = 2,
    HighCard = 1,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ass = 14,
}

impl Card {
    fn from_card(c: &char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ass,
            &_ => panic!("Unknown card given {:?}", c)
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: Type,
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn from_line(line: &str) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();
        let hand_type = Hand::get_type(cards);
        let cards = cards.chars().map(|c| Card::from_card(&c)).collect::<Vec<_>>();
        Self { cards, bid: String::from(bid).parse::<u32>().unwrap(), hand_type }
    }

    fn get_type(cards: &str) -> Type {
        let c_cards = cards.chars();
        let mut map: HashMap<char, u32> = HashMap::new();
        c_cards.for_each(|c| { map.entry(c).and_modify(|counter| *counter += 1).or_insert(1); });

        match map.len() {
            5 => HighCard,
            4 => Pair,
            3 => if map.values().any(|v| *v == 2) { TwoPair } else { Three },
            2 => if map.values().any(|v| *v == 2) { FullHouse } else { Four },
            1 => Five,
            _ => panic!("Unexpected card patter: {:?}", cards)
        }
    }
}

fn solve_part1<T: AsRef<str>>(lines: &[T]) -> u32 {
    let mut hands = lines.iter().map(|l| Hand::from_line(l.as_ref())).collect::<Vec<_>>();
    hands.sort();
    hands.iter().enumerate().map(|(i, h)| (i + 1) as u32 * h.bid).sum()
}


fn solve_part2<T: AsRef<str>>(lines: &[T]) -> u32 {
    0
}


fn main() {
    let lines = common::get_lines_from_file("day07/input.txt");
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
32T3K 765\n\
T55J5 684\n\
KK677 28\n\
KTJJT 220\n\
QQQJA 483");
    let result_1 = solve_part1(&content.lines().collect::<Vec<_>>());
    assert_eq!(6440, result_1);
}

#[test]
fn test_type() {
    assert_eq!(Five, Hand::from_line("AAAAA 0").hand_type);
    assert_eq!(Four, Hand::from_line("KAAAA 0").hand_type);
    assert_eq!(Four, Hand::from_line("AAAAK 0").hand_type);
    assert_eq!(FullHouse, Hand::from_line("AAAKK 0").hand_type);
    assert_eq!(FullHouse, Hand::from_line("AAKAK 0").hand_type);
    assert_eq!(FullHouse, Hand::from_line("KAKAK 0").hand_type);
    assert_eq!(Three, Hand::from_line("KAQAA 0").hand_type);
    assert_eq!(Three, Hand::from_line("KAQAA 0").hand_type);
    assert_eq!(Three, Hand::from_line("AKJAA 0").hand_type);
    assert_eq!(TwoPair, Hand::from_line("KKJAA 0").hand_type);
    assert_eq!(TwoPair, Hand::from_line("KAJKA 0").hand_type);
    assert_eq!(TwoPair, Hand::from_line("KJKAA 0").hand_type);
    assert_eq!(Pair, Hand::from_line("KAJQA 0").hand_type);
    assert_eq!(Pair, Hand::from_line("KJQAA 0").hand_type);
    assert_eq!(Pair, Hand::from_line("AQJKA 0").hand_type);
    assert_eq!(HighCard, Hand::from_line("A2345 0").hand_type);
    assert_eq!(HighCard, Hand::from_line("2345A 0").hand_type);
    assert_eq!(HighCard, Hand::from_line("23A45 0").hand_type);
    assert!(Five > Pair);
}

#[test]
fn test_ordering() {
    let mut vec = vec![
        Hand::from_line("2345A 0"),
        Hand::from_line("AAQQ5 15"),
        Hand::from_line("22QQ5 10"),
        Hand::from_line("AAAAA 30"),
        Hand::from_line("KKKKK 20"),
    ];
    vec.sort();
    assert_eq!(vec![0, 10, 15, 20, 30], vec.iter().map(|h| h.bid).collect::<Vec<_>>());
}