use std::{fmt::Display, fs, process::exit};

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
        3 => run_day(puzzles::day3::part1, puzzles::day3::part2, args, input),
        4 => run_day(puzzles::day4::part1, puzzles::day4::part2, args, input),
        5 => run_day(puzzles::day5::part1, puzzles::day5::part2, args, input),
        6 => run_day(puzzles::day6::part1, puzzles::day6::part2, args, input),
        7 => run_day(puzzles::day7::part1, puzzles::day7::part2, args, input),
        8 => run_day(puzzles::day8::part1, puzzles::day8::part2, args, input),
        9 => run_day(puzzles::day9::part1, puzzles::day9::part2, args, input),
        10 => run_day(puzzles::day10::part1, puzzles::day10::part2, args, input),
        11 => run_day(puzzles::day11::part1, puzzles::day11::part2, args, input),
        12 => run_day(puzzles::day12::part1, puzzles::day12::part2, args, input),
        _ => println!("Unknown puzzle day: {}", args.day),
    }
}

fn run_day<F1, F2, R1, R2>(part1: F1, part2: F2, args: Args, input: String)
where
    R1: Display,
    R2: Display,
    F1: Fn(String) -> R1,
    F2: Fn(String) -> R2,
{
    match args.part {
        1 => println!("{}", part1(input)),
        2 => println!("{}", part2(input)),
        _ => println!("Unknown puzzle part: day {}, part {}", args.day, args.part),
    }
}
