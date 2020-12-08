#![allow(dead_code)]
mod day01;
mod day02;
mod utils;

#[cfg(test)]
mod tests {
    use stdext::function_name;

    #[test]
    fn day01() {
        use crate::day01::{star_one, star_two};
        use crate::utils::load_file;

        let function_name = String::from(function_name!());
        let file_name = format!("{}_input.txt", &function_name[&function_name.len() - 5..]);
        let input = load_file(&file_name);

        assert_eq!(star_one(&input), 445536);
        assert_eq!(star_two(&input), 138688160);
    }

    #[test]
    fn day02() {
        use crate::day02::{star_one, star_two};
        use crate::utils::load_file;

        let function_name = String::from(function_name!());
        let file_name = format!("{}_input.txt", &function_name[&function_name.len() - 5..]);
        let input = load_file(&file_name);

        assert_eq!(star_one(&input), 434);
        assert_eq!(star_two(&input), 1);
    }
}
