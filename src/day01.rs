use crate::utils::load_file;
use std::path::Path;

pub fn star_one(input: &str) -> i64 {
    let numbers: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                return numbers[i] * numbers[j];
            }
        }
    }
    0
}

pub fn star_two(input: &str) -> i64 {
    let numbers: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            for k in (j + 1)..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    return numbers[i] * numbers[j] * numbers[k];
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{get_input, star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(&get_input()), 514579)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(&get_input()), 241861950)
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
