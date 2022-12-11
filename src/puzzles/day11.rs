use std::{
    cmp::Reverse,
    collections::{HashMap, VecDeque},
    fmt::Debug,
    ops::{Add, Mul},
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
    let monkeys: Vec<Monkey<u32>> = super::shared::must_parse(parse, input.as_str());
    simulate_monkey_business::<u32>(monkeys, 20, |x| x / 3)
}

pub fn part2(input: String) -> usize {
    let monkeys: Vec<Monkey<u32>> = super::shared::must_parse(parse, input.as_str());
    let moduli: Vec<u32> = monkeys
        .iter()
        .map(|monkey| monkey.divisibility_test)
        .collect();
    let residue_monkeys = monkeys
        .into_iter()
        .map(
            |Monkey {
                 items,
                 operation,
                 divisibility_test,
                 true_monkey,
                 false_monkey,
             }|
             -> Monkey<ResidueNumber> {
                let items = items
                    .into_iter()
                    .map(|worry_level| ResidueNumber::new(moduli.as_slice(), worry_level))
                    .collect();
                // Can't use update syntax yet :( https://github.com/rust-lang/rust/issues/86555
                Monkey {
                    items,
                    operation,
                    divisibility_test,
                    true_monkey,
                    false_monkey,
                }
            },
        )
        .collect();
    simulate_monkey_business::<ResidueNumber>(residue_monkeys, 10000, |x| x)
}

fn simulate_monkey_business<T>(
    mut monkeys: Vec<Monkey<T>>,
    rounds: usize,
    update_bored_worry: fn(T) -> T,
) -> usize
where
    T: Add<u32, Output = T>
        + Mul<u32, Output = T>
        + Mul<T, Output = T>
        + TryDivisibleBy<u32>
        + Clone,
{
    let monkeys_ptr = &mut monkeys as *mut Vec<Monkey<_>>;

    // Initialize counts.
    let mut monkey_inspection_counts: HashMap<usize, usize> = HashMap::new();
    for i in 0..monkeys.len() {
        monkey_inspection_counts.insert(i, 0);
    }

    // Simulate rounds.
    for _ in 0..rounds {
        for (monkey_id, monkey) in monkeys.iter_mut().enumerate() {
            while let Some(item) = monkey.items.pop_front() {
                let inspection_worry = match monkey.operation {
                    Operation::Add(x) => item + x,
                    Operation::Mul(x) => item * x,
                    Operation::Square => item.clone() * item,
                };
                let bored_worry = update_bored_worry(inspection_worry);
                let target_monkey_index = if let Some(true) =
                    bored_worry.clone().divisible_by(monkey.divisibility_test)
                {
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
                // We use unsafe block to hold a second mutable reference to the
                // monkeys so we can modify the target monkey while still
                // borrowing the current monkey (because we are in the current
                // monkey's scope to iterate over its items).
                unsafe {
                    let monkeys_unsafe = &mut *monkeys_ptr;
                    let target_monkey = &mut monkeys_unsafe[target_monkey_index];
                    target_monkey.items.push_back(bored_worry);
                }
            }
        }
    }

    monkey_inspection_counts
        .values()
        .map(Reverse)
        .k_smallest(2)
        .map(|n| n.0)
        .product()
}

struct Monkey<T> {
    items: VecDeque<T>,
    operation: Operation,
    divisibility_test: u32,
    true_monkey: usize,
    false_monkey: usize,
}

enum Operation {
    Add(u32),
    Mul(u32),
    Square,
}

impl<T> Debug for Monkey<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("divisibility_test", &self.divisibility_test)
            .field("true_monkey", &self.true_monkey)
            .field("false_monkey", &self.false_monkey)
            .finish()
    }
}

trait TryDivisibleBy<Rhs = Self> {
    fn divisible_by(self, rhs: Rhs) -> Option<bool>;
}

impl TryDivisibleBy for u32 {
    fn divisible_by(self, rhs: Self) -> Option<bool> {
        Some(self % rhs == 0)
    }
}

#[derive(Debug, Clone)]
struct ResidueNumber {
    residues: HashMap<u32, u32>,
}

impl ResidueNumber {
    fn new(moduli: &[u32], x: u32) -> ResidueNumber {
        let mut n = ResidueNumber {
            residues: HashMap::new(),
        };
        for modulus in moduli {
            n.residues.insert(*modulus, x % modulus);
        }
        modulate(&mut n);
        n
    }

    fn moduli(&self) -> Vec<u32> {
        self.residues.keys().copied().sorted().collect()
    }
}

fn modulate(n: &mut ResidueNumber) {
    let moduli = n.moduli();
    for modulus in moduli {
        let v = n.residues.get(&modulus).unwrap();
        n.residues.insert(modulus, v % modulus);
    }
}

fn binop(f: fn(u32, u32) -> u32, lhs: &ResidueNumber, rhs: &ResidueNumber) -> ResidueNumber {
    let moduli = lhs.moduli();
    if moduli != rhs.moduli() {
        panic!("cannot operate on residue numbers with different moduli");
    }

    let mut out = ResidueNumber::new(moduli.as_slice(), 0);
    for modulus in moduli {
        if let (Some(l), Some(r)) = (lhs.residues.get(&modulus), rhs.residues.get(&modulus)) {
            out.residues.insert(modulus, f(*l, *r));
        } else {
            panic!("impossible: residue numbers with same moduli have inconsistent residue keys")
        }
    }
    modulate(&mut out);
    out
}

impl Add for ResidueNumber {
    type Output = ResidueNumber;

    fn add(self, rhs: Self) -> Self::Output {
        binop(Add::add, &self, &rhs)
    }
}

impl Add<u32> for ResidueNumber {
    type Output = ResidueNumber;

    fn add(self, rhs: u32) -> Self::Output {
        let moduli = self.moduli();
        self.add(Self::new(moduli.as_slice(), rhs))
    }
}

impl Mul for ResidueNumber {
    type Output = ResidueNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        binop(Mul::mul, &self, &rhs)
    }
}

impl Mul<u32> for ResidueNumber {
    type Output = ResidueNumber;

    fn mul(self, rhs: u32) -> Self::Output {
        let moduli = self.moduli();
        self.mul(Self::new(moduli.as_slice(), rhs))
    }
}

impl PartialEq for ResidueNumber {
    fn eq(&self, other: &Self) -> bool {
        self.residues == other.residues
    }
}

// Residue numbers can only be tested for divisibility against one of their
// moduli.
//
// We could implement this for numbers in 0 <= n <= M where M is the product of
// all moduli if the moduli are pairwise co-prime using the Chinese Remainder
// Theorem.
impl TryDivisibleBy<u32> for ResidueNumber {
    fn divisible_by(self, rhs: u32) -> Option<bool> {
        if let Some(n) = self.residues.get(&rhs) {
            Some(*n == 0)
        } else {
            None
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Monkey<u32>>> {
    separated_list1(newline, parse_monkey)(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey<u32>> {
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
            map(preceded(tag("+ "), u32), |x: u32| Operation::Add(x)),
            map(preceded(tag("* "), u32), |x: u32| Operation::Mul(x)),
            map(tag("* old"), |_| Operation::Square),
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
            true_monkey: true_monkey as usize,
            false_monkey: false_monkey as usize,
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
        assert_eq!(part2(EXAMPLE_INPUT.into()), 2713310158)
    }
}
