/*
Create a binary tree with n nodes.
Calc the sum of the values of the nodes at each level.

 */

// TODO: Rc<RefCell> -> <Box>
// TODO: stack -> recursion
// TODO: HashMap -> Vec
// TODO: cargo clippy
// TODO: tree build

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

//Debug -- for output, PartialEq -- for cmp, Eq -- for caching
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn create_tree_dfs(n: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let depth = (f32::log2(n as f32) as i32) + 1;
    let mut k = 1;
    let root = Rc::new(RefCell::new(TreeNode::new(0)));
    let mut stack = vec![(root.to_owned(), 1)];
    while k < n {
        match stack.pop() {
            Some((x, x_level)) => {
                if x_level < depth {
                    if x.borrow().left.is_none() {
                        let left = Rc::new(RefCell::new(TreeNode::new(k)));
                        k = k+1;
                        x.borrow_mut().left = Some(left.clone());
                        stack.push((x.to_owned(), x_level));
                        stack.push((left.to_owned(), x_level + 1));
                    }
                    else if x.borrow().right.is_none() {
                        let right = Rc::new(RefCell::new(TreeNode::new(k)));
                        k = k+1;
                        x.borrow_mut().right = Some(right.clone());
                        stack.push((x.to_owned(), x_level));
                        stack.push((right.to_owned(), x_level + 1));
                    }
                }
            }
            None => {
                println!("Stack is empty!");
                break;
            }
        }
    }
    
    Some(root)
}

fn print_tree(root: Option<Rc<RefCell<TreeNode>>>, indent: i32) {
    if let Some(node) = root {
        for _ in 0..indent {
            print!("  "); 
        }
        println!("{}", node.borrow().val);
        print_tree(node.borrow().left.clone(), indent + 1);
        print_tree(node.borrow().right.clone(), indent + 1);
    }
}

fn calc_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<HashMap<i32, i32>> {
    if let Some(node) = root {
        let mut d = HashMap::<i32, i32>::new();
        let mut stack = vec![(node.to_owned(), 1)];

        while !stack.is_empty() {
            match stack.pop() {
                Some((x, x_level)) => {
                    let value = d.entry(x_level).or_insert(0);
                    *value += x.borrow().val;                 
                    if !x.borrow().left.is_none() {
                        stack.push((x.borrow().left.clone().unwrap(), x_level+1));
                    };
                    if !x.borrow().right.is_none() {
                        stack.push((x.borrow().right.clone().unwrap(), x_level+1));
                    }
                }
                None => {break;}
            }
        }
        return Some(d);
    }
    else {
        return None;
    }
}


fn print_d_sorted(d: HashMap<i32, i32>) {
    let mut keys: Vec<i32> = d.keys().cloned().collect();
    keys.sort();
    for k in keys {
        print!("{} ", d[&k]);
    }
    println!();
}

fn solution(d: u32) {
    // d-1 is a depth of the full btree 
    // n is a number of nodes in the full btree = 2^d - 1
    let n = 2i32.pow(d+1) - 1;
    let root = create_tree_dfs(n);
    // print_tree(root.clone(), 0);
    let d = calc_tree(root.clone());
    print_d_sorted(d.unwrap());
}

fn main(){
    let d = 2;
    solution(d);
}