pub fn max_calories_carried(calories: Vec<Vec<usize>>) -> usize {
    calories
        .into_iter()
        .map(|elf| elf.iter().sum())
        .max()
        .unwrap()
}

pub fn total_calories_carried_by_top_three_elves(calories: Vec<Vec<usize>>) -> usize {
    let mut sums: Vec<usize> = calories.into_iter().map(|elf| elf.iter().sum()).collect();
    sums.sort();
    sums.reverse();
    sums.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use testing_utils::speculoos::assert_that;
    use testing_utils::{data_sample_string, LineGroups};

    use super::*;

    #[test]
    fn part_1() {
        let input: Vec<Vec<usize>> = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        assert_that(&max_calories_carried(input)).is_equal_to(24000);
    }

    #[test]
    fn part_1_full() {
        let input: Vec<Vec<usize>> = data_sample_string("day_1.txt")
            .line_groups()
            .iter()
            .map(|elf| elf.iter().map(|line| line.parse().unwrap()).collect())
            .collect();

        assert_that(&max_calories_carried(input)).is_equal_to(69528);
    }

    #[test]
    fn part_2() {
        let input: Vec<Vec<usize>> = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        assert_that(&total_calories_carried_by_top_three_elves(input)).is_equal_to(45000);
    }

    #[test]
    fn part_2_full() {
        let input: Vec<Vec<usize>> = data_sample_string("day_1.txt")
            .line_groups()
            .iter()
            .map(|elf| elf.iter().map(|line| line.parse().unwrap()).collect())
            .collect();

        assert_that(&total_calories_carried_by_top_three_elves(input)).is_equal_to(206152);
    }
}
