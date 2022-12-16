use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_input(filename: &str) -> Vec<[[i32; 2]; 2]> {
    let reg = Regex::new(concat!(
        r"Sensor at x=(-?\d+), y=(-?\d+): ",
        r"closest beacon is at x=(-?\d+), y=(-?\d+)"
    ))
    .unwrap();

    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let c = reg.captures(&l).unwrap();
            [
                [c[1].parse().unwrap(), c[2].parse().unwrap()],
                [c[3].parse().unwrap(), c[4].parse().unwrap()],
            ]
        })
        .collect()
}

fn range_disconnected(r1: [i32; 2], r2: [i32; 2]) -> bool {
    r1[0] > r2[1] + 1 || r2[0] > r1[1] + 1
}

fn range_union(r1: [i32; 2], r2: [i32; 2]) -> Option<[i32; 2]> {
    if !range_disconnected(r1, r2) {
        Some([r1[0].min(r2[0]), r1[1].max(r2[1])])
    } else {
        None
    }
}

fn add_range_to_list(ranges: &mut Vec<[i32; 2]>, new_range: [i32; 2]) {
    if let Some(i) = (0..ranges.len())
        .skip_while(|&i| new_range[0] > ranges[i][1] + 1)
        .next()
    {
        ranges[i] = range_union(ranges[i], new_range).unwrap();

        while ranges.len() > i + 1
            && !range_disconnected(ranges[i], ranges[i + 1])
        {
            ranges[i] = range_union(ranges[i], new_range).unwrap();
            ranges.remove(i + 1);
        }
    } else {
        ranges.push(new_range);
    }
}

fn part1() {
    // let input = parse_input("input/day15/ex1");

    let mut ranges = vec![[3, 5], [7, 10], [13, 15], [17, 20], [25, 30]];

    add_range_to_list(&mut ranges, [9, 18]);

    println!("{ranges:?}");
}

fn part2() {}
