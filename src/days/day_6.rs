use itertools::Itertools;

pub fn part_1(signal: String) -> usize {
    index_at_end_of_distinct_window(signal.as_str(), 4)
}

pub fn part_2(signal: String) -> usize {
    index_at_end_of_distinct_window(signal.as_str(), 14)
}

fn index_at_end_of_distinct_window(signal: &str, length: usize) -> usize {
    let chars: Vec<char> = signal.chars().into_iter().collect();
    chars
        .iter()
        .enumerate()
        .find_position(|(index, _)| {
            (*index..index + length)
                .into_iter()
                .filter_map(|window_iter| chars.get(window_iter))
                .duplicates()
                .count()
                == 0
        })
        .unwrap()
        .0
        + length
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

    #[test]
    fn part_2_short() {
        assert_that(&part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string())).is_equal_to(19);
    }

    #[test]
    fn part_2_long() {
        assert_that(&part_2(long_data_set())).is_equal_to(2308)
    }
}
