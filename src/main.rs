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
        fs::read_to_string(&args.input_filepath).unwrap_or_else(|err: std::io::Error| -> String {
            println!("Could not open puzzle input file: {}", err);
            exit(1)
        });

    match args.day {
        1 => run_day(puzzles::day1::part1, puzzles::day1::part2, args, input),
        2 => run_day(puzzles::day2::part1, puzzles::day2::part2, args, input),
        _ => println!("Unknown puzzle day: {}", args.day),
    }
}

fn run_day<F1, F2>(part1: F1, part2: F2, args: Args, input: String)
where
    F1: Fn(String) -> u32,
    F2: Fn(String) -> u32,
{
    match args.part {
        1 => println!("{}", part1(input)),
        2 => println!("{}", part2(input)),
        _ => println!("Unknown puzzle part: day {}, part {}", args.day, args.part),
    }
}
