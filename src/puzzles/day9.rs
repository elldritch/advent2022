use std::collections::HashSet;

use nom::{
    branch::alt,
    character::complete::{char, newline, u32},
    combinator::map,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

// The trick is that there is a simpler way to formulate the behavior of the
// tail: if the head is not touching the tail, then the tail moves to the
// previous location of the head.
pub fn part1(input: String) -> usize {
    let steps = super::shared::must_parse(parse, input.as_str());

    let mut tails: HashSet<Position> = HashSet::new();

    // Coordinate system treats rightwards and upwards as positive.
    let mut head = (0, 0);
    let mut tail = (0, 0);
    for step in steps {
        for _ in 0..step.distance {
            tails.insert(tail);
            let last_head = head;
            match step.direction {
                Up => head.1 += 1,
                Down => head.1 -= 1,
                Left => head.0 -= 1,
                Right => head.0 += 1,
            }
            if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                tail = last_head;
            }
        }
    }
    tails.insert(tail);
    tails.len()
}

type Position = (i32, i32);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

#[derive(Debug)]
struct Step {
    direction: Direction,
    distance: u32,
}

fn parse(input: &str) -> IResult<&str, Vec<Step>> {
    many1(terminated(
        map(
            separated_pair(
                alt((
                    map(char('U'), |_| Up),
                    map(char('D'), |_| Down),
                    map(char('L'), |_| Left),
                    map(char('R'), |_| Right),
                )),
                char(' '),
                u32,
            ),
            |(direction, distance)| Step {
                direction,
                distance,
            },
        ),
        newline,
    ))(input)
}

pub fn part2(input: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.into()), 13)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.into()), 0)
    }
}
