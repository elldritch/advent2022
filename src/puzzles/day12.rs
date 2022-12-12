use std::{collections::HashMap, process::exit};

use petgraph::{algo::dijkstra, prelude::DiGraphMap};

pub fn part1(input: String) -> u32 {
    let (start, goal, graph) = parse(input.as_str());
    let paths = dijkstra(&graph, start, Some(goal), |_| 1);
    *paths.get(&goal).unwrap_or_else(|| {
        println!("Impossible: no path from start to goal");
        exit(1)
    })
}

pub fn part2(_input: String) -> u32 {
    todo!()
}

type Position = (i32, i32);

type Height = u32;

fn parse(input: &str) -> (Position, Position, DiGraphMap<Position, ()>) {
    // First, we parse to a Map<Position, Height>. The origin is at the top
    // left, with the positive x direction being rightwards and the positive y
    // direction being downwards.
    let mut heights: HashMap<Position, Height> = HashMap::new();
    let mut start = None;
    let mut goal = None;
    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            if c >= 'a' && c <= 'z' {
                heights.insert((x, y), (c as u32) - 96);
            } else if c == 'S' {
                start = Some((x, y));
                heights.insert((x, y), 1);
            } else if c == 'E' {
                goal = Some((x, y));
                heights.insert((x, y), 26);
            } else {
                println!("Impossible: unrecognized character: {c:?}");
                exit(1)
            }
        }
    }

    // Then we construct a directed graph from the map, where edge (A, B) exists
    // if height(B) <= height(A) + 1.
    let mut graph = DiGraphMap::new();
    for (position @ (x, y), height) in &heights {
        graph.add_node(*position);
        for neighbor in [(*x + 1, *y), (*x - 1, *y), (*x, *y + 1), (*x, *y - 1)] {
            if let Some(neighbor_height) = heights.get(&neighbor) {
                graph.add_node(neighbor);
                if *neighbor_height <= *height + 1 {
                    graph.add_edge(*position, neighbor, ());
                }
            }
        }
    }

    (
        match start {
            Some(s) => s,
            None => {
                println!("Invalid puzzle: no start position detected");
                exit(1)
            }
        },
        match goal {
            Some(e) => e,
            None => {
                println!("Invalid puzzle: no end position detected");
                exit(1)
            }
        },
        graph,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.into()), 31)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.into()), 0)
    }
}
