use crate::utils::load_file;
use itertools::Itertools;
use std::path::Path;

pub fn star_one(input: &str) -> usize {
    let nums = input
        .lines()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .sorted()
        .tuple_windows()
        .map(|(x, y)| y - x)
        .fold((1, 1), |(num_1s, num_3s), x| match x {
            1 => (num_1s + 1, num_3s),
            3 => (num_1s, num_3s + 1),
            _ => (num_1s, num_3s),
        });
    nums.0 * nums.1
}

pub fn star_two(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::{get_input, star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(&get_input()), 220)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(&get_input()), 1)
    }
}

fn get_input() -> String {
    let current_path = Path::new(&file!());
    let current_path_extension =
        format!(".{}", current_path.extension().unwrap().to_str().unwrap());
    let current_name = current_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .trim_end_matches(&current_path_extension);
    let file_name = format!("{}_example.txt", current_name);
    load_file(&file_name)
}
