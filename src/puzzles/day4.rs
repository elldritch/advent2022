use std::process::exit;

use nom::character::complete::{char, newline, u32};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::{sequence::separated_pair, IResult};

pub fn part1(input: String) -> u32 {
    let pairs = match parse(&input) {
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

    pairs
        .iter()
        .filter(|(a, b)| -> bool { within(a, b) || within(b, a) })
        .count() as u32
}

fn within(a: &Section, b: &Section) -> bool {
    a.0 >= b.0 && a.1 <= b.1
}

fn parse(input: &String) -> IResult<&str, Vec<(Section, Section)>> {
    many1(terminated(pair, newline))(input.as_str())
}

type Section = (u32, u32);

fn pair(input: &str) -> IResult<&str, (Section, Section)> {
    separated_pair(section, char(','), section)(input)
}

fn section(input: &str) -> IResult<&str, Section> {
    separated_pair(u32, char('-'), u32)(input)
}

pub fn part2(input: String) -> u32 {
    todo!("not yet implemented")
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
        todo!("part 2 not yet unlocked");
        // assert_eq!(part2(EXAMPLE_INPUT.into()), 0)
    }
}
