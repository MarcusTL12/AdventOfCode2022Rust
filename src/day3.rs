use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn prio(c: u8) -> i32 {
    match c {
        b'a'..=b'z' => (c - b'a') as i32 + 1,
        b'A'..=b'Z' => (c - b'A') as i32 + 27,
        _ => 0,
    }
}

fn part1() {
    let ans: i32 = BufReader::new(File::open("input/day3/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let b = l.as_bytes();
            let (c1, c2) = b.split_at(b.len() / 2);
            let c1 = c1.iter().cloned().collect::<HashSet<_>>();
            let c2 = c2.iter().cloned().collect::<HashSet<_>>();
            c1.intersection(&c2).next().unwrap().clone()
        })
        .map(prio)
        .sum();

    println!("{ans}");
}

fn part2() {
    let ans: i32 = BufReader::new(File::open("input/day3/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .chunks(3)
        .into_iter()
        .map(|mut ch| {
            let a = ch.next().unwrap();
            let b = ch.next().unwrap();
            let c = ch.next().unwrap();

            let a = a.as_bytes().iter().cloned().collect::<HashSet<_>>();
            let b = b.as_bytes().iter().cloned().collect::<HashSet<_>>();
            let c = c.as_bytes().iter().cloned().collect::<HashSet<_>>();

            a.intersection(&b)
                .cloned()
                .collect::<HashSet<_>>()
                .intersection(&c)
                .next()
                .unwrap()
                .clone()
        })
        .map(prio)
        .sum();

    println!("{ans}");
}
