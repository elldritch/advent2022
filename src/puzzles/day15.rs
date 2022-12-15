use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::map,
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};

pub fn part1(input: String) -> usize {
    part1_solve(2_000_000, input.as_str())
}

fn part1_solve(target_row: i32, input: &str) -> usize {
    let sensors = super::shared::must_parse(parse, input);

    // For each sensor, compute its beaconless range, then filter by the target
    // row.
    sensors
        .into_iter()
        .flat_map(
            |Sensor {
                 position,
                 closest_beacon,
             }| {
                let distance = manhattan(position, closest_beacon);
                (position.0 - distance..=position.0 + distance)
                    .flat_map(move |x| {
                        (position.1 - distance..=position.1 + distance).map(move |y| (x, y))
                    })
                    .filter(move |candidate| {
                        manhattan(position, *candidate) <= distance && *candidate != closest_beacon
                    })
            },
        )
        .filter(|(_, y)| *y == target_row)
        .collect::<HashSet<_>>()
        .len()
}

pub fn part2(_input: String) -> u32 {
    todo!()
}

fn manhattan(a: Position, b: Position) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

type Position = (i32, i32);

#[derive(Debug)]
struct Sensor {
    position: Position,
    closest_beacon: Position,
}

fn parse(input: &str) -> IResult<&str, Vec<Sensor>> {
    many1(terminated(
        map(
            tuple((
                tag("Sensor at x="),
                i32,
                tag(", y="),
                i32,
                tag(": closest beacon is at x="),
                i32,
                tag(", y="),
                i32,
            )),
            |(_, sensor_x, _, sensor_y, _, beacon_x, _, beacon_y)| Sensor {
                position: (sensor_x, sensor_y),
                closest_beacon: (beacon_x, beacon_y),
            },
        ),
        newline,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_part1() {
        assert_eq!(part1_solve(10, EXAMPLE_INPUT.into()), 26)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.into()), 0)
    }
}
