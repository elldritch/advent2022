use std::{
    collections::{HashMap, HashSet},
    process::exit,
};

pub fn part1(input: String) -> usize {
    // Parse input into grid. Top-left is (0, 0). Positive directions are
    // rightwards and downwards.
    let grid = parse_rectangular_grid(input.as_str());

    // From each tree on the edge, find all positions of trees visible in all
    // directions from that tree.
    //
    // Take the union of the set of visible trees from each edge tree. This
    // avoids double-counting trees that are visible from multiple directions.
    let edges = trees_on_edge(&grid);
    let visible_from_edges = edges
        .iter()
        .flat_map(|position| trees_visible_from(&grid, *position))
        .collect::<HashSet<_>>();

    // All edge trees are also visible.
    let visible = visible_from_edges.union(&edges).collect::<HashSet<_>>();
    visible.len()
}

type Position = (usize, usize);

#[derive(Debug)]
struct Grid {
    cells: HashMap<Position, u32>,
    width: usize,
    height: usize,
}

fn parse_rectangular_grid(input: &str) -> Grid {
    let mut cells = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.lines().enumerate() {
        width = line.len();
        height += 1;
        for (x, element) in line.chars().enumerate() {
            cells.insert((x, y), element.to_digit(10).unwrap_or_else(|| exit(1)));
        }
    }

    Grid {
        cells,
        width,
        height,
    }
}

fn trees_on_edge(grid: &Grid) -> HashSet<Position> {
    let top = (0..grid.width)
        .map(|x| (x, 0))
        .collect::<HashSet<Position>>();
    let bottom = (0..grid.width)
        .map(|x| (x, grid.height - 1))
        .collect::<HashSet<Position>>();
    let left = (0..grid.height)
        .map(|y| (0, y))
        .collect::<HashSet<Position>>();
    let right = (0..grid.height)
        .map(|y| (grid.width - 1, y))
        .collect::<HashSet<Position>>();

    top.union(&bottom)
        .copied()
        .collect::<HashSet<_>>()
        .union(&left)
        .copied()
        .collect::<HashSet<_>>()
        .union(&right)
        .copied()
        .collect::<HashSet<_>>()
}

fn trees_visible_from(grid: &Grid, position: Position) -> HashSet<Position> {
    let (x, y) = position;

    let los_up = (0..y).map(|y2| (x, y2)).rev().collect::<Vec<Position>>();
    let los_down = (y + 1..grid.height)
        .map(|y2| (x, y2))
        .collect::<Vec<Position>>();
    let los_left = (0..x).map(|x2| (x2, y)).rev().collect::<Vec<Position>>();
    let los_right = (x + 1..grid.width)
        .map(|x2| (x2, y))
        .collect::<Vec<Position>>();

    vec![los_up, los_down, los_left, los_right]
        .iter()
        .flat_map(|los| trees_visible_along(grid, position, los))
        .collect::<HashSet<_>>()
}

fn trees_visible_along(
    grid: &Grid,
    position: Position,
    line_of_sight: &Vec<Position>,
) -> HashSet<Position> {
    let start_height = grid.cells.get(&position).unwrap_or_else(|| exit(1));
    let mut visible = HashSet::new();
    line_of_sight
        .iter()
        .fold(start_height, |last_highest, position| {
            let height = grid.cells.get(&position).unwrap_or_else(|| exit(1));
            if height > last_highest {
                visible.insert(*position);
                height
            } else {
                last_highest
            }
        });
    visible
}

pub fn part2(input: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.into()), 21)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.into()), 0)
    }
}
