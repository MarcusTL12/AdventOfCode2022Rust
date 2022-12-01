use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans = BufReader::new(File::open("input/day1/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .fold((0, 0), |(m, x), l| {
            if let Ok(n) = l.parse::<i32>() {
                (m, x + n)
            } else {
                (m.max(x), 0)
            }
        })
        .0;

    println!("{}", ans);
}

fn part2() {
    let ans: i32 = BufReader::new(File::open("input/day1/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .fold((vec![0, 0, 0], 0), |(mut m, x), l| {
            if let Ok(n) = l.parse::<i32>() {
                (m, x + n)
            } else {
                m.push(x);
                m.sort_by(|a, b| b.cmp(a));
                m.resize(3, 0);
                (m, 0)
            }
        })
        .0
        .into_iter()
        .sum();

    println!("{}", ans);
}
