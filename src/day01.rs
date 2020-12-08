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
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("1721\n979\n366\n299\n675\n1456"), 514579)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two("1721\n979\n366\n299\n675\n1456"), 241861950)
    }
}
