use crate::utils::load_file;
use std::{collections::HashSet, iter::FromIterator, path::Path};

pub fn star_one(input: &str) -> usize {
    let mut groups: Vec<HashSet<char>> = vec![HashSet::new()];
    for line in input.lines() {
        if line.is_empty() {
            groups.push(HashSet::new());
        }
        for answer in line.chars() {
            groups.last_mut().unwrap().insert(answer);
        }
    }
    groups.iter().map(|group| group.len()).sum()
}

pub fn star_two(input: &str) -> usize {
    let all_answers: HashSet<char> = HashSet::from_iter('a'..='z');
    let mut groups = vec![all_answers.clone()];

    for line in input.lines() {
        if line.is_empty() {
            groups.push(all_answers.clone());
            continue;
        }
        *groups.last_mut().unwrap() = groups
            .last()
            .unwrap()
            .intersection(&HashSet::from_iter(line.chars()))
            .cloned()
            .collect();
    }
    groups.iter().map(|group| group.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::{get_input, star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(&get_input()), 11)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(&get_input()), 6)
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
