use std::{
    cmp::{max, min},
    collections::HashMap,
    ops::{Range, RangeInclusive},
    process::exit,
    u32::{MAX, MIN},
};

use nom::{
    bytes::complete::tag,
    character::complete::{char, newline, u32},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

const SAND_SOURCE: Position = (500, 0);

pub fn part1(input: String) -> u32 {
    // Parse the cave.
    let cave = super::shared::must_parse(parse, input.as_str());
    println!("{}", print_cave(cave));

    // Sand falls into the abyss if it falls lower than the lowest level of
    // rock.

    // Simulate sand falls until fixpoint.

    todo!()
}

pub fn part2(_input: String) -> u32 {
    todo!()
}

// Positive directions are (rightwards, downwards).
type Position = (u32, u32);

// Map from a position to whether it's occupied. Both sand and rock count as
// occupied, and air does not.
type Cave = HashMap<Position, Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rock,
    Sand,
    Air,
}
use Tile::*;

fn print_cave(cave: Cave) -> String {
    let occupied_positions: Vec<Position> = cave
        .clone()
        .into_iter()
        .filter(|(_, occupied)| *occupied != Air)
        .map(|(pos, _)| pos)
        .collect();

    let (min_x, max_x, max_y) = {
        let mut min_x = MAX;
        let mut max_x = MIN;
        let mut max_y = MIN;

        for (x, y) in occupied_positions {
            min_x = min(min_x, x);
            max_x = max(max_x, x);
            max_y = max(max_y, y);
        }

        (min_x, max_x, max_y)
    };

    let mut rendered = String::new();
    for y in 0..max_y + 1 {
        for x in min_x..max_x + 1 {
            if (x, y) == (500, 0) {
                rendered.push('+');
            } else {
                rendered.push(match cave.get(&(x, y)) {
                    Some(Rock) => '#',
                    Some(Sand) => 'o',
                    Some(Air) => '.',
                    None => '.',
                });
            }
        }
        rendered.push('\n');
    }
    rendered
}

fn between_inclusive(a: u32, b: u32) -> RangeInclusive<u32> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

fn parse(input: &str) -> IResult<&str, Cave> {
    let (remaining, paths) = many1(terminated(
        separated_list1(tag(" -> "), separated_pair(u32, char(','), u32)),
        newline,
    ))(input)?;
    Ok((
        remaining,
        HashMap::from_iter(
            paths
                .into_iter()
                .flat_map(|endpoints| {
                    endpoints
                        .windows(2)
                        .flat_map(|ends| match ends {
                            [(a, b), (c, d)] => between_inclusive(*a, *c)
                                .flat_map(|x| between_inclusive(*b, *d).map(move |y| (x, y))),
                            _ => {
                                println!("Impossible: endpoint windows did not pattern match");
                                exit(1)
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .map(|point| (point, Rock)),
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.into()), 24)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.into()), 0)
    }
}
