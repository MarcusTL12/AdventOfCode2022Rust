use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans: i32 = BufReader::new(File::open("input/day2/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            if let [abc, _, xyz] = l.as_bytes() {
                let a = (abc - b'A') as i32;
                let b = (xyz - b'X') as i32;

                (b + 1) + (b - a + 4) % 3 * 3
            } else {
                0
            }
        })
        .sum();

    println!("{}", ans);
}

fn part2() {
    let ans: i32 = BufReader::new(File::open("input/day2/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            if let [abc, _, xyz] = l.as_bytes() {
                let a = (abc - b'A') as i32;
                let b = (xyz - b'X') as i32;

                b * 3 + (b + a + 2) % 3 + 1
            } else {
                0
            }
        })
        .sum();

    println!("{}", ans);
}
