use crate::utils::load_file;
use std::path::Path;

pub fn star_one(input: &str) -> usize {
    let forest: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    forest
        .iter()
        .enumerate()
        .skip(1) // Start position isn't counted
        .map(|(pos_y, row)| row[(pos_y * 3) % row.len()])
        .filter(|&place| place == '#')
        .count()
}

pub fn star_two(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::{get_input, star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(&get_input()), 7)
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
