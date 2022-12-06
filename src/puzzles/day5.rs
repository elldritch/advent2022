use std::{collections::VecDeque, process::exit};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, newline, u32, u8},
    combinator::map,
    error::ParseError,
    multi::{many1, many_till, separated_list1},
    sequence::{delimited, pair, terminated, tuple},
    IResult, InputLength, Parser,
};

pub fn part1(input: String) -> String {
    let (mut crates, steps) = match parse_puzzle(input.as_str()) {
        Ok(("", parsed)) => parsed,
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
    };

    for step in steps {
        for _ in 0..step.quantity {
            let label = match crates[step.from].pop_back() {
                Some(label) => label,
                None => {
                    println!("Impossible: steps resulted in invalid state: {:?}", crates);
                    exit(1);
                }
            };
            crates[step.to].push_back(label);
        }
    }

    let mut message = String::new();
    for column in &crates {
        match column.back() {
            Some(label) => message.push(*label),
            None => {
                println!("Impossible: steps resulted in invalid state: {:?}", crates);
                exit(1);
            }
        }
    }

    message
}

fn parse_puzzle(input: &str) -> IResult<&str, (Vec<VecDeque<char>>, Vec<Step>)> {
    // Parse crate layout.
    let (input, first_row) = row_of_crates(input)?;
    let num_crates = first_row.len();
    let (input, (rows, _)) =
        many_till(row_of_crates_n(num_crates), column_numbers(num_crates))(input)?;

    // Construct crate layout.
    let mut crates: Vec<VecDeque<char>> = Vec::new();
    // Put together the column vector using the first row.
    for crate_ in first_row {
        let mut column: VecDeque<char> = VecDeque::new();
        match crate_ {
            Some(label) => column.push_front(label),
            None => {}
        }
        crates.push(column)
    }
    // Push the other crates into place.
    for row in rows {
        for (i, crate_) in row.iter().enumerate() {
            match crate_ {
                Some(label) => crates[i].push_front(*label),
                None => {}
            }
        }
    }

    // Parse empty line separator.
    let (input, _) = newline(input)?;

    // Parse steps.
    let (input, steps) = many1(step)(input)?;

    Ok((input, (crates, steps)))
}

fn step(input: &str) -> IResult<&str, Step> {
    map(
        tuple((
            tag("move "),
            u32,
            tag(" from "),
            u32,
            tag(" to "),
            u32,
            newline,
        )),
        |(_, quantity, _, from, _, to, _)| Step {
            quantity,
            from: (from - 1) as usize,
            to: (to - 1) as usize,
        },
    )(input)
}

fn row_of_crates(input: &str) -> IResult<&str, Vec<Option<char>>> {
    terminated(separated_list1(char(' '), maybe_crate), newline)(input)
}

fn separated_listn<I, O, O2, E, F, G>(
    sep: G,
    f: F,
    count: usize,
) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    I: Clone + InputLength + PartialEq,
    F: Parser<I, O, E> + Clone,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    let f2 = f.clone();
    map(
        pair(nom::multi::count(terminated(f, sep), count - 1), f2),
        |(mut xs, last): (Vec<O>, O)| -> Vec<O> {
            xs.push(last);
            xs
        },
    )
}

fn row_of_crates_n<'a>(
    num_crates: usize,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Option<char>>> {
    terminated(separated_listn(char(' '), maybe_crate, num_crates), newline)
}

fn maybe_crate(input: &str) -> IResult<&str, Option<char>> {
    let crate_found = map(delimited(char('['), anychar, char(']')), |c| Some(c));
    let crate_missing = map(tag("   "), |_| None);
    alt((crate_found, crate_missing))(input)
}

fn column_numbers<'a>(num_crates: usize) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<u8>> {
    terminated(
        separated_listn(char(' '), column_number, num_crates),
        newline,
    )
}

fn column_number(input: &str) -> IResult<&str, u8> {
    delimited(char(' '), u8, char(' '))(input)
}

pub fn part2(input: String) -> String {
    todo!("not yet implemented")
}

#[derive(Debug)]
struct Step {
    quantity: u32,
    from: usize,
    to: usize,
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
