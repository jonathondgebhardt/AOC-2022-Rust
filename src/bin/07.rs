use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    size: Option<u32>,
    name: String,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn build(size: Option<u32>, name: String) -> Rc<Self> {
        Rc::new(Node {
            size,
            name,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }

    pub fn add_child(self_: &Rc<Node>, child: &Rc<Node>) {
        child.parent.replace(Rc::downgrade(&self_));
        self_.children.borrow_mut().push(Rc::clone(&child));
    }

    pub fn get_child(&self, name: &str) -> Option<Rc<Node>> {
        for child in self.children.borrow().iter() {
            if child.name == name {
                return Some(Rc::clone(child));
            }
        }

        None
    }

    pub fn get_parent(self_: &Rc<Node>, deep: bool) -> Option<Rc<Node>> {
        if deep {
            if let Some(parent) = self_.parent.borrow().upgrade() {
                Node::get_parent(&parent, deep)
            } else {
                Some(Rc::clone(self_))
            }
        } else {
            self_.parent.borrow().upgrade()
        }
    }

    pub fn get_size(&self) -> u32 {
        let mut size: u32 = self.size.unwrap_or_default();
        for child in self.children.borrow().iter() {
            size += child.get_size();
        }

        size
    }

    pub fn get_size_with_ceiling(&self, ceiling: u32) -> u32 {
        let mut size: u32 = self.size.unwrap_or_default();
        for child in self.children.borrow().iter() {
            let child_size = child.get_size_with_ceiling(ceiling);
            if child_size <= ceiling {
                size += child_size;
            }
        }

        size
    }

    pub fn is_directory(&self) -> bool {
        !self.children.borrow().is_empty()
    }

    pub fn collect_directories(&self) -> Vec<Rc<Node>> {
        let mut dirs: Vec<Rc<Node>> = Vec::new();

        for child in self.children.borrow().iter().filter(|c| c.is_directory()) {
            dirs.push(Rc::clone(child));

            let mut child_dirs = child.collect_directories();
            dirs.append(&mut child_dirs);
        }

        dirs
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let root = Node::build(None, String::from("/"));
    let mut node = Rc::clone(&root);

    let mut iter = input.lines();
    iter.next();

    for line in iter {
        if &line[0..1] == "$" {
            if &line[2..4] == "cd" {
                let dir = &line[5..];
                if dir == ".." {
                    node = Node::get_parent(&node, false).unwrap();
                } else {
                    node = node.get_child(dir).unwrap();
                }
            }
        } else {
            let mut tokens = line.split_whitespace();
            let size = match tokens.next().unwrap().parse::<u32>() {
                Ok(size) => Some(size),
                _ => None,
            };
            let name = String::from(tokens.next().unwrap());

            let child = Node::build(size, name);
            Node::add_child(&node, &child);
        }
    }

    let dirs = root.collect_directories();
    let mut size = 0;
    for dir in dirs {
        size += dir.get_size_with_ceiling(100_000);
    }

    Some(size)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    mod day7 {
        use super::super::*;

        #[test]
        fn test_part_one() {
            let input = aoc::read_file("examples", 7);
            assert_eq!(part_one(&input), Some(95437));
        }

        #[test]
        fn test_part_two() {
            let input = aoc::read_file("examples", 7);
            assert_eq!(part_two(&input), None);
        }

        #[test]
        fn test_node() {
            let parent = Node::build(Some(1), String::from("parent"));
            assert!(Node::get_parent(&parent, false).is_none());

            {
                let child = Node::build(Some(5), String::from("child"));
                Node::add_child(&parent, &child);
                assert_eq!(parent.children.borrow().len(), 1);
                assert_eq!(parent.get_size(), 6);
            }

            let child = parent.get_child("child");
            assert!(child.is_some());

            let child = child.as_ref().unwrap();
            assert_eq!(child.get_size(), 5);

            let parent = Node::get_parent(&child, false);
            assert!(parent.is_some());

            let parent = parent.as_ref().unwrap();
            assert_eq!(parent.get_size(), 6);

            let grandchild = Node::build(Some(2), String::from("grandchild"));
            Node::add_child(child, &grandchild);

            let root = Node::get_parent(&grandchild, true);
            assert!(root.is_some());

            let root = root.as_ref().unwrap();
            assert_eq!(root.name, "parent");
        }
    }
}
