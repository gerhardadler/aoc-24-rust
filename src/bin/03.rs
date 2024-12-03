use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut out = 0;
    for (_, [a_str, b_str]) in re.captures_iter(input).map(|c| c.extract()) {
        let a = a_str.parse::<u32>().unwrap();
        let b = b_str.parse::<u32>().unwrap();
        out += a * b;
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
    let mut out = 0;
    let mut active = true;
    for captures in re.captures_iter(input) {
        if &captures[0] == "do()" {
            active = true;
            continue;
        }
        if &captures[0] == "don't()" {
            active = false;
            continue;
        }
        if !active {
            continue;
        }

        let [a, b] = [&captures[1], &captures[2]].map(|c| c.parse::<u32>().unwrap());
        out += a * b;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
