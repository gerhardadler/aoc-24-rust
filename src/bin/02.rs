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
        if !(1..=3).contains(&abs_diff) {
            return false;
        }

        last_number = number;
    }
    true
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
    Some(out)
}

fn is_safe_two<I>(numbers: I, skipped_index: Option<usize>) -> bool
where
    I: Iterator<Item = i32> + Clone,
{
    let mut work_numbers = numbers.clone();

    if skipped_index == Some(0) {
        work_numbers.next();
    }

    let mut last_number = work_numbers.next().unwrap();
    let mut direction = None;

    for (i, number) in work_numbers.enumerate() {
        if skipped_index == Some(i) {
            continue;
        }
        let diff = last_number - number;

        if direction.is_none() {
            direction = Some(diff.signum());
        } else if Some(diff.signum()) != direction {
            if skipped_index.is_some() {
                return false;
            }
            if i == 1 && is_safe_two(numbers.clone(), Some(0)) {
                return true;
            }
            return is_safe_two(numbers.clone(), Some(i));
        }

        let abs_diff = diff.abs();
        if !(1..=3).contains(&abs_diff) {
            if skipped_index.is_some() {
                return false;
            }
            return is_safe_two(numbers.clone(), Some(i));
        }

        last_number = number;
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut out = 0_u32;
    for line in input.lines() {
        let split_line = line.split(' ');
        let numbers = split_line.map(|s| s.parse::<i32>().unwrap());
        if is_safe_two(numbers, None) {
            out += 1;
        }
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
