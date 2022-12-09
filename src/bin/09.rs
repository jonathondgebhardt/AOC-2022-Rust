use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    pub fn new() -> Self {
        Knot { x: 0, y: 0 }
    }

    pub fn move_to(&mut self, d: Direction) {
        match d {
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
        }
    }

    pub fn update(&mut self, other: Knot) {
        if self.distance_to(other) > 1 {
            if self.is_adjacent_to(other) {
                // move UDLR towards other
                if self.x != other.x {
                    if self.x < other.x {
                        self.x += 1;
                    } else {
                        self.x -= 1;
                    }
                } else {
                    if self.y < other.y {
                        self.y += 1;
                    } else {
                        self.y -= 1;
                    }
                }
            } else {
                // move diagonally towards other
                if self.x < other.x {
                    self.x += 1;
                } else {
                    self.x -= 1;
                }

                if self.y < other.y {
                    self.y += 1;
                } else {
                    self.y -= 1;
                }
            }
        }
    }

    pub fn distance_to(&self, other: Knot) -> u32 {
        (((self.x.abs_diff(other.x).pow(2) + self.y.abs_diff(other.y).pow(2)) as f64).sqrt()) as u32
    }

    pub fn is_adjacent_to(&self, other: Knot) -> bool {
        return (self.x == other.x && self.y != other.y)
            || (self.x != other.x && self.y == other.y);
    }
}

pub struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    pub fn build(n: usize) -> Self {
        let mut knots: Vec<Knot> = Vec::new();
        for _ in 0..n {
            knots.push(Knot::new());
        }

        Rope { knots }
    }

    pub fn move_to(&mut self, d: Direction) {
        let mut prev = None;
        for knot in self.knots.iter_mut() {
            match prev {
                Some(prev) => {
                    knot.update(prev);
                }
                None => {
                    knot.move_to(d);
                }
            }

            prev = Some(knot.clone());
        }
    }

    pub fn get_tail(&self) -> Knot {
        *self.knots.last().unwrap()
    }
}

pub fn get_instruction(s: &str) -> (Direction, u32) {
    let mut iter = s.split_whitespace();

    let direction = match iter.next().unwrap() {
        "U" => Direction::Up,
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        _ => panic!("cannot move in unrecognized direction"),
    };

    let magnitude = iter.next().unwrap().parse::<u32>().unwrap();

    (direction, magnitude)
}

pub fn get_unique_tails(n: usize, s: &str) -> u32 {
    let mut rope = Rope::build(n);
    let mut unique_tails = HashSet::new();
    unique_tails.insert(rope.get_tail());

    for line in s.lines() {
        let (dir, mag) = get_instruction(line);
        for _ in 0..mag {
            rope.move_to(dir);
            unique_tails.insert(rope.get_tail());
        }
    }

    unique_tails.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(get_unique_tails(2, input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(get_unique_tails(10, input))
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    mod day9 {
        use super::super::*;

        #[test]
        fn test_part_one() {
            let input = aoc::read_file("examples", 9);
            assert_eq!(part_one(&input), Some(13));
        }

        #[test]
        fn test_part_two() {
            let input = aoc::read_file("examples", 9);
            assert_eq!(part_two(&input), Some(1));
        }

        #[test]
        fn test_part_two_large_sample() {
            let input = String::from(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
            );

            assert_eq!(part_two(&input), Some(36));
        }
    }
}
