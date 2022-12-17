use std::{
    collections::HashSet,
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
        if let Some(r) = range_union(ranges[i], new_range) {
            ranges[i] = r;

            while ranges.len() > i + 1
                && !range_disconnected(ranges[i], ranges[i + 1])
            {
                ranges[i] = range_union(ranges[i], ranges[i + 1]).unwrap();
                ranges.remove(i + 1);
            }
        } else {
            ranges.insert(i, new_range);
        }
    } else {
        ranges.push(new_range);
    }
}

fn x_range_at_y(
    [[sx, sy], [bx, by]]: [[i32; 2]; 2],
    y: i32,
) -> Option<[i32; 2]> {
    let d = (sx - bx).abs() + (sy - by).abs();

    if (sy - y).abs() <= d {
        let half_width = d - (sy - y).abs();
        Some([sx - half_width, sx + half_width])
    } else {
        None
    }
}

fn part1() {
    let input = parse_input("input/day15/input");

    let y = 2000000;
    // let y = 10;

    let mut beacons_in_row = HashSet::new();

    let mut ranges = Vec::new();

    for s in input {
        if s[1][1] == y {
            beacons_in_row.insert(s[1][0]);
        };

        if let Some(r) = x_range_at_y(s, y) {
            add_range_to_list(&mut ranges, r);
        }
    }

    let ans = ranges.iter().map(|[a, b]| b - a + 1).sum::<i32>()
        - beacons_in_row.len() as i32;

    println!("{ans}");
}

fn part2() {
    let input = parse_input("input/day15/input");

    let box_width = 4000_000;

    let mut ranges = Vec::new();

    for y in 0..=box_width {
        ranges.clear();
        for &s in &input {
            if let Some(r) = x_range_at_y(s, y) {
                if r[1] >= 0 && r[0] <= box_width {
                    let r = [r[0].max(0), r[1].min(box_width)];
                    add_range_to_list(&mut ranges, r);
                }
            }
        }

        if ranges.len() > 1 {
            let x = ranges[0][1] + 1;

            let ans = (x as i64) * (box_width as i64) + y as i64;

            println!("{ans}");

            break;
        }
    }
}
