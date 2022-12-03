use std::{process::exit, collections::HashSet};

// In the first compartment, build a set of encountered items. In the second
// compartment, check for items previously seen in the first compartment.
pub fn part1(input: String) -> u32 {
    let lines = input.lines();

    let mut duplicate_priorities = 0;
    for rucksack in lines {
        let len = rucksack.len();
        let mut chars = rucksack.chars();
        assert_eq!(len % 2, 0);
        let compartment_len = len / 2;
        let mut seen = HashSet::new();

        // Build the set of items in the first compartment.
        for _ in 0..compartment_len {
            let c = chars.next().unwrap_or_else(|| -> char {
                println!("Impossible: rucksack did not contain full first compartment");
                exit(1)
            });
            seen.insert(c);
        }

        // Check the second compartment for seen items.
        for _ in 0..compartment_len {
            let c = chars.next().unwrap_or_else(|| -> char {
                println!("Impossible: rucksack did not contain full second compartment");
                exit(1)
            });
            if seen.contains(&c) {
                duplicate_priorities += item_priority(c);
                break
            }
        }
    }

    duplicate_priorities
}

fn item_priority(item: char) -> u32 {
    if item >= 'a' && item <= 'z' {
        (item as u32) - 96
    } else if item >= 'A' && item <= 'Z' {
        (item as u32) - 38
    } else {
        println!("Invalid rucksack item detected: {}", item);
        exit(1)
    }
}

pub fn part2(input: String) -> u32 {
    panic!("not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.into()), 157)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.into()), 70)
    }
}
