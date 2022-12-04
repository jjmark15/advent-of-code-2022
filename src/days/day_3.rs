use std::collections::HashSet;

pub fn part_1(backpacks: Vec<(Vec<char>, Vec<char>)>) -> usize {
    backpacks
        .into_iter()
        .map(|(first, second)| priority(common_item(first, second)))
        .sum()
}

fn common_item(first: Vec<char>, second: Vec<char>) -> char {
    let first_set = first.into_iter().collect::<HashSet<char>>();
    let second_set = second.into_iter().collect::<HashSet<char>>();
    let intersection = first_set.intersection(&second_set);
    *intersection.min().unwrap()
}

fn priority(c: char) -> usize {
    match c as u8 {
        b'a'..=b'z' => 1 + (c as u8 - b'a') as usize,
        b'A'..=b'Z' => 27 + (c as u8 - b'A') as usize,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use crate::testing_utils::data_sample_string;

    use super::*;

    type Data = Vec<(Vec<char>, Vec<char>)>;

    fn halve<T>(list: Vec<T>) -> (Vec<T>, Vec<T>) {
        let length = list.len();
        let mut first = vec![];
        let mut second = vec![];

        for (index, el) in list.into_iter().enumerate() {
            if index < length / 2 {
                first.push(el);
            } else {
                second.push(el);
            }
        }

        (first, second)
    }

    fn short_data_set() -> Data {
        vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
        .into_iter()
        .map(|line| halve(line.chars().collect()))
        .collect()
    }

    fn long_data_set() -> Data {
        data_sample_string("day_3.txt")
            .lines()
            .map(|line| halve(line.chars().collect()))
            .collect()
    }

    #[test]
    fn part_1_short() {
        assert_that(&part_1(short_data_set())).is_equal_to(157)
    }

    #[test]
    fn part_1_long() {
        assert_that(&part_1(long_data_set())).is_equal_to(7826)
    }
}
