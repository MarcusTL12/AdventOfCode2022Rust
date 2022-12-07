use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Debug)]
struct Dir {
    size: usize,
    subdirs: Vec<Dir>,
}

fn parse_filesystem<I: Iterator<Item = String>>(lines: &mut I) -> Dir {
    let mut local_root = Dir {
        size: 0,
        subdirs: Vec::new(),
    };

    while let Some(l) = lines.next() {
        if l.starts_with("$ cd") {
            if let Some("..") = l.split_ascii_whitespace().skip(2).next() {
                break;
            } else {
                let new_node = parse_filesystem(lines);
                local_root.size += new_node.size;
                local_root.subdirs.push(new_node);
            }
        } else if let Some('0'..='9') = l.chars().next() {
            local_root.size += l
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
        }
    }

    local_root
}

fn sum_up_sub_100000(node: &Dir) -> usize {
    (if node.size <= 100000 { node.size } else { 0 })
        + node.subdirs.iter().map(sum_up_sub_100000).sum::<usize>()
}

fn part1() {
    let mut lines = BufReader::new(File::open("input/day7/input").unwrap())
        .lines()
        .map(|l| l.unwrap());

    lines.next().unwrap(); // get rid of root cd

    let root = parse_filesystem(&mut lines);

    let ans = sum_up_sub_100000(&root);

    println!("{ans}");
}

fn find_smallest_delete(node: &Dir, cur_smallest: &mut usize, unused: usize) {
    if node.size + unused > 30000000 && node.size < *cur_smallest {
        *cur_smallest = node.size
    }

    for subdir in &node.subdirs {
        find_smallest_delete(&subdir, cur_smallest, unused);
    }
}

fn part2() {
    let mut lines = BufReader::new(File::open("input/day7/input").unwrap())
        .lines()
        .map(|l| l.unwrap());

    lines.next().unwrap(); // get rid of root cd

    let root = parse_filesystem(&mut lines);

    let unused = 70000000 - root.size;

    let mut smallest = 70000000;

    find_smallest_delete(&root, &mut smallest, unused);

    println!("{smallest}");
}
