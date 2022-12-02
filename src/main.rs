use std::{fs, process::exit};

use clap::Parser;

mod puzzles;

/// Run Advent of Code 2022 puzzle solvers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day number of puzzle to run
    #[arg(short, long)]
    day: u32,

    /// Part number of puzzle to run
    #[arg(short, long)]
    part: u32,

    /// Path to puzzle input file
    #[arg(short, long)]
    input_filepath: String,
}

fn main() {
    let args = Args::parse();

    let input =
        fs::read_to_string(args.input_filepath).unwrap_or_else(|err: std::io::Error| -> String {
            println!("Could not open puzzle input file: {}", err);
            exit(1)
        });

    match args.day {
        1 => match args.part {
            1 => println!("{}", puzzles::day1::part1(input)),
            2 => println!("{}", puzzles::day1::part2(input)),
            _ => println!("Unknown puzzle part: day {}, part {}", args.day, args.part),
        },
        2 => match args.part {
            1 => puzzles::day2::part1(input),
            2 => puzzles::day2::part2(input),
            _ => println!("Unknown puzzle part: day {}, part {}", args.day, args.part),
        },
        _ => println!("Unknown puzzle day: {}", args.day),
    }
}
