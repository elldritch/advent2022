use std::process::exit;

use nom::{
    character::complete::{char, newline, u32},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

type Section = (u32, u32);

pub fn part1(input: String) -> u32 {
    count_pairs(
        |(a, b)| -> bool { within(a, b) || within(b, a) },
        input.as_str(),
    )
}

pub fn part2(input: String) -> u32 {
    count_pairs(overlaps, input.as_str())
}

fn count_pairs<F>(predicate: F, input: &str) -> u32
where
    F: Fn(&&((u32, u32), (u32, u32))) -> bool,
{
    let pairs = match parse(input) {
        Ok(("", pairs)) => pairs,
        Ok((remaining, _)) => {
            println!(
                "Invalid puzzle input: could not parse input suffix: {}",
                remaining
            );
            exit(1)
        }
        Err(err) => {
            println!("Could not parse puzzle input: {}", err);
            exit(1)
        }
    };

    pairs.iter().filter(predicate).count() as u32
}

fn within(a: &Section, b: &Section) -> bool {
    a.0 >= b.0 && a.1 <= b.1
}

fn overlaps(input: &&(Section, Section)) -> bool {
    let (a, b) = input;
    a.1 >= b.0 && b.1 >= a.0
}

fn parse(input: &str) -> IResult<&str, Vec<(Section, Section)>> {
    many1(terminated(pair, newline))(input)
}

fn pair(input: &str) -> IResult<&str, (Section, Section)> {
    separated_pair(section, char(','), section)(input)
}

fn section(input: &str) -> IResult<&str, Section> {
    separated_pair(u32, char('-'), u32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.into()), 2)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.into()), 4)
    }
}
