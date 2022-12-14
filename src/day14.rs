use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashSet,
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_tiles(filename: &str) {
    let mut tiles = HashSet::new();

    for l in BufReader::new(File::open("input/day13/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
    {
        
    }
}

fn part1() {}

fn part2() {}
