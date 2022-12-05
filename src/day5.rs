use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_stacks<I: Iterator<Item = String>>(
    mut lines: &mut I,
) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();

    for l in &mut lines {
        if let Some('1') = l.chars().skip(1).next() {
            break;
        }

        if stacks.is_empty() {
            for _ in 0..=l.len() / 4 {
                stacks.push(Vec::new());
            }
        }

        let mut chrs = l.chars().skip(1).step_by(4);
        for s in stacks.iter_mut() {
            let nc = chrs.next();

            if let Some('A'..='Z') = nc {
                s.push(nc.unwrap());
            }
        }
    }

    lines.next().unwrap();

    for s in stacks.iter_mut() {
        s.reverse();
    }

    stacks
}

fn part1() {
    let mut lines = BufReader::new(File::open("input/day5/input").unwrap())
        .lines()
        .map(|l| l.unwrap());

    let mut stacks = parse_stacks(&mut lines);

    for l in lines {
        let mut spl = l.split_ascii_whitespace().skip(1).step_by(2);
        let a: usize = spl.next().unwrap().parse().unwrap();
        let b = spl.next().unwrap().parse::<usize>().unwrap() - 1;
        let c = spl.next().unwrap().parse::<usize>().unwrap() - 1;

        for _ in 0..a {
            if let Some(element) = stacks[b].pop() {
                stacks[c].push(element);
            }
        }
    }

    let ans: String = stacks.iter().map(|s| s.last().unwrap()).collect();

    println!("{ans}");
}

fn part2() {
    let mut lines = BufReader::new(File::open("input/day5/input").unwrap())
        .lines()
        .map(|l| l.unwrap());

    let mut stacks = parse_stacks(&mut lines);

    let mut tmpstack = Vec::new();

    for l in lines {
        let mut spl = l.split_ascii_whitespace().skip(1).step_by(2);
        let a: usize = spl.next().unwrap().parse().unwrap();
        let b = spl.next().unwrap().parse::<usize>().unwrap() - 1;
        let c = spl.next().unwrap().parse::<usize>().unwrap() - 1;

        for _ in 0..a {
            if let Some(element) = stacks[b].pop() {
                tmpstack.push(element);
            }
        }

        stacks[c].extend(tmpstack.iter().rev());
        tmpstack.clear();
    }

    let ans: String = stacks.iter().map(|s| s.last().unwrap()).collect();

    println!("{ans}");
}
