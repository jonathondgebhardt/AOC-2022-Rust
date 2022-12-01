pub fn sum_top_n_calorie_counts(input: &str, num: usize) -> u32 {
    let mut calorie_counts: Vec<u32> = Vec::new();
    let mut calorie_count = 0;

    for line in input.lines() {
        if let Ok(calories) = line.parse::<u32>() {
            calorie_count += calories;
        } else {
            calorie_counts.push(calorie_count);
            calorie_count = 0;
        }
    }

    // We skip the empty line at the end of the file, so add the remaining calories.
    calorie_counts.push(calorie_count);

    assert!(num < calorie_counts.len());

    calorie_counts.sort();

    calorie_count = 0;

    let mut iter = calorie_counts.iter().rev();
    let mut countdown = num;
    while countdown > 0 {
        calorie_count += *iter.next().unwrap();
        countdown -= 1;
    }

    calorie_count
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(sum_top_n_calorie_counts(input, 1))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(sum_top_n_calorie_counts(input, 3))
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    mod day1 {
        use super::super::*;

        #[test]
        fn test_part_one() {
            let input = aoc::read_file("examples", 1);
            assert_eq!(part_one(&input), Some(24000));
        }

        #[test]
        fn test_part_two() {
            let input = aoc::read_file("examples", 1);
            assert_eq!(part_two(&input), Some(45000));
        }
    }
}
