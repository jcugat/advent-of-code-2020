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

struct Bag {
    name: String,
    contents: Vec<BagContent>,
}
impl Bag {
    fn new(name: String) -> Bag {
        Bag {
            name,
            contents: Default::default(),
        }
    }
}

struct BagContent {
    quantity: i64,
    bag: Bag,
}
impl BagContent {
    fn new(quantity: i64, name: String) -> BagContent {
        BagContent {
            quantity,
            bag: Bag::new(name),
        }
    }
}

pub fn star_two(input: &str) -> i64 {
    let mut rules: Vec<Bag> = Default::default();

    for line in input.lines() {
        let mut iter1 = line.split(" bags contain ");
        let bag_name = iter1.next().unwrap().to_owned();
        let bag_contents = iter1.next().unwrap().strip_suffix('.').unwrap();

        let mut bag = Bag::new(bag_name);

        if bag_contents == "no other bags" {
            rules.push(bag);
            continue;
        }

        for bag_content in bag_contents.split(", ") {
            let num_and_bag = bag_content
                .strip_suffix(" bags")
                .unwrap_or_else(|| bag_content.strip_suffix(" bag").unwrap());
            let mut iter2 = num_and_bag.splitn(2, ' ');
            let inner_bag_quantity = iter2.next().unwrap().parse::<i64>().unwrap();
            let inner_bag_name = iter2.next().unwrap().to_owned();

            bag.contents
                .push(BagContent::new(inner_bag_quantity, inner_bag_name));
        }
        rules.push(bag);
    }
    search(&rules, "shiny gold") - 1
}

fn search(rules: &[Bag], current_name: &str) -> i64 {
    for bag in rules {
        if bag.name == current_name {
            if bag.contents.is_empty() {
                return 1;
            }

            let mut total_inner = 0;
            for inner_bag in bag.contents.iter() {
                let count_inner = search(rules, &inner_bag.bag.name);
                total_inner += count_inner * inner_bag.quantity;
            }
            return total_inner + 1;
        }
    }
    panic!("Could not find bag with name {}", current_name);
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
        assert_eq!(star_two(&get_input()), 32)
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
