use crate::utils::load_file;
use itertools::Itertools;
use std::path::Path;

pub fn star_one(input: &str, range: usize) -> u64 {
    let lines: Vec<u64> = input.lines().map(str::parse).map(Result::unwrap).collect();

    for pos_test in range..lines.len() {
        if lines[pos_test - range..pos_test]
            .iter()
            .tuple_combinations()
            .find(|(&x, &y)| x + y == lines[pos_test])
            .is_none()
        {
            return lines[pos_test];
        }
    }
    panic!("Could not find solution");
}

pub fn star_two(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::{get_input, star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(&get_input(), 5), 127)
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
