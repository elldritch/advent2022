use std::{collections::HashSet, process::exit};

pub fn part1(input: String) -> u32 {
    for (i, window) in input
        .chars()
        .collect::<Vec<char>>()
        .as_slice()
        .windows(4)
        .enumerate()
    {
        if window.iter().collect::<HashSet<&char>>().len() == 4 {
            return (i + 4) as u32;
        }
    }
    println!("Impossible: no marker value detected");
    exit(1)
}

pub fn part2(input: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE_INPUT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE_INPUT_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE_INPUT_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE_INPUT_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT_1.into()), 7);
        assert_eq!(part1(EXAMPLE_INPUT_2.into()), 5);
        assert_eq!(part1(EXAMPLE_INPUT_3.into()), 6);
        assert_eq!(part1(EXAMPLE_INPUT_4.into()), 10);
        assert_eq!(part1(EXAMPLE_INPUT_5.into()), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT_1.into()), 0)
    }
}
