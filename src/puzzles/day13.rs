use std::{
    cmp::{Ordering, Ordering::*},
    process::exit,
};

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, newline, u32},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, terminated},
    IResult,
};

pub fn part1(input: String) -> usize {
    let pairs = super::shared::must_parse(parse, input.as_str());

    pairs
        .into_iter()
        .map(|(left, right)| {
            left.partial_cmp(&right).unwrap_or_else(|| {
                println!("Impossible: packet pair has ambiguous ordering: {left:?} {right:?}");
                exit(1)
            }) == Less
        })
        .enumerate()
        .filter(|(_, in_order)| *in_order)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(input: String) -> usize {
    let divider1 = List(vec![List(vec![Int(2)])]);
    let divider2 = List(vec![List(vec![Int(6)])]);
    let pairs = super::shared::must_parse(parse, input.as_str());
    let packets = {
        let mut packets: Vec<Packet> = pairs.into_iter().flat_map(|(a, b)| [a, b]).collect();
        packets.push(divider1.clone());
        packets.push(divider2.clone());
        packets
    };
    packets
        .into_iter()
        .sorted_by(|l, r| {
            l.partial_cmp(r).unwrap_or_else(|| {
                println!("Impossible: packets have ambiguous ordering: {l:?} {r:?}");
                exit(1)
            })
        })
        .enumerate()
        .filter(|(_, p)| *p == divider1 || *p == divider2)
        .map(|(i, _)| i + 1)
        .product()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

use Packet::*;

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Int(l), List(_)) => Packet::partial_cmp(&List(vec![Int(*l)]), other),
            (List(_), Int(r)) => Packet::partial_cmp(self, &List(vec![Int(*r)])),
            (Int(l), Int(r)) => match l.cmp(&r) {
                Equal => None,
                c => Some(c),
            },
            (List(l), List(r)) => {
                for i in 0.. {
                    match (l.get(i), r.get(i)) {
                        (Some(x), Some(y)) => match Packet::partial_cmp(x, y) {
                            Some(result) => return Some(result),
                            None => continue,
                        },
                        (None, Some(_)) => return Some(Less),
                        (Some(_), None) => return Some(Greater),
                        (None, None) => return None,
                    }
                }
                panic!("impossible: list matching loop completed without reaching empty case")
            }
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(newline, pair(packet_outer, packet_outer))(input)
}

fn packet_outer(input: &str) -> IResult<&str, Packet> {
    terminated(packet_list, newline)(input)
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((map(u32, Int), packet_list))(input)
}

fn packet_list(input: &str) -> IResult<&str, Packet> {
    map(
        delimited(char('['), separated_list0(char(','), packet), char(']')),
        List,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.into()), 13)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.into()), 140)
    }
}
