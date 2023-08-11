pub fn part1(_input: String) -> u32 {
    // I think the approach here is dynamic programming, starting from the end.
    //
    // - We have 30 minutes worth of choices, and each choice takes 1 minute,
    //   and we have N choices every minute (open a valve or go down a tunnel).
    // - Start from the end because the value of opening a valve depends on how
    //   many minutes it's opened for (i.e. distance from the end).
    // - On each step backwards, memoize the most valuable subsequent steps that
    //   can be taken if you take that step.
    // - Memoization table: column = "minute", row = "room", transition =
    //   "action", cell = "most value that can be achieved from here forwards"
    // - Build the table, then traverse forwards from the starting position.
    //
    // Alternatively, maybe it's a graph search? Or a greedy search?
    todo!()
}

pub fn part2(_input: String) -> u32 {
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
