use crate::utils::load_file;
use std::{collections::HashSet, path::Path};

pub fn star_one(input: &str) -> usize {
    let mut rules: Vec<(&str, Vec<&str>)> = Default::default();

    for line in input.lines() {
        let mut iter1 = line.split(" bags contain ");
        let bag_name = iter1.next().unwrap();
        let bag_contents = iter1.next().unwrap().strip_suffix('.').unwrap();

        rules.push((bag_name, Default::default()));

        if bag_contents == "no other bags" {
            continue;
        }

        for bag_content in bag_contents.split(", ") {
            let num_and_bag = bag_content
                .strip_suffix(" bags")
                .unwrap_or_else(|| bag_content.strip_suffix(" bag").unwrap());
            let mut iter2 = num_and_bag.splitn(2, ' ');
            let _num = iter2.next().unwrap().parse::<i64>().unwrap();
            let bag = iter2.next().unwrap();

            rules.last_mut().unwrap().1.push(bag);
        }
    }

    let mut visited: HashSet<&str> = ["shiny gold"].iter().cloned().collect();
    while let Some(pos) = rules.iter().position(|(_name, contents)| {
        contents
            .iter()
            .any(|&contained_bag| visited.contains(contained_bag))
    }) {
        let found = rules.remove(pos);
        visited.insert(found.0);
    }
    visited.len() - 1
}

pub fn star_two(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::{get_input, star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(&get_input()), 4)
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
