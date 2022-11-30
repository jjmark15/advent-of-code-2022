pub fn double(n: u64) -> u64 {
    n * 2
}

#[cfg(test)]
mod tests {
    use testing_utils::data_sample_string;
    use testing_utils::speculoos::prelude::*;

    use super::*;

    #[test]
    fn it_works() {
        let number: u64 = data_sample_string("example_1.txt").parse().unwrap();

        assert_that(&double(number)).is_equal_to(2);
    }
}
