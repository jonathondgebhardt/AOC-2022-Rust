pub enum Instruction {
    NoOp,
    AddX(i32),
}

impl Instruction {
    pub fn build(s: &str) -> Self {
        let mut iter = s.split_whitespace();

        let inst_type = iter.next().unwrap();
        if inst_type == "noop" {
            Instruction::NoOp
        } else {
            let x = iter.next().unwrap().parse::<i32>().unwrap();
            Instruction::AddX(x)
        }
    }
}

pub struct CPU {
    x: i32,
    cycle: u32,
    current_index: usize,
    waiting_instruction: Option<Instruction>,
    instructions: Vec<Instruction>,
}

impl CPU {
    pub fn build(instructions: Vec<Instruction>) -> Self {
        CPU {
            x: 1,
            cycle: 1,
            current_index: 0,
            waiting_instruction: None,
            instructions,
        }
    }

    pub fn execute(&mut self) {
        if let Some(Instruction::AddX(val)) = self.waiting_instruction {
            self.x += val;
            self.waiting_instruction = None;
        } else if let Instruction::AddX(val) = self.get_instruction() {
            self.waiting_instruction = Some(Instruction::AddX(*val));
        }

        self.cycle += 1;
    }

    fn get_instruction(&mut self) -> &Instruction {
        let inst = &self.instructions[self.current_index];
        self.current_index += 1;
        inst
    }

    pub fn is_done(&self) -> bool {
        self.current_index == self.instructions.len() && self.waiting_instruction.is_none()
    }

    pub fn get_cycle(&self) -> u32 {
        self.cycle
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }
}

pub struct Screen {
    pixel: u32,
}

impl Screen {
    pub fn new() -> Self {
        Self { pixel: 0 }
    }

    pub fn draw(&mut self, cpu: &CPU) {
        let x = cpu.get_x();
        if (x - 1..=x + 1).contains(&(self.pixel as i32)) {
            eprint!("#");
        } else {
            eprint!(".");
        }

        self.pixel += 1;

        if self.pixel % 40 == 0 {
            eprintln!();
            self.pixel = 0;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions: Vec<Instruction> = input.lines().map(Instruction::build).collect();
    let mut cpu = CPU::build(instructions);
    let mut signal_strength = 0;
    let mut inspection_cycle = 20;

    while !cpu.is_done() {
        cpu.execute();
        let cycle = cpu.get_cycle();
        if cycle == inspection_cycle {
            signal_strength += cycle as i32 * cpu.get_x();
            inspection_cycle += 40;
        }
    }

    Some(signal_strength as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions: Vec<Instruction> = input.lines().map(Instruction::build).collect();
    let mut cpu = CPU::build(instructions);
    let mut screen = Screen::new();

    while !cpu.is_done() {
        screen.draw(&cpu);
        cpu.execute();
    }

    None
}

fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    mod day10 {
        use super::super::*;

        #[test]
        fn test_part_one() {
            let input = aoc::read_file("examples", 10);
            assert_eq!(part_one(&input), Some(13140));
        }

        #[test]
        fn test_part_one_small() {
            let input = String::from(
                "noop
addx 3
addx -5",
            );
            assert_eq!(part_one(&input), None);
        }

        #[test]
        fn test_part_two() {
            let input = aoc::read_file("examples", 10);
            assert!(part_two(&input).is_some());
        }
    }
}
