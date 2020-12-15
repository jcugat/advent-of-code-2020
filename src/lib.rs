#![allow(dead_code)]
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
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
        assert_eq!(star_two(&input), 509);
    }

    #[test]
    fn day03() {
        use crate::day03::{star_one, star_two};
        use crate::utils::load_file;

        let function_name = String::from(function_name!());
        let file_name = format!("{}_input.txt", &function_name[&function_name.len() - 5..]);
        let input = load_file(&file_name);

        assert_eq!(star_one(&input), 171);
        assert_eq!(star_two(&input), 1206576000);
    }

    #[test]
    fn day04() {
        use crate::day04::{star_one, star_two};
        use crate::utils::load_file;

        let function_name = String::from(function_name!());
        let file_name = format!("{}_input.txt", &function_name[&function_name.len() - 5..]);
        let input = load_file(&file_name);

        assert_eq!(star_one(&input), 222);
        assert_eq!(star_two(&input), 140);
    }

    #[test]
    fn day05() {
        use crate::day05::{star_one, star_two};
        use crate::utils::load_file;

        let function_name = String::from(function_name!());
        let file_name = format!("{}_input.txt", &function_name[&function_name.len() - 5..]);
        let input = load_file(&file_name);

        assert_eq!(star_one(&input), 904);
        assert_eq!(star_two(&input), 669);
    }

    #[test]
    fn day06() {
        use crate::day06::{star_one, star_two};
        use crate::utils::load_file;

        let function_name = String::from(function_name!());
        let file_name = format!("{}_input.txt", &function_name[&function_name.len() - 5..]);
        let input = load_file(&file_name);

        assert_eq!(star_one(&input), 6382);
        assert_eq!(star_two(&input), 3197);
    }

    #[test]
    fn day07() {
        use crate::day07::{star_one, star_two};
        use crate::utils::load_file;

        let function_name = String::from(function_name!());
        let file_name = format!("{}_input.txt", &function_name[&function_name.len() - 5..]);
        let input = load_file(&file_name);

        assert_eq!(star_one(&input), 265);
        assert_eq!(star_two(&input), 14177);
    }

    #[test]
    fn day08() {
        use crate::day08::{star_one, star_two};
        use crate::utils::load_file;

        let function_name = String::from(function_name!());
        let file_name = format!("{}_input.txt", &function_name[&function_name.len() - 5..]);
        let input = load_file(&file_name);

        assert_eq!(star_one(&input), 1614);
        assert_eq!(star_two(&input), 1);
    }
}
