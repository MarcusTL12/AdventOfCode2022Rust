use std::fs::read_to_string;

pub const PARTS: [fn(); 2] = [part1, part2];

fn solve(s: &[u8], n: usize) -> usize {
    let mut counts = [0; 26];

    let mut n_diff = 0;

    for i in 0..n {
        if counts[(s[i] - b'a') as usize] == 0 {
            n_diff += 1;
        }

        counts[(s[i] - b'a') as usize] += 1;
    }

    for i in 1..(s.len() - n) {
        counts[(s[i - 1] - b'a') as usize] -= 1;
        if counts[(s[i - 1] - b'a') as usize] == 0 {
            n_diff -= 1;
        }

        if counts[(s[i + n - 1] - b'a') as usize] == 0 {
            n_diff += 1
        }
        counts[(s[i + n - 1] - b'a') as usize] += 1;

        if n_diff == n {
            return i + n;
        }
    }

    0
}

fn part1() {
    let s = read_to_string("input/day6/input").unwrap();

    let ans = solve(s.as_bytes(), 4);

    println!("{ans}");
}

fn part2() {
    let s = read_to_string("input/day6/input").unwrap();

    let ans = solve(s.as_bytes(), 14);

    println!("{ans}");
}
