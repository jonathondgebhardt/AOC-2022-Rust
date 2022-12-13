use std::collections::HashMap;

#[derive(Debug)]
pub enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    modifier: Option<u32>,
    divisor: u32,
    true_monkey: usize,
    false_monkey: usize,
    inspect_count: u32,
    should_apply_bored: bool,
}

impl Monkey {
    pub fn build(s: &[&str], should_apply_bored: bool) -> Self {
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

        let modifier = match s[2].split_whitespace().rev().next().unwrap().parse::<u32>() {
            Ok(modifier) => Some(modifier),
            _ => None,
        };

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
            inspect_count: 0,
            should_apply_bored,
        }
    }

    fn apply_worry(&self, item: u32) -> u32 {
        match self.operation {
            Operation::Add => match self.modifier {
                Some(modifier) => {
                    ((item % self.divisor) + (modifier % self.divisor)) % self.divisor
                }
                None => ((item % self.divisor) + (item % self.divisor)) % self.divisor,
            },
            Operation::Multiply => match self.modifier {
                Some(modifier) => {
                    ((item % self.divisor) * (modifier % self.divisor)) % self.divisor
                }
                None => ((item % self.divisor) * (item % self.divisor)) % self.divisor,
            },
        }
    }

    fn apply_bored(&self, item: u32) -> u32 {
        if self.should_apply_bored {
            item / 3
        } else {
            item
        }
    }

    fn get_monkey_receiver(&self, item: u32) -> usize {
        if item % self.divisor == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }

    fn receive_items(&mut self, items: &mut Vec<u32>) {
        self.items.append(items);
    }

    pub fn remove_first(&mut self) -> u32 {
        self.items.remove(0)
    }

    pub fn inspect_item(&mut self) -> (u32, usize) {
        assert!(!self.is_done());

        self.inspect_count += 1;

        let mut item = self.remove_first();
        item = self.apply_worry(item);
        item = self.apply_bored(item);

        (item, self.get_monkey_receiver(item))
    }

    pub fn is_done(&self) -> bool {
        self.items.is_empty()
    }

    pub fn get_inspect_count(&self) -> u32 {
        self.inspect_count
    }
}

pub fn get_monkey_business(s: &str, n: u32, apply_bored: bool) -> u32 {
    let monkey_lines: Vec<&str> = s.lines().collect();
    let mut monkeys: Vec<Monkey> = monkey_lines[..]
        .chunks(7)
        .map(|line| Monkey::build(line, apply_bored))
        .collect();

    for _ in 0..n {
        for index in 0..monkeys.len() {
            let monkey = &mut monkeys[index];
            let mut sent_items: HashMap<usize, Vec<u32>> = HashMap::new();

            while !monkey.is_done() {
                let (item, receiver) = monkey.inspect_item();
                sent_items.entry(receiver).or_default().push(item);
            }

            for (index, mut items) in sent_items {
                monkeys[index].receive_items(&mut items);
            }
        }
    }

    monkeys.sort_by(|a, b| a.get_inspect_count().cmp(&b.get_inspect_count()).reverse());
    monkeys[0].get_inspect_count() * monkeys[1].get_inspect_count()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(get_monkey_business(input, 20, true))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(get_monkey_business(input, 10_000, false))
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
            assert_eq!(part_two(&input), Some(2_713_310_158));
        }
    }
}
