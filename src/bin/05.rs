use std::collections::HashMap;

pub struct SupplyStacks {
    // This could probably be a vector of vectors, but the api for creating
    // something at an "index" if it doesn't exist is super convenient
    supply: HashMap<u32, Vec<char>>,
}

impl SupplyStacks {
    pub fn build(s: &str) -> Self {
        let mut supply: HashMap<u32, Vec<char>> = HashMap::new();

        // Get only the crates, in reverse, and skip the first line because it
        // contains the crate indexes.
        let mut iter = s.split("\n\n").next().unwrap().lines().rev();
        iter.next();

        for line in iter {
            for (index, c) in line.chars().enumerate() {
                if c.is_alphabetic() {
                    let index = index / 4;
                    let stack = supply.entry(index as u32).or_default();
                    stack.push(c);
                }
            }
        }

        SupplyStacks { supply }
    }

    pub fn move_stack(&mut self, n: u32, from: u32, to: u32) {
        for _ in 0..n {
            let from_vec = self.supply.get_mut(&from).unwrap();
            let popped = from_vec.pop().unwrap();

            let to_vec = self.supply.get_mut(&to).unwrap();
            to_vec.push(popped);
        }
    }

    pub fn move_vec(&mut self, n: u32, from: u32, to: u32) {
        let from_vec = self.supply.get_mut(&from).unwrap();
        let boxes: Vec<char> = from_vec.drain(from_vec.len() - (n as usize)..).collect();

        let to_vec = self.supply.get_mut(&to).unwrap();
        for b in boxes {
            to_vec.push(b);
        }
    }

    pub fn get_top(&self) -> String {
        let mut top = String::new();

        for i in 0..self.supply.len() {
            let stack = self.supply.get(&(i as u32)).unwrap();
            top.push(stack.last().copied().unwrap());
        }

        top
    }
}

pub fn instruction_from_str(s: &str) -> (u32, u32, u32) {
    let mut iter = s.split_whitespace();
    iter.next(); // skip "move"
    let n: u32 = iter.next().unwrap().parse().unwrap();
    iter.next(); // skip "from"
    let from: u32 = iter.next().unwrap().parse().unwrap();
    iter.next(); // skip "to"
    let to: u32 = iter.next().unwrap().parse().unwrap();

    (n, from - 1, to - 1)
}

pub fn part_one(input: &str) -> Option<String> {
    let mut supply = SupplyStacks::build(input);

    let mut iter = input.split("\n\n");
    iter.next();

    for line in iter.next().unwrap().lines() {
        let inst = instruction_from_str(line);
        supply.move_stack(inst.0, inst.1, inst.2);
    }

    Some(supply.get_top())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut supply = SupplyStacks::build(input);

    let mut iter = input.split("\n\n");
    iter.next();

    for line in iter.next().unwrap().lines() {
        let inst = instruction_from_str(line);
        supply.move_vec(inst.0, inst.1, inst.2);
    }

    Some(supply.get_top())
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    mod day5 {
        use super::super::*;

        #[test]
        fn test_part_one() {
            let input = aoc::read_file("examples", 5);
            assert_eq!(part_one(&input), Some(String::from("CMZ")));
        }

        #[test]
        fn test_part_two() {
            let input = aoc::read_file("examples", 5);
            assert_eq!(part_two(&input), Some(String::from("MCD")));
        }
    }
}
