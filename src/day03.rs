use crate::utils::load_file;
use std::path::Path;

pub fn walk_forest(input: &str, step_x: f64, step_y: f64) -> usize {
    let forest: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    forest
        .iter()
        .enumerate()
        .skip(1) // Start position isn't counted
        .map(|(pos_y, row)| match pos_y % step_y as usize {
            0 => row[(pos_y as f64 * step_x) as usize % row.len()],
            _ => '.',
        })
        .filter(|&place| place == '#')
        .count()
}

pub fn star_one(input: &str) -> usize {
    walk_forest(input, 3.0, 1.0)
}

pub fn star_two(input: &str) -> usize {
    let mut result = 1;
    let slopes = [
        (1.0, 1.0),
        (3.0, 1.0), // This is the slope you already checked
        (5.0, 1.0),
        (7.0, 1.0),
        (0.5, 2.0), // Had to do a hack here with 0.5
    ];
    for &(step_x, step_y) in slopes.iter() {
        result *= walk_forest(input, step_x, step_y);
    }
    result
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
        assert_eq!(star_two(&get_input()), 336)
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
