use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use ndarray::Array2;

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_input(filename: &str) -> Array2<u8> {
    let mut w = 0;

    let v = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .fold(Vec::new(), |mut v, l| {
            w = w.max(l.len());
            v.extend(l.chars().map(|c| c as u8 - b'0'));
            v
        });

    let h = v.len() / w;

    Array2::from_shape_vec((w, h), v).unwrap()
}

const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn part1() {
    let forest = parse_input("input/day8/input");

    let ans = forest
        .indexed_iter()
        .filter(|&((x, y), h)| {
            DIRS.iter().any(|&(dx, dy)| {
                (0..)
                    .scan((x as isize, y as isize), |(nx, ny), _| {
                        *nx += dx;
                        *ny += dy;

                        forest.get((*nx as usize, *ny as usize))
                    })
                    .all(|nh| nh < h)
            })
        })
        .count();

    println!("{ans}");
}

fn part2() {
    let forest = parse_input("input/day8/input");

    let ans = forest
        .indexed_iter()
        .map(|((x, y), h)| {
            DIRS.iter()
                .map(|&(dx, dy)| {
                    (0..)
                        .scan((x as isize, y as isize), |(nx, ny), _| {
                            *nx += dx;
                            *ny += dy;

                            forest.get((*nx as usize, *ny as usize))
                        })
                        .scan(false, |blocked, nh| {
                            if *blocked {
                                None
                            } else {
                                if nh >= h {
                                    *blocked = true;
                                }
                                Some(())
                            }
                        })
                        .count()
                })
                .fold(1, |p, x| p * x)
        })
        .max()
        .unwrap();

    println!("{ans}");
}
