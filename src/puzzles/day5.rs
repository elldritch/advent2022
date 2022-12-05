pub fn part1(input: String) -> String {
    todo!("not yet implemented")
}

pub fn part2(input: String) -> String {
    todo!("not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "    [D]    
 [N] [C]    
 [Z] [M] [P]
  1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.into()), "CMZ")
    }

    #[test]
    fn test_part2() {
        todo!("part 2 not yet unlocked");
        // assert_eq!(part2(EXAMPLE_INPUT.into()), "")
    }
}
