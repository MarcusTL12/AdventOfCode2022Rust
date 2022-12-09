use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

fn solve(filename: &str, ropelen: usize) -> usize {
    let mut rope = vec![[0i32, 0]; ropelen];
    let mut visited = HashSet::new();
    visited.insert([0, 0]);

    for l in BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
    {
        let l = l
            .split_ascii_whitespace()
            .collect::<ArrayVec<_, 2>>()
            .into_inner()
            .unwrap();

        let d = match l[0] {
            "R" => [1, 0],
            "L" => [-1, 0],
            "U" => [0, 1],
            "D" => [0, -1],
            _ => panic!("Got weird input: {l:?}"),
        };

        let n = l[1].parse().unwrap();

        for _ in 0..n {
            rope[0] = rope[0].zip(d).map(|(a, b)| a + b);

            for i in 1..ropelen {
                let df = rope[i - 1].zip(rope[i]).map(|(a, b)| a - b);

                if *df.map(|x| x.abs()).iter().max().unwrap() > 1 {
                    rope[i] =
                        rope[i].zip(df.map(|x| x.signum())).map(|(a, b)| a + b);
                }
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

fn part1() {
    let ans = solve("input/day9/input", 2);

    println!("{ans}");
}

fn part2() {
    let ans = solve("input/day9/input", 10);

    println!("{ans}");
}
