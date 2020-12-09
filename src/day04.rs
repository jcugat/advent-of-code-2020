use crate::utils::load_file;
use std::{collections::HashSet, path::Path};

pub fn star_one(input: &str) -> usize {
    let required_keys: HashSet<&'static str> = [
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
        // "cid"
    ]
    .iter()
    .cloned()
    .collect();

    let forest: Vec<&str> = input.lines().collect();
    let mut passports: Vec<HashSet<&str>> = vec![HashSet::new()];

    for line in forest {
        if line.len() == 0 {
            // New passport break
            passports.push(HashSet::new());
            continue;
        }
        passports
            .last_mut()
            .unwrap()
            .extend(line.split(' ').filter_map(|seq| seq.split(':').next()));
    }

    passports
        .iter()
        .filter(|passport| passport.is_superset(&required_keys))
        .count()
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
