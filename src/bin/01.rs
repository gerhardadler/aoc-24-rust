advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut items1 = Vec::new();
    let mut items2 = Vec::new();
    for line in input.lines() {
        let mut split_line = line.split_whitespace();
        items1.push(
            split_line
                .next()
                .ok_or("Line has less than two elements")
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        );
        items2.push(
            split_line
                .next()
                .ok_or("Line has less than two elements")
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        );
    }
    (items1, items2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut items1, mut items2) = parse_input(input);
    items1.sort();
    items2.sort();

    Some(
        items1
            .iter()
            .zip(items2.iter())
            .fold(0_u32, |acc, (x, y)| acc + x.abs_diff(*y)),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (items1, items2) = parse_input(input);

    let mut result = 0_u32;
    for item in items1 {
        let other_count = items2.iter().filter(|&n| *n == item).count();
        result += item * other_count as u32;
    }
    Some(result)
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
