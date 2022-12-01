use itertools::Itertools;

pub fn max_calories_carried(calories: Vec<Vec<usize>>) -> usize {
    calories
        .into_iter()
        .map(|elf| elf.iter().sum())
        .max()
        .unwrap()
}

pub fn total_calories_carried_by_top_three_elves(calories: Vec<Vec<usize>>) -> usize {
    calories
        .into_iter()
        .map(|elf| elf.iter().sum::<usize>())
        .sorted_by(|a, b| a.cmp(b).reverse())
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use crate::testing_utils::{data_sample_string, LineGroups};

    use super::*;

    fn short_data_set() -> Vec<Vec<usize>> {
        vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ]
    }

    fn long_data_set() -> Vec<Vec<usize>> {
        data_sample_string("day_1.txt")
            .line_groups()
            .iter()
            .map(|elf| elf.iter().map(|line| line.parse().unwrap()).collect())
            .collect()
    }

    #[test]
    fn part_1_short() {
        assert_that(&max_calories_carried(short_data_set())).is_equal_to(24000);
    }

    #[test]
    fn part_1_long() {
        assert_that(&max_calories_carried(long_data_set())).is_equal_to(69528);
    }

    #[test]
    fn part_2_short() {
        assert_that(&total_calories_carried_by_top_three_elves(short_data_set()))
            .is_equal_to(45000);
    }

    #[test]
    fn part_2_long() {
        assert_that(&total_calories_carried_by_top_three_elves(long_data_set()))
            .is_equal_to(206152);
    }
}
