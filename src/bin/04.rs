pub fn str_to_assignment_pair(s: &str) -> [(u32, u32); 2] {
    let mut iter = s.split(",");

    let mut half = iter.next().unwrap().split("-");
    let lower: u32 = half.next().unwrap().parse().unwrap();
    let upper: u32 = half.next().unwrap().parse().unwrap();
    let first = (lower, upper);

    let mut half = iter.next().unwrap().split("-");
    let lower: u32 = half.next().unwrap().parse().unwrap();
    let upper: u32 = half.next().unwrap().parse().unwrap();
    let second = (lower, upper);

    [first, second]
}

pub fn contains(pair: [(u32, u32); 2]) -> bool {
    (pair[0].0 >= pair[1].0 && pair[0].1 <= pair[1].1)
        || (pair[1].0 >= pair[0].0 && pair[1].1 <= pair[0].1)
}

pub fn overlap(pair: [(u32, u32); 2]) -> bool {
    let (lower, upper) = if pair[0].0 < pair[1].0 {
        (pair[0], pair[1])
    } else {
        (pair[1], pair[0])
    };

    lower.1 >= upper.0
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(str_to_assignment_pair)
            .fold(0, |acc, pair| if contains(pair) { acc + 1 } else { acc }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(str_to_assignment_pair)
            .fold(0, |acc, pair| if overlap(pair) { acc + 1 } else { acc }),
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    mod day4 {
        use super::super::*;

        #[test]
        fn test_part_one() {
            let input = aoc::read_file("examples", 4);
            assert_eq!(part_one(&input), Some(2));
        }

        #[test]
        fn test_part_two() {
            let input = aoc::read_file("examples", 4);
            assert_eq!(part_two(&input), Some(4));
        }
    }
}
