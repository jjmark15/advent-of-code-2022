use std::path::{Path, PathBuf};

pub use speculoos;

fn data_sample_path(filename: &str) -> PathBuf {
    Path::new("../../data_samples").join(filename)
}

pub fn data_sample_string(filename: &str) -> String {
    std::fs::read_to_string(data_sample_path(filename)).unwrap()
}

pub trait LineGroups {
    fn line_groups(&self) -> Vec<Vec<String>>;
}

impl LineGroups for str {
    fn line_groups(&self) -> Vec<Vec<String>> {
        self.lines()
            .fold(vec![vec![]], |mut acc, curr| {
                if curr.is_empty() {
                    acc.push(vec![]);
                } else {
                    acc.last_mut().unwrap().push(curr.to_string());
                }
                acc
            })
            .into_iter()
            .filter(|group| !group.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    #[test]
    fn groups_lines() {
        assert_that(&"line1\n\nline2".line_groups())
            .is_equal_to(vec![vec!["line1".to_string()], vec!["line2".to_string()]]);
    }
}
