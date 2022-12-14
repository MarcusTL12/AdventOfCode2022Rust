use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;
use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_tiles(filename: &str) -> (HashSet<[i32; 2]>, i32) {
    let mut tiles = HashSet::new();

    for l in BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
    {
        for (from, to) in l
            .split(" -> ")
            .map(|s| {
                s.split(',')
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<ArrayVec<_, 2>>()
                    .into_inner()
                    .unwrap()
            })
            .tuple_windows()
        {
            if from[0] == to[0] {
                let miny = from[1].min(to[1]);
                let maxy = from[1].max(to[1]);
                for y in miny..=maxy {
                    tiles.insert([from[0], y]);
                }
            } else if from[1] == to[1] {
                let minx = from[0].min(to[0]);
                let maxx = from[0].max(to[0]);
                for x in minx..=maxx {
                    tiles.insert([x, from[1]]);
                }
            } else {
                panic!("Diagonals not allowed! {from:?}, {to:?}");
            }
        }
    }

    let max_y = tiles.iter().map(|&[_, y]| y).max().unwrap();

    (tiles, max_y)
}

const DIRS: [[i32; 2]; 3] = [[0, 1], [-1, 1], [1, 1]];

fn add_sand(
    tiles: &mut HashSet<[i32; 2]>,
    mut sand: [i32; 2],
    max_y: i32,
) -> bool {
    while sand[1] < max_y {
        if let Some(n_sand) = DIRS
            .iter()
            .map(|&d| sand.zip(d).map(|(a, b)| a + b))
            .filter(|s| !tiles.contains(s))
            .next()
        {
            sand = n_sand;
        } else {
            tiles.insert(sand);
            return true;
        }
    }

    false
}

fn part1() {
    let (mut tiles, max_y) = parse_tiles("input/day14/input");

    let ans = (0..)
        .filter(|_| !add_sand(&mut tiles, [500, 0], max_y))
        .next()
        .unwrap();

    println!("{ans}");
}

fn part2() {
    let (mut tiles, max_y) = parse_tiles("input/day14/input");

    for x in (500 - (max_y + 5))..=(500 + (max_y + 5)) {
        tiles.insert([x, max_y + 2]);
    }

    let ans = (1..)
        .filter(|_| {
            add_sand(&mut tiles, [500, 0], max_y + 3);
            tiles.contains(&[500, 0])
        })
        .next()
        .unwrap();

    println!("{ans}");
}
