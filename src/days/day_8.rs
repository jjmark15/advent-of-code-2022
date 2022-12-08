use itertools::Itertools;

type InputData = Vec<Vec<TreeHeight>>;

pub fn part_1(trees: InputData) -> usize {
    count_visible_trees(trees)
}

fn count_visible_trees(trees: InputData) -> usize {
    let trees = Trees(trees);
    (0..trees.height())
        .cartesian_product(0..trees.width())
        .map(|(row, column)| trees.tree_visible_at(&TreePosition { column, row }))
        .filter(|visible| *visible)
        .count()
}

struct Trees(Vec<Vec<TreeHeight>>);

impl Trees {
    fn tree_visible_at(&self, position: &TreePosition) -> bool {
        self.is_at_edge(position)
            || self.is_visible_in_row(position)
            || self.is_visible_in_column(position)
    }

    fn is_visible_in_row(&self, position: &TreePosition) -> bool {
        let first_positions = (0..position.column).into_iter().map(|column| TreePosition {
            row: position.row,
            column,
        });
        let second_positions = (position.column + 1..self.height())
            .into_iter()
            .map(|column| TreePosition {
                row: position.row,
                column,
            });

        let height = self.height_at(position);

        self.height_is_greater_than_positions(height, first_positions)
            || self.height_is_greater_than_positions(height, second_positions)
    }

    fn is_visible_in_column(&self, position: &TreePosition) -> bool {
        let first_positions = (0..position.row).into_iter().map(|row| TreePosition {
            row,
            column: position.column,
        });
        let second_positions =
            (position.row + 1..self.width())
                .into_iter()
                .map(|row| TreePosition {
                    row,
                    column: position.column,
                });

        let height = self.height_at(position);

        self.height_is_greater_than_positions(height, first_positions)
            || self.height_is_greater_than_positions(height, second_positions)
    }

    fn height_is_greater_than_positions(
        &self,
        height: TreeHeight,
        positions: impl IntoIterator<Item = TreePosition>,
    ) -> bool {
        !positions
            .into_iter()
            .any(|position| self.height_at(&position) >= height)
    }

    fn is_at_edge(&self, position: &TreePosition) -> bool {
        position.column == 0
            || position.column == self.width() - 1
            || position.row == 0
            || position.row == self.height() - 1
    }

    fn height_at(&self, position: &TreePosition) -> TreeHeight {
        *self
            .0
            .get(position.row)
            .unwrap()
            .get(position.column)
            .unwrap()
    }

    fn width(&self) -> usize {
        self.0.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }
}

type TreeHeight = u8;

#[derive(Debug, Copy, Clone)]
struct TreePosition {
    column: usize,
    row: usize,
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use crate::testing_utils::data_sample_string;

    use super::*;

    fn data_set<'a>(lines: impl IntoIterator<Item = &'a str>) -> InputData {
        lines
            .into_iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .collect()
            })
            .collect::<InputData>()
    }

    fn short_data_set() -> InputData {
        data_set(vec!["30373", "25512", "65332", "33549", "35390"])
    }

    fn long_data_set() -> InputData {
        data_set(data_sample_string("day_8.txt").lines())
    }

    #[test]
    fn part_1_short() {
        assert_that(&part_1(short_data_set())).is_equal_to(21)
    }

    #[test]
    fn part_1_long() {
        assert_that(&part_1(long_data_set())).is_equal_to(1695)
    }
}
