use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;
use ndarray::Array2;

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

fn part2() {
    let mut x = 1;

    let mut sx = 0;
    let mut sy = 0;

    let mut screen =
        Array2::from_shape_vec((6, 40), vec![false; 40 * 6]).unwrap();

    fn push_to_screen(
        screen: &mut Array2<bool>,
        sx: &mut usize,
        sy: &mut usize,
        x: i32,
    ) {
        if (*sx as i32 - x).abs() <= 1 {
            screen[(*sy, *sx)] = true;
        }

        *sx += 1;
        if *sx == 40 {
            *sx = 0;
            *sy += 1;
        }
    }

    fn draw_screen(screen: &Array2<bool>) {
        for r in screen.rows() {
            for &x in r {
                print!("{}", if x { '#' } else { ' ' });
            }
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
            push_to_screen(&mut screen, &mut sx, &mut sy, x);
            push_to_screen(&mut screen, &mut sx, &mut sy, x);

            x += n;
        } else {
            push_to_screen(&mut screen, &mut sx, &mut sy, x);
        }
    }

    draw_screen(&screen);
}
