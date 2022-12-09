use std::{collections::HashSet, process::exit};

use nom::{
    branch::alt,
    character::complete::{char, newline, u32},
    combinator::map,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

pub fn part1(input: String) -> usize {
    let steps = super::shared::must_parse(parse, input.as_str());

    let mut tails: HashSet<Position> = HashSet::new();

    // Coordinate system treats rightwards and upwards as positive.
    let mut head = (0, 0);
    let mut tail = (0, 0);
    for step in steps {
        for _ in 0..step.distance {
            tails.insert(tail);
            match step.direction {
                Up => head.1 += 1,
                Down => head.1 -= 1,
                Left => head.0 -= 1,
                Right => head.0 += 1,
            }
            tail = follow(head, tail);
        }
    }
    tails.insert(tail);
    tails.len()
}

pub fn part2(input: String) -> usize {
    let steps = super::shared::must_parse(parse, input.as_str());

    let mut tails: HashSet<Position> = HashSet::new();

    let rope_len = 10;
    let mut rope: Vec<Position> = (0..rope_len).map(|_| (0, 0)).collect();
    for step in steps {
        for _ in 0..step.distance {
            tails.insert(rope[rope_len - 1]);
            match step.direction {
                Up => rope[0].1 += 1,
                Down => rope[0].1 -= 1,
                Left => rope[0].0 -= 1,
                Right => rope[0].0 += 1,
            }
            for i in 1..rope_len {
                rope[i] = follow(rope[i - 1], rope[i]);
            }
        }
    }
    tails.insert(rope[rope_len - 1]);
    tails.len()
}

fn follow(head: Position, tail: Position) -> Position {
    // We only move when the head is at least 2 away in at least one axis.
    if (head.0 - tail.0).abs() == 2 || (head.1 - tail.1).abs() == 2 {
        // Diagonal cases.
        if head.0 != tail.0 && head.1 != tail.1 {
            (
                tail.0 + (head.0 - tail.0).signum(),
                tail.1 + (head.1 - tail.1).signum(),
            )
        } else if head.0 != tail.0 {
            (tail.0 + (head.0 - tail.0).signum(), tail.1)
        } else if head.1 != tail.1 {
            (tail.0, tail.1 + (head.1 - tail.1).signum())
        } else {
            println!(
                "Impossible: head has distance from tail but all elements equal: {head:?} {tail:?}"
            );
            exit(1)
        }
    } else {
        tail
    }
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
    const EXAMPLE_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.into()), 13)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.into()), 1);
        assert_eq!(part2(EXAMPLE_INPUT_2.into()), 36);
    }
}
