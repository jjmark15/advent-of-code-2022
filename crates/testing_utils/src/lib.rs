use std::path::{Path, PathBuf};

pub use speculoos;

fn data_sample_path(filename: &str) -> PathBuf {
    Path::new("../../data_samples").join(filename)
}

pub fn data_sample_string(filename: &str) -> String {
    std::fs::read_to_string(data_sample_path(filename)).unwrap()
}