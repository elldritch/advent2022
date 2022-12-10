use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::map,
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

pub fn part1(input: String) -> i32 {
    let instructions = super::shared::must_parse(parse, input.as_str());

    let mut state = CPU { cycle: 1, x: 1 };
    let states = instructions
        .iter()
        .flat_map(|instruction| match instruction {
            NoOp => {
                state.cycle += 1;
                vec![state.clone()]
            }
            AddX(x) => {
                state.cycle += 1;
                let c1_state = state.clone();
                state.cycle += 1;
                state.x += x;
                let c2_state = state.clone();
                vec![c1_state, c2_state]
            }
        });

    let signals_of_interest = states.filter(|state| {
        state.cycle == 20
            || state.cycle == 60
            || state.cycle == 100
            || state.cycle == 140
            || state.cycle == 180
            || state.cycle == 220
    });

    signals_of_interest.map(|CPU { cycle, x }| cycle * x).sum()
}

pub fn part2(input: String) -> u32 {
    todo!()
}

#[derive(Debug, Clone)]
struct CPU {
    cycle: i32,
    x: i32,
}

#[derive(Debug)]
enum Instruction {
    NoOp,
    AddX(i32),
}
use Instruction::*;

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(terminated(
        alt((
            map(tag("noop"), |_| NoOp),
            map(preceded(tag("addx "), i32), |x| AddX(x)),
        )),
        newline,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.into()), 13140)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.into()), 0)
    }
}
