use std::collections::HashMap;

pub fn part1(input: String) -> u32 {
    // Parse input into grid. Top-left is (0, 0). Positive directions are
    // rightwards and downwards.

    // From each direction, find all positions of trees visible in that
    // direction.

    // Take the union of the set of visible trees from each direction. This
    // avoids double-counting trees that are visible from multiple directions.
    todo!()
}

type Position = (u32, u32);

struct Grid {
    cells: HashMap<Position, u32>,
    width: u32,
    height: u32,
}

pub fn part2(input: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.into()), 0)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.into()), 0)
    }
}
