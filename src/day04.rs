use crate::utils::load_file;
use std::{collections::HashSet, ops::RangeInclusive, path::Path};

const BYR_VALID_RANGE: RangeInclusive<usize> = 1920..=2002;
const IYR_VALID_RANGE: RangeInclusive<usize> = 2010..=2020;
const EYR_VALID_RANGE: RangeInclusive<usize> = 2020..=2030;
const VALID_CM_HEIGHTS: RangeInclusive<usize> = 150..=193;
const VALID_IN_HEIGHTS: RangeInclusive<usize> = 59..=76;

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

pub fn star_two(input: &str) -> usize {
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

        let values: Vec<[&str; 2]> = line
            .split(' ')
            .map(|seq| {
                let mut key_value = seq.split(':');
                let key = key_value.next().unwrap();
                let value = key_value.next().unwrap();
                [key, value]
            })
            .collect();

        for value in values {
            let key = value[0];
            let value = value[1];
            let mut valid = false;
            if key == "byr" {
                valid = BYR_VALID_RANGE.contains(&value.parse::<usize>().unwrap_or_default());
            } else if key == "iyr" {
                valid = IYR_VALID_RANGE.contains(&value.parse::<usize>().unwrap_or_default());
            } else if key == "eyr" {
                valid = EYR_VALID_RANGE.contains(&value.parse::<usize>().unwrap_or_default());
            } else if key == "hgt" {
                valid = validate_hgt(value);
            } else if key == "hcl" {
                valid = validate_hcl(value);
            } else if key == "ecl" {
                valid = validate_ecl(value);
            } else if key == "pid" {
                valid = validate_pid(value);
            } else if key == "cid" {
                valid = true
            }
            if valid {
                passports.last_mut().unwrap().insert(key);
            }
        }
    }

    passports
        .iter()
        .filter(|passport| passport.is_superset(&required_keys))
        .count()
}

fn validate_hgt(value: &str) -> bool {
    if value.ends_with("cm") {
        return VALID_CM_HEIGHTS.contains(
            &value
                .split("cm")
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap_or_default(),
        );
    } else if value.ends_with("in") {
        return VALID_IN_HEIGHTS.contains(
            &value
                .split("in")
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap_or_default(),
        );
    }
    false
}

fn validate_hcl(value: &str) -> bool {
    if !value.starts_with("#") {
        return false;
    }
    let hex = value.strip_prefix("#").unwrap();
    u32::from_str_radix(hex, 16).is_ok()
}

fn validate_ecl(value: &str) -> bool {
    match value {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

fn validate_pid(value: &str) -> bool {
    value.len() == 9 && value.parse::<u64>().is_ok()
}

#[cfg(test)]
mod tests {
    use crate::utils::load_file;

    use super::{get_input, star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(&get_input()), 2)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(&load_file("day04_example_invalids.txt")), 0);
        assert_eq!(star_two(&load_file("day04_example_valids.txt")), 4);
        assert_eq!(star_two(&get_input()), 2);
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
