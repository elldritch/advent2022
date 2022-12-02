use std::process::exit;

enum Move {
    Rock,
    Paper,
    Scissors,
}

use Move::*;

enum Outcome {
    Loss,
    Draw,
    Win,
}

use Outcome::*;

pub fn part1(input: String) -> u32 {
    let lines = input.lines();

    let mut score = 0;
    for line in lines {
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
        let player = {
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

        score += score_of_round(opponent, player);
    }

    score
}

pub fn part2(input: String) -> u32 {
    let lines = input.lines();

    let mut score = 0;
    for line in lines {
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
        let outcome = {
            let player_move = chars.next().unwrap_or_else(|| -> char {
                println!("Impossible: line did not contain player move");
                exit(1)
            });
            match player_move {
                'X' => Loss,
                'Y' => Draw,
                'Z' => Win,
                _ => {
                    println!("Invalid player move: {}", player_move);
                    exit(1)
                }
            }
        };
        assert_eq!(chars.next(), None);

        let player = {
            match (&opponent, outcome) {
                (Rock, Loss) => Scissors,
                (Rock, Draw) => Rock,
                (Rock, Win) => Paper,
                (Paper, Loss) => Rock,
                (Paper, Draw) => Paper,
                (Paper, Win) => Scissors,
                (Scissors, Loss) => Paper,
                (Scissors, Draw) => Scissors,
                (Scissors, Win) => Rock,
            }
        };

        score += score_of_round(opponent, player);
    }

    score
}

fn score_of_round(opponent: Move, player: Move) -> u32 {
    let score_from_selected: u32 = match player {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    let score_from_outcome: u32 = match (opponent, player) {
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

    score_from_selected + score_from_outcome
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.into()), 12)
    }
}
