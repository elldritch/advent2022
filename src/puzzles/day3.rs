use std::{collections::HashSet, process::exit, str::Lines};

use itertools::{Chunk, Itertools};

// In the first compartment, build a set of encountered items. In the second
// compartment, check for items previously seen in the first compartment.
pub fn part1(input: String) -> u32 {
    let rucksacks = input.lines();

    let mut duplicate_priorities = 0;
    for rucksack in rucksacks {
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
                break;
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
    let groups = input.lines().chunks(3);

    let mut badge_priorities = 0;
    for mut group in &groups {
        // Build a set out of each group member's rucksack items.
        let first_rucksack = next_rucksack_set(&mut group);
        let second_rucksack = next_rucksack_set(&mut group);
        let third_rucksack = next_rucksack_set(&mut group);
        assert_eq!(group.next(), None);

        // Find the intersection of all rucksacks.
        let common: HashSet<char> = first_rucksack
            .intersection(&second_rucksack)
            .copied()
            .collect::<HashSet<char>>()
            .intersection(&third_rucksack)
            .copied()
            .collect();
        assert_eq!(common.len(), 1);
        let badge = common
            .iter()
            .exactly_one()
            .copied()
            .unwrap_or_else(|err| -> char {
                println!("Group did not contain exactly one badge: {}", err);
                exit(1)
            });
        badge_priorities += item_priority(badge)
    }

    badge_priorities
}

fn next_rucksack_set(group: &mut Chunk<Lines>) -> HashSet<char> {
    let rucksack = group.next().unwrap_or_else(|| -> &str {
        println!("Impossible: group did not contain three rucksacks");
        exit(1)
    });
    rucksack.chars().collect()
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
