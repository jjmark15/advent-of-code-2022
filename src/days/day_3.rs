use std::collections::HashSet;
use std::hash::Hash;

pub fn part_1(backpacks: Vec<Backpack>) -> usize {
    backpacks
        .into_iter()
        .map(|(first, second)| priority(common_item(first, second)))
        .sum()
}

pub fn part_2(backpacks: Vec<Backpack>) -> usize {
    grouped(backpacks)
        .into_iter()
        .map(|group| priority(identify_badge(group)))
        .sum()
}

fn common_item(first: Vec<char>, second: Vec<char>) -> char {
    let first_set = to_set(first);
    let second_set = to_set(second);
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

fn grouped(backpacks: Vec<Backpack>) -> Vec<Vec<Backpack>> {
    backpacks
        .into_iter()
        .enumerate()
        .fold(vec![], |mut acc, (index, backpack)| {
            if index % 3 == 0 {
                acc.push(vec![backpack])
            } else {
                acc.last_mut().unwrap().push(backpack)
            }

            acc
        })
}

fn concatenate((first, second): Backpack) -> Vec<char> {
    first.into_iter().chain(second).collect()
}

fn identify_badge(group: Vec<Backpack>) -> char {
    let sets = group
        .into_iter()
        .map(concatenate)
        .map(to_set)
        .collect::<Vec<HashSet<char>>>();

    *sets
        .first()
        .unwrap()
        .intersection(sets.get(1).unwrap())
        .into_iter()
        .cloned()
        .collect::<HashSet<char>>()
        .intersection(sets.get(2).unwrap())
        .min()
        .unwrap()
}

fn to_set<C: IntoIterator<Item = T>, T: Eq + Hash>(v: C) -> HashSet<T> {
    v.into_iter().collect()
}

type Backpack = (Vec<char>, Vec<char>);

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

    #[test]
    fn part_2_short() {
        assert_that(&part_2(short_data_set())).is_equal_to(70)
    }

    #[test]
    fn part_2_long() {
        assert_that(&part_2(long_data_set())).is_equal_to(2577)
    }
}
