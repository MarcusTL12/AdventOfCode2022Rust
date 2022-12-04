use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans = BufReader::new(File::open("input/day4/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split(',')
                .map(|s| {
                    s.split('-')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<ArrayVec<_, 2>>()
                        .into_inner()
                        .unwrap()
                })
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap()
        })
        .filter(|rs| {
            rs[0][0] <= rs[1][0] && rs[0][1] >= rs[1][1]
                || rs[0][0] >= rs[1][0] && rs[0][1] <= rs[1][1]
        })
        .count();

    println!("{ans}");
}

fn part2() {
    let ans = BufReader::new(File::open("input/day4/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split(',')
                .map(|s| {
                    s.split('-')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<ArrayVec<_, 2>>()
                        .into_inner()
                        .unwrap()
                })
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap()
        })
        .filter(|rs| {
            rs[0][0] >= rs[1][0] && rs[0][0] <= rs[1][1]
                || rs[0][1] >= rs[1][0] && rs[0][1] <= rs[1][1]
                || rs[1][0] >= rs[0][0] && rs[1][0] <= rs[0][1]
                || rs[1][1] >= rs[0][0] && rs[1][1] <= rs[0][1]
        })
        .count();

    println!("{ans}");
}
