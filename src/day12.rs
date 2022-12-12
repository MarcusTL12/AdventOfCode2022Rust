use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use ndarray::Array2;

pub const PARTS: [fn(); 2] = [part1, part2];

const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn parse_input(filename: &str) -> (Array2<i32>, [usize; 2], [usize; 2]) {
    let mut w = 0;

    let mut start = [0, 0];
    let mut stop = [0, 0];

    let v = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
        .fold(Vec::new(), |mut v, (i, l)| {
            w = w.max(l.len());
            v.extend(l.chars().enumerate().map(|(j, c)| match c {
                'S' => {
                    start = [i, j];
                    'a'
                }
                'E' => {
                    stop = [i, j];
                    'z'
                }
                _ => c,
            } as i32));
            v
        });

    let h = v.len() / w;

    (Array2::from_shape_vec((h, w), v).unwrap(), start, stop)
}

fn part1() {
    let (mountain, start, stop) = parse_input("input/day12/input");

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start, 0));
    visited.insert(start);

    'outer: while let Some((pos, l)) = queue.pop_front() {
        for npos in DIRS
            .into_iter()
            .map(|d| pos.zip(d).map(|(q, dq)| (q as isize + dq) as usize))
        {
            if !visited.contains(&npos) {
                if let Some(c) = mountain.get(npos) {
                    if c - mountain[pos] <= 1 {
                        if npos == stop {
                            println!("{}", l + 1);
                            break 'outer;
                        }

                        queue.push_back((npos, l + 1));
                        visited.insert(npos);
                    }
                }
            }
        }
    }
}

fn part2() {
    let (mountain, _, stop) = parse_input("input/day12/input");

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((stop, 0));
    visited.insert(stop);

    'outer: while let Some((pos, l)) = queue.pop_front() {
        for npos in DIRS
            .into_iter()
            .map(|d| pos.zip(d).map(|(q, dq)| (q as isize + dq) as usize))
        {
            if !visited.contains(&npos) {
                if let Some(c) = mountain.get(npos) {
                    if c - mountain[pos] >= -1 {
                        if *c == (b'a' as i32) {
                            println!("{}", l + 1);
                            break 'outer;
                        }

                        queue.push_back((npos, l + 1));
                        visited.insert(npos);
                    }
                }
            }
        }
    }
}
