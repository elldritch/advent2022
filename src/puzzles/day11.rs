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
    character::complete::{newline, u16},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

pub fn part1(input: String) -> usize {
    let monkeys: Vec<Monkey<u16>> = must_parse(From::from, input.as_str());
    simulate_monkey_business::<u16>(monkeys, 20, |x| x / 3)
}

pub fn part2(input: String) -> usize {
    let moduli = [0; 9];
    let monkeys: Vec<Monkey<ResidueNumber>> =
        must_parse(ResidueNumber::make_from_moduli(&moduli), input.as_str());
    simulate_monkey_business::<ResidueNumber>(monkeys, 10000, |x| x)
}

fn must_parse<F, T>(parse_number: F, input: &str) -> Vec<Monkey<T>>
where
    T: Add<Output = T> + Mul<Output = T> + Clone,
    F: Fn(u16) -> T + Clone + 'static,
{
    match parse(input, parse_number) {
        Ok(("", pairs)) => pairs,
        Ok((remaining, _)) => {
            println!(
                "Invalid puzzle input: could not parse input suffix: {}",
                remaining
            );
            exit(1)
        }
        Err(err) => {
            println!("Could not parse puzzle input: {}", err);
            exit(1)
        }
    }
}

fn simulate_monkey_business<T>(
    mut monkeys: Vec<Monkey<T>>,
    rounds: usize,
    update_worry: fn(T) -> T,
) -> usize
where
    T: TryDivisibleBy<u16> + Clone,
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
            let op = &monkey.operation;
            while let Some(item) = monkey.items.pop_front() {
                let new_worry_level = update_worry(op(item));
                let target_monkey_index = if let Some(true) = new_worry_level
                    .clone()
                    .divisible_by(monkey.divisibility_test)
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
                    target_monkey.items.push_back(new_worry_level);
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
    operation: Box<dyn Fn(T) -> T>,
    divisibility_test: u16,
    true_monkey: usize,
    false_monkey: usize,
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

impl TryDivisibleBy for u16 {
    fn divisible_by(self, rhs: Self) -> Option<bool> {
        Some(self % rhs == 0)
    }
}

// This residue number is hard-coded to the moduli in my puzzle input.
//
// TODO: Is there a way to construct this type so that we can pick moduli at
// runtime, based on the input? We could store the moduli as a HashMap. But how
// would we implement From<u16>? The type signature doesn't let us pass in
// moduli, and I don't think we can reify a term list out of a type parameter
// because Rust has no Proxy-like mechanism.
//
// Maybe instead of using From<u16> we should pass in the `u16 -> T` function as
// a parameter, just using From to provide the implementation where convenient.
#[derive(Debug, Clone)]
struct ResidueNumber {
    // Moduli: 2, 3, 5, 7, 11, 13, 17, 19, 23
    residues: [u32; RESIDUE_MODULI_LEN],
}

const RESIDUE_MODULI_LEN: usize = 9;
const RESIDUE_MODULI: [u32; RESIDUE_MODULI_LEN] = [2, 3, 5, 7, 11, 13, 17, 19, 23];

impl ResidueNumber {
    fn new(x: u16) -> ResidueNumber {
        let mut n = ResidueNumber {
            residues: [x as u32; RESIDUE_MODULI_LEN],
        };
        modulate(&mut n);
        n
    }

    fn from_moduli(moduli: &[u32], n: u16) -> ResidueNumber {
        todo!()
    }

    fn make_from_moduli(moduli: &[u32]) -> impl Fn(u16) -> ResidueNumber + Clone {
        |n| ResidueNumber { residues: [0; 9] }
    }
}

fn modulate(n: &mut ResidueNumber) {
    for i in 0..RESIDUE_MODULI_LEN {
        n.residues[i] = n.residues[i] % RESIDUE_MODULI[i];
    }
}

fn binop(f: fn(u32, u32) -> u32, lhs: &ResidueNumber, rhs: &ResidueNumber) -> ResidueNumber {
    let mut out = ResidueNumber::new(0);
    for i in 0..RESIDUE_MODULI_LEN {
        out.residues[i] = f(lhs.residues[i], rhs.residues[i]);
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

impl Mul for ResidueNumber {
    type Output = ResidueNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        binop(Mul::mul, &self, &rhs)
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
impl TryDivisibleBy<u16> for ResidueNumber {
    fn divisible_by(self, rhs: u16) -> Option<bool> {
        for (i, m) in RESIDUE_MODULI.into_iter().enumerate() {
            if m as u16 == rhs {
                return Some(self.residues[i] == 0);
            }
        }
        None
    }
}

fn parse<T, F>(input: &str, parse_number: F) -> IResult<&str, Vec<Monkey<T>>>
where
    T: Add<Output = T> + Mul<Output = T> + Clone,
    F: Fn(u16) -> T + Clone + 'static,
{
    separated_list1(newline, parse_monkey(parse_number))(input)
}

fn parse_monkey<T, F>(parse_number: F) -> impl Fn(&str) -> IResult<&str, Monkey<T>>
where
    T: Add<Output = T> + Mul<Output = T> + Clone,
    F: Fn(u16) -> T + Clone + 'static,
{
    move |input| {
        let (input, _) = delimited(tag("Monkey "), u16, tag(":\n"))(input)?;
        let (input, items) = delimited(
            tag("  Starting items: "),
            map(separated_list1(tag(", "), u16), |xs| {
                let f = parse_number.clone();
                xs.into_iter().map(f).collect()
            }),
            newline,
        )(input)?;
        let (input, operation) = delimited(
            tag("  Operation: new = old "),
            alt((
                map(preceded(tag("* "), u16), |x: u16| {
                    let f = parse_number.clone();
                    Box::new(move |old: T| old * f(x)) as Box<dyn Fn(T) -> T>
                }),
                map(preceded(tag("+ "), u16), |x: u16| {
                    let f = parse_number.clone();
                    Box::new(move |old: T| old + f(x)) as Box<dyn Fn(T) -> T>
                }),
                map(tag("* old"), |_| {
                    Box::new(|old: T| {
                        let old_clone = old.clone();
                        old * old_clone
                    }) as Box<dyn Fn(T) -> T>
                }),
            )),
            newline,
        )(input)?;
        let (input, divisibility_test) =
            delimited(tag("  Test: divisible by "), u16, newline)(input)?;
        let (input, true_monkey) =
            delimited(tag("    If true: throw to monkey "), u16, newline)(input)?;
        let (input, false_monkey) =
            delimited(tag("    If false: throw to monkey "), u16, newline)(input)?;

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
