use std::collections::BTreeSet;

pub fn get_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 96, // 'a' starts at 97 in ASCII, but challenge says it starts at 1
        'A'..='Z' => c as u32 - 64 + 26, // 'A' starts at 65 in ASCII, but challenge says it starts at 26
        _ => panic!("unsupported priority for {}", c),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut priority_sum: u32 = 0;

    for line in input.lines() {
        let first_compartment: BTreeSet<char> = line[..line.len() / 2].chars().collect();

        for item in line[line.len() / 2..].chars() {
            if first_compartment.contains(&item) {
                priority_sum += get_priority(item);

                break;
            }
        }
    }

    Some(priority_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut priority_sum: u32 = 0;
    let mut groups: Vec<BTreeSet<char>> = Vec::new();

    for group in input.lines() {
        if groups.len() < 3 {
            groups.push(group.chars().collect());
        }

        if groups.len() == 3 {
            for item in &groups[0] {
                if groups[1].contains(&item) && groups[2].contains(&item) {
                    priority_sum += get_priority(*item);

                    break;
                }
            }

            groups.clear();
        }
    }

    Some(priority_sum)
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    mod day3 {
        use super::super::*;

        #[test]
        fn test_part_one() {
            let input = aoc::read_file("examples", 3);
            assert_eq!(part_one(&input), Some(157));
        }

        #[test]
        fn test_part_two() {
            let input = aoc::read_file("examples", 3);
            assert_eq!(part_two(&input), Some(70));
        }
    }
}
