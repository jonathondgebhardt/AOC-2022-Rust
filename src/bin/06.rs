use std::collections::BTreeSet;

pub fn first_n_unique(n: u32, s: &str) -> Option<u32> {
    for (i, w) in s.as_bytes().windows(n as usize).enumerate() {
        let set: BTreeSet<char> = w.iter().map(|c| *c as char).collect();
        if set.len() == n as usize {
            return Some(i as u32 + n);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    first_n_unique(4, input)
}

pub fn part_two(input: &str) -> Option<u32> {
    first_n_unique(14, input)
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    mod day6 {
        use super::super::*;

        #[test]
        fn test_part_one() {
            let input = aoc::read_file("examples", 6);
            assert_eq!(part_one(&input), Some(7));
        }

        #[test]
        fn test_part_one_extra1() {
            assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        }

        #[test]
        fn test_part_one_extra2() {
            assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        }

        #[test]
        fn test_part_one_extra3() {
            assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        }

        #[test]
        fn test_part_one_extra4() {
            assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
        }

        #[test]
        fn test_part_two() {
            let input = aoc::read_file("examples", 6);
            assert_eq!(part_two(&input), Some(19));
        }

        #[test]
        fn test_part_two_extra1() {
            assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        }

        #[test]
        fn test_part_two_extra2() {
            assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        }

        #[test]
        fn test_part_two_extra3() {
            assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        }

        #[test]
        fn test_part_two_extra4() {
            assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
        }
    }
}
