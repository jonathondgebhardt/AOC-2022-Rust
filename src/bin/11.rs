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

        // may be old instead of number
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
        }
    }

    fn apply_worry(&self, item: u32) -> u32 {
        match self.operation {
            Operation::Add => match self.modifier {
                Some(modifier) => item + modifier,
                None => item + item,
            },
            Operation::Multiply => match self.modifier {
                Some(modifier) => item * modifier,
                None => item * item,
            },
        }
    }

    fn apply_bored(&self, item: u32) -> u32 {
        item / 3
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

    fn receive_item(&mut self, item: u32) {
        self.items.push(item);
    }

    pub fn drop_first(&mut self) {
        self.items.remove(0);
    }

    pub fn inspect_item(&self) -> (u32, usize) {
        assert!(!self.is_done());

        let mut item = self.items[0];
        item = self.apply_worry(item);
        item = self.apply_bored(item);

        (item, self.get_monkey_receiver(item))
    }

    pub fn is_done(&self) -> bool {
        self.items.is_empty()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let monkey_lines: Vec<&str> = input.lines().collect();
    let mut monkeys: Vec<Monkey> = monkey_lines[..].chunks(7).map(Monkey::build).collect();

    for monkey in &monkeys {
        while !monkey.is_done() {
            let (item, receiver) = monkey.inspect_item();
            println!("{} goes to {}", item, receiver);

            let receiver = &mut monkeys[receiver];
        }
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
