use crate::day_2::Move::{Paper, Rock, Scissors};
use crate::day_2::RoundResult::{Draw, Lose, Win};

#[derive(Eq, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn points(&self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

fn to_move(symbol: char) -> Move {
    match symbol {
        'A' | 'X' => Rock,
        'B' | 'Y' => Paper,
        'C' | 'Z' => Scissors,
        _ => panic!("unrecognised symbol"),
    }
}

enum RoundResult {
    Win,
    Draw,
    Lose,
}

impl RoundResult {
    fn points(&self) -> usize {
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

pub fn score_following_guide_part_1(guide: Vec<(char, char)>) -> usize {
    guide
        .into_iter()
        .map(|(m1, m2)| (to_move(m1), to_move(m2)))
        .map(|(their_move, my_move)| {
            round_result(&my_move, &their_move).points() + my_move.points()
        })
        .sum()
}

pub fn score_following_guide_part_2(guide: Vec<(char, char)>) -> usize {
    guide
        .into_iter()
        .map(|(their_move, result)| (to_move(their_move), to_result(result)))
        .map(|(their_move, round_result)| {
            let my_move = my_move_for_result(&their_move, &round_result);
            (their_move, my_move)
        })
        .map(|(their_move, my_move)| {
            round_result(&my_move, &their_move).points() + my_move.points()
        })
        .sum()
}

fn my_move_for_result(their_move: &Move, round_result: &RoundResult) -> Move {
    match their_move {
        Rock => match round_result {
            Win => Paper,
            Draw => Rock,
            Lose => Scissors,
        },
        Paper => match round_result {
            Win => Scissors,
            Draw => Paper,
            Lose => Rock,
        },
        Scissors => match round_result {
            Win => Rock,
            Draw => Scissors,
            Lose => Paper,
        },
    }
}

fn round_result(my_move: &Move, their_move: &Move) -> RoundResult {
    match my_move {
        Rock => match their_move {
            Rock => Draw,
            Paper => Lose,
            Scissors => Win,
        },
        Paper => match their_move {
            Rock => Win,
            Paper => Draw,
            Scissors => Lose,
        },
        Scissors => match their_move {
            Rock => Lose,
            Paper => Win,
            Scissors => Draw,
        },
    }
}

fn to_result(symbol: char) -> RoundResult {
    match symbol {
        'X' => Lose,
        'Y' => Draw,
        'Z' => Win,
        _ => panic!("unrecognised symbol"),
    }
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use crate::testing_utils::data_sample_string;

    use super::*;

    fn short_data_set() -> Vec<(char, char)> {
        vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')]
    }

    fn long_data_set() -> Vec<(char, char)> {
        data_sample_string("day_2.txt")
            .lines()
            .map(|line| {
                let mut chars = line.chars();
                (chars.next().unwrap(), chars.nth(1).unwrap())
            })
            .collect()
    }

    #[test]
    fn part_1_short() {
        assert_that(&score_following_guide_part_1(short_data_set())).is_equal_to(15)
    }

    #[test]
    fn part_1_long() {
        assert_that(&score_following_guide_part_1(long_data_set())).is_equal_to(15337)
    }

    #[test]
    fn part_2_short() {
        assert_that(&score_following_guide_part_2(short_data_set())).is_equal_to(12)
    }

    #[test]
    fn part_2_long() {
        assert_that(&score_following_guide_part_2(long_data_set())).is_equal_to(11696)
    }
}
