advent_of_code::solution!(2);

fn is_safe<I>(mut numbers: I) -> bool
where
    I: Iterator<Item = i32>,
{
    let mut last_number = numbers.next().unwrap();
    let mut direction = None;
    for number in numbers {
        let diff = last_number - number;
        match direction {
            Some(direction) => {
                if direction != diff.signum() {
                    return false;
                }
            }
            None => {
                direction = Some(diff.signum());
            }
        }
        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }

        last_number = number;
    }
    return true;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut out = 0_u32;
    for line in input.lines() {
        let split_line = line.split(' ');
        let numbers = split_line.map(|s| s.parse::<i32>().unwrap());
        if is_safe(numbers) {
            out += 1;
        }
    }
    return Some(out);
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
