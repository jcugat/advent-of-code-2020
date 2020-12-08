use crate::utils::load_file;
use std::path::Path;

pub fn star_one(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut valid = 0;
    for line in lines {
        // Format is:
        //    N-M C: abcde
        //     ^ ^ ^
        //     | | |
        // idx 1 2 3
        // N: min count
        // M: max count
        // C: character to check
        let idx1 = line.find('-').unwrap();
        let idx2 = line.find(' ').unwrap();
        let idx3 = line.find(':').unwrap();

        let min_count: usize = line[0..idx1].parse().unwrap();
        let max_count: usize = line[idx1 + 1..idx2].parse().unwrap();
        let char_to_check: char = line[idx2 + 1..idx2 + 2].parse().unwrap();
        let password = line[idx3 + 2..].to_string();

        let occurrences = password.chars().filter(|x| x == &char_to_check).count();
        if occurrences <= max_count && occurrences >= min_count {
            valid += 1;
        }
    }
    valid
}

pub fn star_two(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::{get_input, star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(&get_input()), 2)
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
