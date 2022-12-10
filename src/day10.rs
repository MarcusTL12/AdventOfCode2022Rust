use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
    thread,
    time::Duration,
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans = BufReader::new(File::open("input/day10/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            match l
                .split_ascii_whitespace()
                .collect::<ArrayVec<_, 2>>()
                .as_slice()
            {
                ["noop"] => None,
                ["addx", n] => n.parse::<i32>().ok(),
                _ => panic!("Weird input: {l}!"),
            }
        })
        .scan((0, 1), |(cycles, x), op| {
            Some(if let Some(n) = op {
                let ret = [(*cycles + 1, *x), (*cycles + 2, *x)]
                    .into_iter()
                    .collect::<ArrayVec<_, 2>>();
                *cycles += 2;
                *x += n;
                ret
            } else {
                *cycles += 1;
                let ret =
                    [(*cycles, *x)].into_iter().collect::<ArrayVec<_, 2>>();
                ret
            })
        })
        .flat_map(|v| v.into_iter())
        .skip(19)
        .step_by(40)
        .fold(0, |ans, (cycles, x)| {
            println!("{cycles}, {x}");
            ans + cycles * x
        });

    println!("{ans}");
}

const ANIMATE: bool = true;

fn part2() {
    let mut x = 1;

    let mut sx = 0;

    fn push_to_screen(sx: &mut usize, x: i32) {
        print!(
            "{}",
            if (*sx as i32 - x).abs() <= 1 {
                '#'
            } else {
                ' '
            }
        );

        if ANIMATE {
            stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(10));
        }

        *sx += 1;
        if *sx == 40 {
            *sx = 0;
            println!();
        }
    }

    for op in BufReader::new(File::open("input/day10/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            match l
                .split_ascii_whitespace()
                .collect::<ArrayVec<_, 2>>()
                .as_slice()
            {
                ["noop"] => None,
                ["addx", n] => n.parse::<i32>().ok(),
                _ => panic!("Weird input: {l}!"),
            }
        })
    {
        if let Some(n) = op {
            push_to_screen(&mut sx, x);
            push_to_screen(&mut sx, x);

            x += n;
        } else {
            push_to_screen(&mut sx, x);
        }
    }
}
