pub fn part_1(pairs: Vec<Pair>) -> usize {
    pairs
        .into_iter()
        .map(|pair| pair.0.contains(&pair.1) || pair.1.contains(&pair.0))
        .filter(|res| *res)
        .count()
}

pub fn part_2(pairs: Vec<Pair>) -> usize {
    pairs
        .into_iter()
        .map(|pair| pair.0.overlaps(&pair.1))
        .filter(|res| *res)
        .count()
}

#[derive(Debug)]
pub struct Assignment(pub usize, pub usize);

impl Assignment {
    fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn spans(&self, point: usize) -> bool {
        self.0 <= point && self.1 >= point
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.spans(other.0) || self.spans(other.1) || self.contains(other) || other.contains(self)
    }
}

#[derive(Debug)]
pub struct Pair(pub Assignment, pub Assignment);

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use crate::testing_utils::data_sample_string;

    use super::*;

    type Data = Vec<Pair>;

    fn split_pair(line: &str) -> (&str, &str) {
        line.split_once(',').unwrap()
    }

    fn to_assignment(s: &str) -> Assignment {
        let (first, second) = s.split_once('-').unwrap();
        Assignment(first.parse().unwrap(), second.parse().unwrap())
    }

    fn short_data_set() -> Data {
        "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"
            .lines()
            .map(split_pair)
            .map(|(first, second)| Pair(to_assignment(first), to_assignment(second)))
            .collect()
    }

    fn long_data_set() -> Data {
        data_sample_string("day_4.txt")
            .lines()
            .map(split_pair)
            .map(|(first, second)| Pair(to_assignment(first), to_assignment(second)))
            .collect()
    }

    #[test]
    fn part_1_short() {
        assert_that(&part_1(short_data_set())).is_equal_to(2)
    }

    #[test]
    fn part_1_long() {
        assert_that(&part_1(long_data_set())).is_equal_to(490)
    }

    #[test]
    fn part_2_short() {
        assert_that(&part_2(short_data_set())).is_equal_to(4)
    }

    #[test]
    fn part_2_long() {
        assert_that(&part_2(long_data_set())).is_equal_to(921)
    }
}
