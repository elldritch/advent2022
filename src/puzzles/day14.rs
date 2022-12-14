use std::{
    cmp::{max, min},
    collections::HashMap,
    ops::RangeInclusive,
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
    let mut cave = super::shared::must_parse(parse, input.as_str());

    // Simulate sand falls until fixpoint.
    let mut sands = 0;
    while let Some(new_sand) = add_sand(&cave) {
        cave.insert(new_sand, Sand);
        sands += 1;
    }

    sands
}

pub fn part2(_input: String) -> u32 {
    todo!()
}

fn add_sand(cave: &Cave) -> Option<Position> {
    // Calculate abyss boundary. If the sand goes past the lowest rock
    // level, it falls into the abyss.
    let abyss_y = cave
        .iter()
        .filter(|(_, tile)| **tile == Rock)
        .max_by_key(|((_, y), _)| *y)
        .map(|((_, y), _)| *y)
        .unwrap_or_else(|| {
            println!("Invalid: cave had no rocks");
            exit(1)
        });

    // Take steps until the sand settles. If the sand goes past the lowest rock
    // level, it falls into the abyss.
    let mut sand_position = SAND_SOURCE;
    loop {
        let (x, y) = sand_position;
        if y > abyss_y {
            return None;
        } else if let None = cave.get(&(x, y + 1)) {
            sand_position = (x, y + 1)
        } else if let None = cave.get(&(x - 1, y + 1)) {
            sand_position = (x - 1, y + 1)
        } else if let None = cave.get(&(x + 1, y + 1)) {
            sand_position = (x + 1, y + 1)
        } else {
            return Some(sand_position);
        }
    }
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
}
use Tile::*;

fn print_cave(cave: &Cave) -> String {
    let occupied_positions: Vec<Position> = cave.clone().into_iter().map(|(pos, _)| pos).collect();

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
