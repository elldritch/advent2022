pub fn part1(input: String) -> u32 {
    panic!("not yet implemented")
}

pub fn part2(input: String) -> u32 {
    panic!("not yet implemented")
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
        panic!("part 2 not yet unlocked");
        // assert_eq!(part2(EXAMPLE_INPUT.into()), 0)
    }
}
