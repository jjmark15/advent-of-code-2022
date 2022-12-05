pub fn part_1(stacks: Vec<Stack>, instructions: Vec<Instruction>) -> Vec<Crate> {
    let mut stacks = Stacks(stacks);
    instructions.iter().for_each(|i| {
        stacks.apply(i);
    });

    stacks.top_crates()
}

#[derive(derive_new::new, Debug)]
pub struct Instruction {
    quantity: usize,
    source: usize,
    destination: usize,
}

#[derive(Debug)]
struct Stacks(Vec<Stack>);

impl Stacks {
    fn apply(&mut self, instruction: &Instruction) {
        let mut drain: Vec<Crate> = {
            let source = self.0.get_mut(instruction.source - 1).unwrap();
            source
                .drain(source.len() - instruction.quantity..)
                .collect()
        };
        drain.reverse();
        self.0
            .get_mut(instruction.destination - 1)
            .unwrap()
            .append(&mut drain);
    }

    fn top_crates(&self) -> Vec<Crate> {
        self.0
            .iter()
            .filter_map(|stack| stack.last())
            .copied()
            .collect()
    }
}

type Stack = Vec<Crate>;

type Crate = char;

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use regex::{Captures, Regex};
    use speculoos::prelude::*;

    use crate::testing_utils::data_sample_string;

    use super::*;

    type Data = (Vec<Stack>, Vec<Instruction>);

    fn short_data_set() -> Data {
        data_set("day_5_short.txt")
    }

    fn long_data_set() -> Data {
        data_set("day_5.txt")
    }

    fn data_set(file_name: &str) -> Data {
        let sample_string = data_sample_string(file_name);
        let (stacks_input, instructions_input) = sample_string.split_once("\n\n").unwrap();
        (
            parse_stacks(stacks_input),
            parse_instructions(instructions_input),
        )
    }

    fn parse_stacks(input: &str) -> Vec<Vec<Crate>> {
        let lines: Vec<&str> = input.lines().collect();
        let stack_keys: Vec<usize> = lines
            .last()
            .unwrap()
            .split_whitespace()
            .map(|key| key.parse().unwrap())
            .collect();

        stack_keys
            .iter()
            .map(|stack_key| {
                let mut stack: Vec<Crate> = vec![];

                for row_index in 0..lines.len() - 1 {
                    let row = lines.get(row_index).unwrap();
                    let value: &str = row.get(4 * stack_key - 3..4 * stack_key - 2).unwrap_or("");
                    if !value.trim().is_empty() {
                        stack.push(value.chars().last().unwrap());
                    }
                }

                stack.reverse();
                stack
            })
            .collect()
    }

    fn get_capture_integer(captures: &Captures, index: usize) -> usize {
        captures.get(index).unwrap().as_str().parse().unwrap()
    }

    fn parse_instructions(input: &str) -> Vec<Instruction> {
        let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

        regex
            .captures_iter(input)
            .into_iter()
            .map(|cap| {
                Instruction::new(
                    get_capture_integer(&cap, 1),
                    get_capture_integer(&cap, 2),
                    get_capture_integer(&cap, 3),
                )
            })
            .collect()
    }

    #[test]
    fn part_1_short() {
        let (stacks, instructions) = short_data_set();
        assert_that(&part_1(stacks, instructions).into_iter().join(""))
            .is_equal_to("CMZ".to_string())
    }

    #[test]
    fn part_1_long() {
        let (stacks, instructions) = long_data_set();
        assert_that(&part_1(stacks, instructions).into_iter().join(""))
            .is_equal_to("CVCWCRTVQ".to_string())
    }
}
