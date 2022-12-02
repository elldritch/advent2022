use std::process::exit;

enum Move {
    Rock,
    Paper,
    Scissors,
}

use Move::*;

pub fn part1(input: String) -> u32 {
    let lines = input.lines();

    let mut score = 0;
    for line in lines {
        // Parse input line.
        let mut chars = line.chars();
        let opponent = {
            let opponent_move = chars.next().unwrap_or_else(|| -> char {
                println!("Impossible: line did not contain opponent move");
                exit(1)
            });
            match opponent_move {
                'A' => Rock,
                'B' => Paper,
                'C' => Scissors,
                _ => {
                    println!("Invalid opponent move: {}", opponent_move);
                    exit(1)
                }
            }
        };
        assert_eq!(chars.next(), Some(' '));
        let mine = {
            let player_move = chars.next().unwrap_or_else(|| -> char {
                println!("Impossible: line did not contain player move");
                exit(1)
            });
            match player_move {
                'X' => Rock,
                'Y' => Paper,
                'Z' => Scissors,
                _ => {
                    println!("Invalid player move: {}", player_move);
                    exit(1)
                }
            }
        };
        assert_eq!(chars.next(), None);

        let score_from_selected: u32 = match mine {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };

        let score_from_outcome: u32 = match (opponent, mine) {
            (Rock, Rock) => 3,
            (Rock, Paper) => 6,
            (Rock, Scissors) => 0,
            (Paper, Rock) => 0,
            (Paper, Paper) => 3,
            (Paper, Scissors) => 6,
            (Scissors, Rock) => 6,
            (Scissors, Paper) => 0,
            (Scissors, Scissors) => 3,
        };

        score += score_from_selected + score_from_outcome;
    }

    score
}

pub fn part2(input: String) {
    panic!("not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.into()), 15)
    }
}
