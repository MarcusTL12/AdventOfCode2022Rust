use std::{fs::read_to_string, mem::replace};

use num::integer::lcm;
use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    fn call(&self, x: u64) -> u64 {
        match self {
            Self::Add(y) => x + y,
            Self::Mul(y) => x * y,
            Self::Square => x * x,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divtest: u64,
    victims: [usize; 2],
}

fn parse_input(filename: &str) -> Vec<Monkey> {
    let reg = Regex::new(
        r#"Monkey \d+:
 +Starting items: ((?:\d+,? ?)+)
 +Operation: new = old (.) (\w+)
 +Test: divisible by (\d+)
 +If true: throw to monkey (\d+)
 +If false: throw to monkey (\d+)"#,
    )
    .unwrap();

    let inp = read_to_string(filename).unwrap();

    reg.captures_iter(&inp)
        .map(|c| Monkey {
            items: c[1].split(", ").map(|s| s.parse().unwrap()).collect(),
            operation: match (&c[2], &c[3]) {
                ("*", "old") => Operation::Square,
                ("+", s) => Operation::Add(s.parse().unwrap()),
                ("*", s) => Operation::Mul(s.parse().unwrap()),
                _ => panic!("Weird input {} {}", &c[2], &c[3]),
            },
            divtest: c[4].parse().unwrap(),
            victims: [&c[5], &c[6]].map(|s| s.parse().unwrap()),
        })
        .collect()
}

fn part1() {
    let mut monkeys = parse_input("input/day11/input");

    let mut inspections = vec![0; monkeys.len()];

    let mut throws = Vec::new();

    let mut items_buf = Vec::new();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            inspections[i] += monkeys[i].items.len();

            items_buf = replace(&mut monkeys[i].items, items_buf);
            let m = &mut monkeys[i];

            for item in &mut items_buf {
                *item = m.operation.call(*item) / 3;

                throws.push(m.victims[(*item % m.divtest != 0) as usize]);
            }

            for (&item, &victim) in items_buf.iter().zip(&throws) {
                monkeys[victim].items.push(item);
            }

            throws.clear();
            items_buf.clear();
        }
    }

    inspections.sort();

    let ans = inspections.pop().unwrap() * inspections.pop().unwrap();

    println!("{ans}");
}

fn part2() {
    let mut monkeys = parse_input("input/day11/input");

    let mut inspections = vec![0; monkeys.len()];

    let mut throws = Vec::new();

    let mut items_buf = Vec::new();

    let max_div = monkeys.iter().fold(1, |x, m| lcm(x, m.divtest));

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            inspections[i] += monkeys[i].items.len();

            items_buf = replace(&mut monkeys[i].items, items_buf);
            let m = &mut monkeys[i];

            for item in &mut items_buf {
                *item = m.operation.call(*item) % max_div;

                throws.push(m.victims[(*item % m.divtest != 0) as usize]);
            }

            for (&item, &victim) in items_buf.iter().zip(&throws) {
                monkeys[victim].items.push(item);
            }

            throws.clear();
            items_buf.clear();
        }
    }

    inspections.sort();

    let ans = inspections.pop().unwrap() * inspections.pop().unwrap();

    println!("{ans}");
}
