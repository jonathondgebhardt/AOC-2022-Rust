use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
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

    pub fn is_directory(&self) -> bool {
        !self.children.borrow().is_empty()
    }

    pub fn collect_directories(self_: &Rc<Node>) -> Vec<Rc<Node>> {
        let mut dirs: Vec<Rc<Node>> = Vec::new();

        for child in self_.children.borrow().iter().filter(|c| c.is_directory()) {
            dirs.push(Rc::clone(child));

            let mut child_dirs = Node::collect_directories(child);
            dirs.append(&mut child_dirs);
        }

        dirs
    }
}

pub fn build_fs(s: &str, root: &Rc<Node>) {
    let mut node = Rc::clone(&root);

    let mut iter = s.lines();
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
}

pub fn part_one(input: &str) -> Option<u32> {
    let root = Node::build(None, String::from("/"));
    build_fs(input, &root);

    let dirs = Node::collect_directories(&root);
    let mut size = 0;
    for dir in dirs {
        let dir_size = dir.get_size();
        if dir_size <= 100_000 {
            size += dir_size;
        }
    }

    Some(size)
}

pub fn part_two(input: &str) -> Option<u32> {
    let root = Node::build(None, String::from("/"));
    build_fs(input, &root);

    let mut doomed_size = root.get_size();
    let total_disk_space = 70_000_000;
    let remaining_space = total_disk_space - doomed_size;

    let upgrade_size = 30_000_000;
    let required_space = upgrade_size - remaining_space;

    let dirs = Node::collect_directories(&root);
    for dir in dirs {
        let dir_size = dir.get_size();
        if dir_size >= required_space && dir_size < doomed_size {
            doomed_size = dir_size;
        }
    }

    Some(doomed_size)
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
            assert_eq!(part_two(&input), Some(24933642));
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
