use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    process::exit,
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

pub fn part1(input: String) -> usize {
    let mut monkeys = super::shared::must_parse(parse, input.as_str());
    let monkeys_ptr = &mut monkeys as *mut Vec<Monkey>;
    let num_rounds = 20;

    // Initialize counts.
    let mut monkey_inspection_counts: HashMap<usize, usize> = HashMap::new();
    for i in 0..monkeys.len() {
        monkey_inspection_counts.insert(i, 0);
    }

    // Simulate rounds.
    for _ in 0..num_rounds {
        for (monkey_id, monkey) in monkeys.iter_mut().enumerate() {
            let op = &monkey.operation;
            while let Some(item) = monkey.items.pop_front() {
                let new_worry_level = op(item) / 3;
                let target_monkey_index = if new_worry_level % monkey.divisibility_test == 0 {
                    monkey.true_monkey
                } else {
                    monkey.false_monkey
                };
                match monkey_inspection_counts.get(&monkey_id) {
                    Some(count) => {
                        monkey_inspection_counts.insert(monkey_id, count + 1);
                    }
                    None => {
                        println!(
                            "Impossible: tried to count monkey inspections for non-existent monkey"
                        );
                        exit(1)
                    }
                }
                // We use unsafe block to hold a second mutable reference to the monkeys so we can modify the target
                // monkey while still borrowing the current monkey (because we are in the current monkey's scope to
                // iterate over its items).
                unsafe {
                    let monkeys_unsafe = &mut *monkeys_ptr;
                    let target_monkey = &mut monkeys_unsafe[target_monkey_index as usize];
                    target_monkey.items.push_back(new_worry_level);
                }
            }
        }
    }

    monkey_inspection_counts
        .values()
        .sorted()
        .rev()
        .take(2)
        .product()
}

pub fn part2(_input: String) -> u32 {
    todo!()
}

struct Monkey {
    items: VecDeque<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
    divisibility_test: u32,
    true_monkey: u32,
    false_monkey: u32,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("divisibility_test", &self.divisibility_test)
            .field("true_monkey", &self.true_monkey)
            .field("false_monkey", &self.false_monkey)
            .finish()
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(newline, parse_monkey)(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = delimited(tag("Monkey "), u32, tag(":\n"))(input)?;
    let (input, items) = delimited(
        tag("  Starting items: "),
        map(separated_list1(tag(", "), u32), |xs| {
            xs.into_iter().collect()
        }),
        newline,
    )(input)?;
    let (input, operation) = delimited(
        tag("  Operation: new = old "),
        alt((
            map(preceded(tag("* "), u32), |x: u32| {
                Box::new(move |old: u32| old * x) as Box<dyn Fn(u32) -> u32>
            }),
            map(preceded(tag("+ "), u32), |x: u32| {
                Box::new(move |old: u32| old + x) as Box<dyn Fn(u32) -> u32>
            }),
            map(tag("* old"), |_| {
                Box::new(|old: u32| old * old) as Box<dyn Fn(u32) -> u32>
            }),
        )),
        newline,
    )(input)?;
    let (input, divisibility_test) = delimited(tag("  Test: divisible by "), u32, newline)(input)?;
    let (input, true_monkey) =
        delimited(tag("    If true: throw to monkey "), u32, newline)(input)?;
    let (input, false_monkey) =
        delimited(tag("    If false: throw to monkey "), u32, newline)(input)?;

    Ok((
        input,
        Monkey {
            items,
            operation,
            divisibility_test,
            true_monkey,
            false_monkey,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.into()), 10605)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.into()), 0)
    }
}
