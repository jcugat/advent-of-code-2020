use crate::utils::load_file;
use std::path::Path;

pub fn boarding_pass_id(input: &str) -> i64 {
    let mut row_start = 0;
    let mut row_end = 127;
    let mut row_size = (row_end - row_start) + 1;

    let mut column_start = 0;
    let mut column_end = 7;
    let mut column_size = (column_end - column_start) + 1;

    for (index, character) in input.chars().enumerate() {
        if index < 7 {
            // Rows
            row_size /= 2;
            match character {
                'F' => row_end -= row_size,
                'B' => row_start += row_size,
                _ => panic!("Found unknown value {}", character),
            }
        } else {
            // Columns
            column_size /= 2;
            match character {
                'L' => column_end -= column_size,
                'R' => column_start += column_size,
                _ => panic!("Found unknown value {}", character),
            }
        }
    }
    row_start * 8 + column_start
}

pub fn star_one(input: &str) -> i64 {
    input
        .lines()
        .map(|boarding_pass| boarding_pass_id(boarding_pass))
        .max()
        .unwrap()
}

pub fn star_two(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::{get_input, star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(&get_input()), 820)
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
