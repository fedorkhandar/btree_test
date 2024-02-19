/*
Create a binary tree with n nodes.
Calc the sum of the values of the nodes at each level.
 */

// DONE: HashMap -> Vec
// TODO: Rc<RefCell> -> <Box>

// TODO: stack -> recursion

// TODO: cargo clippy
// TODO: tree build

// use std::cell::RefCell;
// use std::rc::Rc;

use std::borrow::Borrow;

//Debug -- for output, PartialEq -- for cmp, Eq -- for caching
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    // pub left: Option<Rc<RefCell<TreeNode>>>,
    // pub right: Option<Rc<RefCell<TreeNode>>>,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>
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
#[warn(dead_code)]
fn create_tree_dfs(n: i32) -> Option<Box<TreeNode>> {
    let depth = (f32::log2(n as f32) as i32) + 1;
    let mut k = 1;
    let root = Box::new(TreeNode::new(0)); 
    // root is a ptr to the TreeNode
    // TreeNode is a struct with val, left, right
    // Box::new is a ptr to the TreeNode
    // Box::new(TreeNode::new(0)) is a ptr to the TreeNode with val = 0
    // TreeNode is stored in the heap
    // Box::new is a ptr to the heap

    let mut stack = vec![(root, 1)];
    // root is not valid anymore, it is now a part of the stack, 
    // now stack owns it, stack[0] is a ptr to the TreeNode with val = 0

    while k < n {
        if let Some((mut x, x_level)) = stack.pop() {
            if x_level < depth {
                if x.left.is_none() {
                    let left = Box::new(TreeNode::new(k));
                    x.left = Some(left);
                    let mut y = x.left.as_ref().unwrap();
                    k = k+1;
                    stack.push((x, x_level));  
                    stack.push((*y, x_level + 1));
                }
                // else if x.right.is_none() {
                //     let right = Box::new(TreeNode::new(k));
                //     k = k+1;
                //     stack.push((x, x_level));
                //     stack.push((&x.right.unwrap(), x_level + 1));
                // }
            } 
        } else {
            println!("Stack is empty!");
            break;
        }
    //     match stack.pop() {
    //         Some((mut x, x_level)) => {
    //             if x_level < depth {
    //                 if x.left.is_none() {
    //                     x.left = Some(Box::new(TreeNode::new(k)));
    //                     k = k+1;
    //                     // x.left = &left;
    //                     stack.push((&x, x_level));  
    //                     stack.push((&(x.left.unwrap()), x_level + 1));
    //                 }
    //                 else if x.right.is_none() {
    //                     x.right = Some(Box::new(TreeNode::new(k)));
    //                     k = k+1;
    //                     // x.right = &right;
    //                     stack.push((x, x_level));
    //                     stack.push((&(x.right.unwrap()), x_level + 1));
    //                 }
    // //                 println!("{}", x_level);
    //             }
    //         }
    //         None => {
    //             println!("Stack is empty!");
    //             break;
    //         }
    //     }
    }
    
    None
}

// fn print_tree(root: Option<Rc<RefCell<TreeNode>>>, indent: i32) {
//     if let Some(node) = root {
//         for _ in 0..indent {
//             print!("  "); 
//         }
//         println!("{}", node.borrow().val);
//         print_tree(node.borrow().left.clone(), indent + 1);
//         print_tree(node.borrow().right.clone(), indent + 1);
//     }
// }

// fn calc_tree(root: Option<Rc<RefCell<TreeNode>>>, depth: u32) -> Option<Vec<i32>> {
//     if let Some(node) = root {
//         let mut d = vec![0; depth as usize + 2];
//         let mut stack = vec![(node.to_owned(), 1)];

//         while !stack.is_empty() {
//             match stack.pop() {
//                 Some((x, x_level)) => {
//                     d[x_level] += x.borrow().val; 
                                    
//                     if !x.borrow().left.is_none() {
//                         stack.push((x.borrow().left.clone().unwrap(), x_level+1));
//                     };
//                     if !x.borrow().right.is_none() {
//                         stack.push((x.borrow().right.clone().unwrap(), x_level+1));
//                     }
//                 }
//                 None => {break;}
//             }
//         }
//         Some(d)
//     }
//     else {
//         None
//     }
// }


// fn solution(depth: u32) {
//     // d-1 is a depth of the full btree 
//     // n is a number of nodes in the full btree = 2^d - 1
//     let n = 2i32.pow(depth + 1) - 1;
//     let root = create_tree_dfs(n);
//     // print_tree(root.clone(), 0);
//     let result = calc_tree(root.clone(), depth);
//     let result = &result.unwrap()[1..];

//     for element in result.iter().skip(0) {
//         print!("{} ", element);
//     }
// }

fn main() {
    let d = 2;
    // solution(d);
    // let root = Box::new(TreeNode::new(0));
}
