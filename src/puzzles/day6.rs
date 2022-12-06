use std::{collections::HashSet, process::exit};

pub fn part1(input: String) -> usize {
    find_marker(4, input)
}

pub fn part2(input: String) -> usize {
    find_marker(14, input)
}

pub fn find_marker(window_size: usize, input: String) -> usize {
    for (i, window) in input
        .chars()
        .collect::<Vec<char>>()
        .as_slice()
        .windows(window_size)
        .enumerate()
    {
        if window.iter().collect::<HashSet<&char>>().len() == window_size {
            return i + window_size;
        }
    }
    println!("Impossible: no marker value detected");
    exit(1)
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
        assert_eq!(part2(EXAMPLE_INPUT_1.into()), 19);
        assert_eq!(part2(EXAMPLE_INPUT_2.into()), 23);
        assert_eq!(part2(EXAMPLE_INPUT_3.into()), 23);
        assert_eq!(part2(EXAMPLE_INPUT_4.into()), 29);
        assert_eq!(part2(EXAMPLE_INPUT_5.into()), 26);
    }
}
