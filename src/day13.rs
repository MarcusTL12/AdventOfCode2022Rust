use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

fn parse_packet(s: &str) -> (Packet, &str) {
    let s = s.trim();

    let (first_char, rest) = s.split_at(1);

    match first_char {
        "[" => {
            let mut s = rest;

            let mut v = Vec::new();

            while !s.is_empty() && s.chars().next() != Some(']') {
                let (p, rest) = parse_packet(s);
                if let Some(',') = rest.chars().next() {
                    s = rest.split_at(1).1;
                } else {
                    s = rest;
                }

                v.push(p);
            }

            (Packet::List(v), s.split_at(1).1)
        }
        _ => {
            let off = s.chars().take_while(|c| c.is_numeric()).count();

            let (s, rest) = s.split_at(off);

            let n = s.parse().unwrap();

            (Packet::Integer(n), rest)
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.partial_cmp(b),
            (Packet::Integer(a), Packet::List(_)) => {
                Packet::List(vec![Packet::Integer(*a)]).partial_cmp(other)
            }
            (Packet::List(_), Packet::Integer(b)) => {
                self.partial_cmp(&Packet::List(vec![Packet::Integer(*b)]))
            }
            (Packet::List(a), Packet::List(b)) => {
                let mut c = Ordering::Equal;

                for (ax, bx) in a.iter().zip(b) {
                    match ax.partial_cmp(bx) {
                        Some(Ordering::Equal) => (),
                        Some(x) => {
                            c = x;
                            break;
                        }
                        None => panic!(),
                    }
                }

                match c {
                    Ordering::Equal => a.len().partial_cmp(&b.len()),
                    x => Some(x),
                }
            }
        }
    }
}

fn part1() {
    let mut lines = BufReader::new(File::open("input/day13/input").unwrap())
        .lines()
        .map(|l| l.unwrap());

    let mut ans = 0;

    for i in 1.. {
        if let Some((a, b)) = lines.next().and_then(|l1| {
            if !l1.is_empty() {
                lines.next().and_then(|l2| {
                    if !l2.is_empty() {
                        Some((parse_packet(&l1).0, parse_packet(&l2).0))
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        }) {
            if a < b {
                ans += i;
            }

            lines.next();
        } else {
            break;
        }
    }

    println!("{ans}");
}

fn part2() {
    let packets: Vec<_> =
        BufReader::new(File::open("input/day13/input").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .filter(|l| !l.is_empty())
            .map(|l| parse_packet(&l).0)
            .collect();

    let divl = parse_packet("[[2]]").0;
    let divh = parse_packet("[[6]]").0;

    let mut packets_l = Vec::new();
    let mut packets_h = Vec::new();

    for p in packets {
        if p < divl {
            packets_l.push(p);
        } else {
            packets_h.push(p);
        }
    }

    let l_pos = packets_l.len() + 1;
    let h_pos = l_pos + packets_h.into_iter().filter(|p| p < &divh).count() + 1;

    let ans = l_pos * h_pos;

    println!("{ans}");
}
