advent_of_code::solution!(4);

fn count_xmas<I: Iterator<Item = u8>>(chars: I) -> u32 {
    let mut out = 0;
    let search = "XMAS".as_bytes();
    let search_reverse = "SAMX".as_bytes();
    let mut found = 0;
    let mut found_reverse = 0;
    for char in chars {
        if char == search[found] {
            found += 1;
            if found == 4 {
                out += 1;
                found = 0;
            }
        } else {
            found = 0;
            if char == search[found] {
                found += 1;
            }
        }

        if char == search_reverse[found_reverse] {
            found_reverse += 1;
            if found_reverse == 4 {
                out += 1;
                found_reverse = 0;
            }
        } else {
            found_reverse = 0;
            if char == search_reverse[found_reverse] {
                found_reverse += 1;
            }
        }
    }
    out
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut out = 0;
    let columns = input.find('\n').unwrap();
    let chars: Vec<u8> = input
        .as_bytes()
        .iter()
        .cloned()
        .filter(|&c| c != b'\n' && c != b'\r')
        .collect();

    let chars_len = chars.len();
    let rows = chars_len / columns;

    for y in 0..rows {
        let i = y * columns;
        out += count_xmas((i..(i + columns)).map(|j| chars[j]));

        let first_step_count = (chars_len - i) / columns;
        let second_sted_count = columns - first_step_count;

        {
            let step = columns + 1;

            let step_range_1 = i..(i + step * first_step_count);
            let step_range_2_start = step_range_1.end;
            out += count_xmas(step_range_1.step_by(step).map(|j| chars[j]));

            let step_range_2 = step_range_2_start..(step_range_2_start + step * second_sted_count);
            out += count_xmas(step_range_2.step_by(step).map(|j| chars[j % chars_len]));
        }

        {
            let step = columns - 1;
            let up_i = i + step;

            let step_range_1 = up_i..(up_i + step * first_step_count);
            let step_range_2_start = step_range_1.end;
            out += count_xmas(step_range_1.step_by(step).map(|j| chars[j]));

            let step_range_2 = step_range_2_start..(step_range_2_start + step * second_sted_count);
            out += count_xmas(step_range_2.step_by(step).map(|j| chars[j % chars_len]));
        };
    }

    for x in 0..columns {
        out += count_xmas((x..chars_len).step_by(columns).map(|j| chars[j]));
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
