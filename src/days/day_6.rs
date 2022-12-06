use std::collections::HashSet;

use itertools::Itertools;

pub fn part_1(signal: String) -> usize {
    get_index_of_marker(signal.as_str())
}

fn get_index_of_marker(signal: &str) -> usize {
    signal
        .chars()
        .tuple_windows()
        .find_position(|(s1, s2, s3, s4)| {
            let set: HashSet<&char> = HashSet::from_iter([s1, s2, s3, s4].into_iter());
            set.len() == 4
        })
        .unwrap()
        .0
        + 4
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use crate::testing_utils::data_sample_string;

    use super::*;

    type Data = String;

    fn short_data_set() -> Vec<(Data, usize)> {
        vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 5),
            ("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 6),
        ]
    }

    fn long_data_set() -> Data {
        data_sample_string("day_6.txt").trim().to_string()
    }

    #[test]
    fn part_1_short() {
        short_data_set()
            .into_iter()
            .for_each(|(data, expectation)| assert_that(&part_1(data)).is_equal_to(expectation))
    }

    #[test]
    fn part_1_long() {
        assert_that(&part_1(long_data_set())).is_equal_to(1848)
    }
}
