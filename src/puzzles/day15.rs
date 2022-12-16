use std::{collections::HashSet, process::exit, time::SystemTime};

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

    // For each sensor, compute excluded positions for the target row.
    sensors
        .into_iter()
        .flat_map(
            |Sensor {
                 position,
                 closest_beacon,
             }| {
                let distance = manhattan(position, closest_beacon);
                (position.0 - distance..=position.0 + distance)
                    .map(move |x| (x, target_row))
                    .filter(move |candidate| {
                        manhattan(position, *candidate) <= distance && *candidate != closest_beacon
                    })
            },
        )
        .collect::<HashSet<_>>()
        .len()
}

pub fn part2(input: String) -> i32 {
    part2_solve(4_000_000, input.as_str())
}

fn part2_solve(search_area: i32, input: &str) -> i32 {
    let start = SystemTime::now();
    let sensors = super::shared::must_parse(parse, input);
    println!(
        "{:?} | Parsed {} sensor readings",
        start.elapsed().unwrap(),
        sensors.len()
    );

    // The key insight: since there's only a single possible position for a
    // distress beacon, that position must be exactly one tile just outside a
    // covered position. Otherwise, there would be multiple possible positions
    // for a distress beacon!
    //
    // Rather than searching the entire space, we can instead limit our search
    // to all tiles that are just outside covered sensor range, which is equal
    // to all tiles that are distance+1 from a sensor.
    let mut candidates = sensors
        .clone()
        .into_iter()
        .map(
            |Sensor {
                 position,
                 closest_beacon,
             }| {
                println!(
                    "{:?} | Calculating candidates for sensor at {:?}",
                    start.elapsed().unwrap(),
                    position
                );
                let distance = manhattan(position, closest_beacon);
                // Map a sensor to the set of boundary tiles that are candidates for
                // the distress beacon's position.
                let mut candidates = HashSet::new();
                let (x, y) = position;
                let left_bound = x - (distance + 1);
                for i in left_bound..=x {
                    candidates.insert((i, y + (i - left_bound)));
                    candidates.insert((i, y - (i - left_bound)));
                }
                let right_bound = x + (distance + 1);
                for i in x..=right_bound {
                    candidates.insert((i, y + (right_bound - i)));
                    candidates.insert((i, y - (right_bound - i)));
                }
                candidates
                    .into_iter()
                    .filter(|(x, y)| *x > 0 && *x <= search_area && *y > 0 && *y <= search_area)
                    .collect::<HashSet<_>>()
            },
        )
        .reduce(|acc, candidates| {
            println!("{:?} | Reducing with {} accumulated candidates", start.elapsed().unwrap(), acc.len());
            acc.union(&candidates).copied().collect()
        })
        .unwrap_or_else(|| {
            println!("Impossible: no sensor readings");
            exit(1)
        });

    println!("{:?} | Found {} candidates", start.elapsed().unwrap(), candidates.len());

    for Sensor {
        position,
        closest_beacon,
    } in &sensors
    {
        println!("{:?} | Filtering for sensor at {:?}", start.elapsed().unwrap(), position);
        let distance = manhattan(*position, *closest_beacon);
        let mut filtered_candidates = candidates.clone();
        for candidate in candidates {
            if manhattan(*position, candidate) <= distance {
                filtered_candidates.remove(&candidate);
            }
        }
        println!("{:?} | After filtering, {} candidates remain", start.elapsed().unwrap(), filtered_candidates.len());
        candidates = filtered_candidates;
    }

    assert_eq!(candidates.len(), 1);
    let (x, y) = candidates.drain().next().unwrap_or_else(|| {
        println!("Impossible: no distress beacon candidates");
        exit(1)
    });
    x * 4_000_000 + y
}

fn manhattan(a: Position, b: Position) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

type Position = (i32, i32);

#[derive(Debug, Clone)]
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
        assert_eq!(part2_solve(20, EXAMPLE_INPUT.into()), 56000011)
    }
}
