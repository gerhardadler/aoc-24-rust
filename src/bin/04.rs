advent_of_code::solution!(4);

fn count_xmas<I: Iterator<Item = u8>>(chars: I) -> u32 {
    let mut out = 0;
    let search = "XMAS".as_bytes();
    let search_reverse = "SAMX".as_bytes();
    let mut found = 0;
    let mut found_reverse = 0;
    for c in chars {
        if c == search[found] {
            found += 1;
            if found == 4 {
                out += 1;
                found = 0;
            }
        } else {
            found = 0;
            if c == search[found] {
                found += 1;
            }
        }

        if c == search_reverse[found_reverse] {
            found_reverse += 1;
            if found_reverse == 4 {
                out += 1;
                found_reverse = 0;
            }
        } else {
            found_reverse = 0;
            if c == search_reverse[found_reverse] {
                found_reverse += 1;
            }
        }
    }
    out
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut out = 0;
    let columns = input.find('\n').unwrap();
    let chars: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().iter().cloned().collect())
        .collect();

    let rows = chars.len();

    for y in 0..rows {
        out += count_xmas((0..rows).map(|x| chars[y][x]));

        let steps = rows - y;
        out += count_xmas((0..steps).map(|step| chars[step + y][step]));
        out += count_xmas((1..steps).map(|step| chars[step + y][columns - step - 1]));

        out += count_xmas((0..=y).map(|step| chars[y - step][step]));
        out += count_xmas((1..=y).map(|step| chars[y - step][columns - step - 1]));
    }

    for x in 0..columns {
        out += count_xmas((0..columns).map(|y| chars[y][x]));
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
        println!("{r}", r = result.unwrap());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
