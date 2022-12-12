#![feature(array_zip)]

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let funcs = [
        day1::PARTS,
        day2::PARTS,
        day3::PARTS,
        day4::PARTS,
        day5::PARTS,
        day6::PARTS,
        day7::PARTS,
        day8::PARTS,
        day9::PARTS,
        day10::PARTS,
        day11::PARTS,
        day12::PARTS,
    ];
    let mut args = std::env::args();
    args.next();
    match args.next() {
        Some(x) if x == "all" => {
            println!("Running all days:");
            println!("===========================");
            let timer = std::time::Instant::now();
            for (i, parts) in funcs.iter().enumerate() {
                let subtimer = std::time::Instant::now();
                println!("---------------------------");
                println!("Running Day {}", i + 1);
                println!("Part 1:");
                parts[0]();
                println!("{:?}\n", subtimer.elapsed());

                let subtimer = std::time::Instant::now();
                println!("Part 2:");
                parts[1]();
                println!("{:?}", subtimer.elapsed());
            }
            println!("===========================");
            println!("Took {:?}", timer.elapsed());
        }
        Some(x) => {
            if let Ok(x) = x.parse::<usize>() {
                if let Some(y) = args.next() {
                    if let Ok(y) = y.parse::<usize>() {
                        if let Some(x) = funcs.get(x - 1) {
                            if let Some(x) = x.get(y - 1) {
                                let timer = std::time::Instant::now();
                                x();
                                println!("Took {:?}", timer.elapsed());
                            } else {
                                println!("Not implemented");
                            }
                        } else {
                            println!("Not implemented");
                        }
                    } else {
                        println!("Must enter numbers!");
                    }
                } else {
                    println!("Pass day and part as commandline parameters");
                }
            } else {
                println!("Must enter numbers!");
            }
        }
        _ => println!(concat!(
            "Run specific day with day and part as command ",
            "line arguments, \nor run all days by giving \"all\" as argument"
        )),
    }
}
