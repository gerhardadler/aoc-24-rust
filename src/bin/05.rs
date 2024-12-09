advent_of_code::solution!(5);

struct Rule {
    a: u32,
    b: u32,
}

impl Rule {
    fn is_valid(&self, update: &Vec<u32>) -> bool {
        let mut b_found = false;
        for entry in update {
            if entry == &self.a && b_found {
                return false;
            }
            if entry == &self.b {
                b_found = true
            }
        }
        true
    }
}

fn is_update_valid(update: &Vec<u32>, rules: &Vec<Rule>) -> bool {
    for rule in rules {
        if !rule.is_valid(&update) {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut out = 0;

    let mut rules = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        if line.contains('|') {
            let mut split_line = line.split('|');
            let a = split_line.next().unwrap().parse::<u32>().unwrap();
            let b = split_line.next().unwrap().parse::<u32>().unwrap();
            let rule = Rule { a, b };
            rules.push(rule);
            continue;
        }
        if line.is_empty() {
            continue;
        }
        updates.push(line.split(',').map(|n| n.parse::<u32>().unwrap()).collect())
    }
    for update in updates {
        if is_update_valid(&update, &rules) {
            out += update[update.len() / 2]
        }
    }
    Some(out)
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
