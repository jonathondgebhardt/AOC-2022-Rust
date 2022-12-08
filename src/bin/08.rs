use std::collections::HashSet;

#[derive(PartialEq, Eq, PartialOrd, Hash, Debug, Copy, Clone)]
struct Tree {
    x: u32,
    y: u32,
    height: u32,
}

impl Tree {
    pub fn build(x: u32, y: u32, height: u32) -> Self {
        Tree { x, y, height }
    }
}

#[derive(Debug)]
pub struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    pub fn build(s: &str) -> Self {
        let mut trees: Vec<Vec<Tree>> = Vec::new();
        for (row, line) in s.lines().enumerate() {
            let mut col: u32 = 0;
            let line: Vec<Tree> = line
                .chars()
                .map(|c| {
                    let height = c.to_digit(10).unwrap();
                    let t = Tree::build(row as u32, col, height);
                    col += 1;
                    t
                })
                .collect();

            trees.push(line);
        }

        Forest { trees }
    }

    fn get_is_tree_external(&self, t: &Tree) -> bool {
        t.x == 0
            || t.y == 0
            || t.x == (self.trees.len() as u32) - 1
            || t.y == (self.trees.first().unwrap().len() as u32) - 1
    }

    fn check_north(&self, t: &Tree) -> Option<Tree> {
        for col in self.trees[..t.x as usize].iter().rev() {
            if col[t.y as usize].height >= t.height {
                return Some(col[t.y as usize]);
            }
        }

        None
    }

    fn check_east(&self, t: &Tree) -> Option<Tree> {
        let row = &self.trees[t.x as usize];
        for other in &row[(t.y + 1) as usize..] {
            if other.height >= t.height {
                return Some(*other);
            }
        }

        None
    }

    fn check_south(&self, t: &Tree) -> Option<Tree> {
        for (row, trees) in self.trees.iter().enumerate() {
            if row <= t.x as usize {
                continue;
            } else if trees[t.y as usize].height >= t.height {
                return Some(trees[t.y as usize]);
            }
        }

        None
    }

    fn check_west(&self, t: &Tree) -> Option<Tree> {
        let row = &self.trees[t.x as usize];
        for other in row[..t.y as usize].iter().rev() {
            if other.height >= t.height {
                return Some(*other);
            }
        }

        None
    }

    fn get_is_tree_visible(&self, t: &Tree) -> bool {
        self.get_is_tree_external(t)
            || self.check_north(t) == None
            || self.check_east(t) == None
            || self.check_south(t) == None
            || self.check_west(t) == None
    }

    pub fn get_num_visible_trees(&self) -> u32 {
        let mut visible_trees: HashSet<Tree> = HashSet::new();

        for row in &self.trees {
            for tree in row {
                if self.get_is_tree_visible(tree) {
                    visible_trees.insert(*tree);
                }
            }
        }

        visible_trees.len() as u32
    }

    fn get_scenic_score(&self, t: &Tree) -> u32 {
        if self.get_is_tree_external(t) {
            0
        } else {
            let north = match self.check_north(t) {
                Some(other) => t.x.abs_diff(other.x),
                None => t.x,
            };
            let east = match self.check_east(t) {
                Some(other) => t.y.abs_diff(other.y),
                None => t.y.abs_diff(self.trees.first().unwrap().len() as u32 - 1),
            };
            let south = match self.check_south(t) {
                Some(other) => t.x.abs_diff(other.x),
                None => t.x.abs_diff(self.trees.len() as u32 - 1),
            };
            let west = match self.check_west(t) {
                Some(other) => t.y.abs_diff(other.y),
                None => t.y,
            };

            north * east * south * west
        }
    }

    pub fn get_max_scenic_score(&self) -> u32 {
        let mut max = std::u32::MIN;

        for row in &self.trees {
            max = std::cmp::max(
                max,
                row.iter().map(|t| self.get_scenic_score(t)).max().unwrap(),
            )
        }

        max
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let forest = Forest::build(input);
    Some(forest.get_num_visible_trees())
}

pub fn part_two(input: &str) -> Option<u32> {
    let forest = Forest::build(input);
    Some(forest.get_max_scenic_score())
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    mod day8 {
        use super::super::*;

        #[test]
        fn test_part_one() {
            let input = aoc::read_file("examples", 8);
            assert_eq!(part_one(&input), Some(21));
        }

        #[test]
        fn test_part_two() {
            let input = aoc::read_file("examples", 8);
            assert_eq!(part_two(&input), Some(8));
        }
    }
}
