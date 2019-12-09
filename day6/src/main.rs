#![feature(hash_set_entry)]

use std::cell::RefCell;
use std::cmp::Ord;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

const INPUT_PATH: &str = "day6/data/input.txt";

#[derive(Eq, Ord)]
struct TreeNode<T> {
    value: T,
    parent: Option<Rc<RefCell<TreeNode<T>>>>,
    children: BTreeSet<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Debug + Clone> Debug for TreeNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TreeNode {{\n\tvalue: {:?},\n\tparent: {:?},\n\tchildren: {:?}\n}}",
            self.value,
            self.parent.as_ref().map(|v| v.borrow().value.clone()),
            self.children.len()
        )
    }
}

impl<T: PartialEq> std::cmp::PartialEq for TreeNode<T> {
    fn eq(&self, rhs: &Self) -> bool {
        PartialEq::eq(&self.value, &rhs.value)
    }
}

impl<T: PartialOrd> std::cmp::PartialOrd for TreeNode<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.value, &rhs.value)
    }
}

impl<T> TreeNode<T>
where
    T: Eq + Ord + Clone,
{
    fn new(value: T) -> TreeNode<T> {
        TreeNode {
            value,
            parent: None,
            children: BTreeSet::new(),
        }
    }
}

struct Tree<T> {
    nodes: BTreeSet<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Eq + Clone + Ord> Tree<T> {
    fn new() -> Tree<T> {
        Tree {
            nodes: BTreeSet::new(),
        }
    }

    fn get_or_create_node(&mut self, value: &T) -> Rc<RefCell<TreeNode<T>>> {
        let node = self
            .nodes
            .iter()
            .filter(|c| (*c).borrow().value == *value)
            .nth(0);

        match node {
            Some(n) => n.clone(),
            None => {
                let new_node = Rc::new(RefCell::new(TreeNode::new(value.clone())));
                self.nodes.insert(new_node.clone());
                new_node
            }
        }
    }

    fn insert<U: Into<T>>(&mut self, orbiting: U, orbited: U) {
        let orbiting_node = self.get_or_create_node(&orbiting.into());
        let orbited_node = self.get_or_create_node(&orbited.into());

        orbiting_node
            .borrow_mut()
            .parent
            .replace(orbited_node.clone());

        orbited_node.borrow_mut().children.insert(orbiting_node);
    }

    fn count<U: Into<T>>(&self, starting: U) -> usize {
        let mut total = 0usize;
        let root: T = starting.into();

        for e in self.nodes.iter() {
            let mut current_orbits = 0usize;
            let mut current = e.clone();

            loop {
                if current.borrow().value == root {
                    break;
                }

                current_orbits += 1;
                let new_current = match &current.borrow().parent {
                    Some(n) => n.clone(),
                    None => {
                        current_orbits = 0;
                        break;
                    }
                };

                current = new_current;
            }

            total += current_orbits;
        }

        total
    }

    fn path<U: Into<T>>(&self, from: U) -> Vec<Rc<RefCell<TreeNode<T>>>> {
        let from_owned: T = from.into();
        let node = self
            .nodes
            .iter()
            .filter(|n| n.borrow().value == from_owned)
            .nth(0)
            .unwrap();

        let mut path: Vec<Rc<RefCell<TreeNode<T>>>> = Vec::new();
        let mut current = node.clone();
        path.push(current.clone());

        loop {
            let new_current = match &current.borrow().parent {
                Some(n) => {
                    path.push(n.clone());
                    n.clone()
                }
                None => break,
            };

            current = new_current;
        }

        path
    }

    fn jumps<U: Into<T>>(&self, from: U, to: U) -> usize {
        let from_path = self.path(from.into());
        let to_path = self.path(to.into());

        let mut i = 1;
        loop {
            let from_last = from_path.get(from_path.len() - i).unwrap();
            let to_last = to_path.get(to_path.len() - i).unwrap();

            if (from_last != to_last) {
                break;
            }
            i += 1;
        }

        return from_path.len() + to_path.len() - 2 * i;
    }
}

fn read_input<'a>() -> Result<Tree<String>, std::io::Error> {
    let mut tree = Tree::new();

    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.expect("BufRead::lines()::[item]::unwrap()"))
        .map(|l| l.split(')').map(String::from).collect::<Vec<String>>())
        .for_each(|data| {
            let left = data.get(0).unwrap();
            let right = data.get(1).unwrap();

            tree.insert(right, left);
        });

    Ok(tree)
}

fn main() {
    let orbits = read_input().unwrap();
    let total = orbits.count("COM");
    println!("Total orbits: {}", total);

    let jumps = orbits.jumps("YOU", "SAN");
    println!("Total jumps: {}", jumps);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn check_count_all_orbits_for_one_value() {
        let mut orbits: Tree<String> = Tree::new();
        orbits.insert("A", "COM");

        assert_eq!(orbits.count("COM"), 1);
    }

    #[test]
    fn check_count_all_orbits_for_one_level() {
        let mut orbits: Tree<String> = Tree::new();
        orbits.insert("A", "COM");
        orbits.insert("B", "COM");
        orbits.insert("C", "COM");

        assert_eq!(orbits.count("COM"), 3);
    }

    #[test]
    fn check_count_all_orbits_for_nested() {
        let mut orbits: Tree<String> = Tree::new();
        orbits.insert("A", "COM");
        orbits.insert("B", "A");
        orbits.insert("C", "B");

        assert_eq!(orbits.count("COM"), 6);
    }

    fn _check_count_all_orbits_for_example() {
        //      G - H       J - K - L
        //     /           /
        // COM - B - C - D - E - F
        //             \
        //               I
        let mut orbits: Tree<String> = Tree::new();
        orbits.insert("G", "COM");
        orbits.insert("H", "G");

        orbits.insert("B", "COM");
        orbits.insert("C", "B");
        orbits.insert("I", "C");
        orbits.insert("D", "C");
        orbits.insert("E", "D");
        orbits.insert("F", "E");
        orbits.insert("J", "D");
        orbits.insert("K", "J");
        orbits.insert("L", "K");

        assert_eq!(orbits.count("COM"), 42);
    }
}
