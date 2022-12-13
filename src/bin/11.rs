use std::collections::HashMap;

#[derive(Debug)]
pub enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    modifier: Option<u128>,
    divisor: u128,
    true_monkey: usize,
    false_monkey: usize,
    inspect_count: u32,
    should_apply_worry: bool,
}

impl Monkey {
    pub fn build(s: &[&str], apply_worry: bool) -> Self {
        let mut iter = s[1].split(": ");
        iter.next();
        let items: Vec<u128> = iter
            .next()
            .unwrap()
            .split(", ")
            .map(|n| n.parse::<u128>().unwrap())
            .collect();

        let operation = if s[2].contains("*") {
            Operation::Multiply
        } else {
            Operation::Add
        };

        let modifier = match s[2]
            .split_whitespace()
            .rev()
            .next()
            .unwrap()
            .parse::<u128>()
        {
            Ok(modifier) => Some(modifier),
            _ => None,
        };

        let divisor = s[3]
            .split_whitespace()
            .rev()
            .next()
            .unwrap()
            .parse::<u128>()
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
            should_apply_worry: apply_worry,
        }
    }

    fn apply_worry(&self, item: u128) -> u128 {
        if self.should_apply_worry {
            match self.operation {
                Operation::Add => match self.modifier {
                    Some(modifier) => item + modifier,
                    None => item + item,
                },
                Operation::Multiply => match self.modifier {
                    Some(modifier) => item * modifier,
                    // overflow on part 2
                    None => item * item,
                },
            }
        } else {
            item
        }
    }

    fn apply_bored(&self, item: u128) -> u128 {
        item / 3
    }

    fn get_monkey_receiver(&self, item: u128) -> usize {
        if item % self.divisor == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }

    fn receive_items(&mut self, items: &mut Vec<u128>) {
        self.items.append(items);
    }

    pub fn remove_first(&mut self) -> u128 {
        self.items.remove(0)
    }

    pub fn inspect_item(&mut self) -> (u128, usize) {
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

pub fn get_monkey_business(s: &str, n: u32, apply_worry: bool) -> u128 {
    let monkey_lines: Vec<&str> = s.lines().collect();
    let mut monkeys: Vec<Monkey> = monkey_lines[..]
        .chunks(7)
        .map(|line| Monkey::build(line, apply_worry))
        .collect();

    for _ in 0..n {
        for index in 0..monkeys.len() {
            let monkey = &mut monkeys[index];
            let mut sent_items: HashMap<usize, Vec<u128>> = HashMap::new();

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
    monkeys[0].get_inspect_count() as u128 * monkeys[1].get_inspect_count() as u128
}

pub fn part_one(input: &str) -> Option<u128> {
    Some(get_monkey_business(input, 20, true))
}

pub fn part_two(input: &str) -> Option<u128> {
    Some(get_monkey_business(input, 10_000, true))
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
