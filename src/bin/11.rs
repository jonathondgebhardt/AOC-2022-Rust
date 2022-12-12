#[derive(Debug)]
pub enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    modifier: u32,
    divisor: u32,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    pub fn build(s: &[&str]) -> Self {
        let mut iter = s[1].split(": ");
        iter.next();
        let items: Vec<u32> = iter
            .next()
            .unwrap()
            .split(", ")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let operation = if s[2].contains("*") {
            Operation::Multiply
        } else {
            Operation::Add
        };

        // maybe old instead of number
        let modifier = s[2]
            .split_whitespace()
            .rev()
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let divisor = s[3]
            .split_whitespace()
            .rev()
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let true_monkey = s[4]
            .split_whitespace()
            .rev()
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let false_monkey = s[5]
            .split_whitespace()
            .rev()
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Self {
            items,
            operation,
            modifier,
            divisor,
            true_monkey,
            false_monkey,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let monkey_lines: Vec<&str> = input.lines().collect();
    for monkey in monkey_lines[..].chunks(7) {
        let monkey = Monkey::build(monkey);
        println!("{:?}", monkey);
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 11);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    mod day11 {
        use super::super::*;

        #[test]
        fn test_part_one() {
            let input = aoc::read_file("examples", 11);
            assert_eq!(part_one(&input), Some(10605));
        }

        #[test]
        fn test_part_two() {
            let input = aoc::read_file("examples", 11);
            assert_eq!(part_two(&input), None);
        }
    }
}
