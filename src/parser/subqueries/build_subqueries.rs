use std::collections::HashMap;

use crate::{
    constants::{
        keywords::supqueries::SUBQ_PATTERN,
        special_chars::{DOUBLE_QUOTE, SINGLE_QUOTE, subqueries::SUBQ_END},
    },
    parser::{errors::{ParseSubqueryError, ParseSubqueryErrorReason}, objects::Subquery, subqueries::tree::SubqueryIndexTree},
};

enum Mode {
    Normal,
    StringDQ,
    StringSQ,
    Ended(usize),
}

pub fn build_subquery_index_tree(query_str: &str) -> Result<SubqueryIndexTree, ParseSubqueryError> {
    let mut subquery_end: Option<usize> = None;
    let mut mode = Mode::Normal;
    let chars = &query_str.chars();
    let mut level = 0;
    let mut tree = SubqueryIndexTree::new(0);
    tree.indices_map.insert(0, Some(query_str.len()));
    let mut stack: Vec<usize> = vec![];

    for (idx, cur) in chars.clone().enumerate() {
        println!("{cur} - {:?}", stack);
        match mode {
            Mode::Normal => match cur {
                DOUBLE_QUOTE => mode = Mode::StringDQ,
                SINGLE_QUOTE => mode = Mode::StringSQ,
                SUBQ_END => {
                    subquery_end = Some(idx);
                    if level == 0 {
                        mode = Mode::Ended(idx)
                    } else {
                        let rm_val = stack.pop().unwrap();
                        println!("Got query [{rm_val}-{idx}]");
                        let v = tree.indices_map.get_mut(&rm_val).unwrap();
                        *v = Some(idx);
                        level -= 1;
                        println!("Decrementing level from {} to {}", level + 1, level);
                    }
                }
                'S' => {
                    if &query_str[idx..idx + SUBQ_PATTERN.len()] == SUBQ_PATTERN && idx > 0 {
                        println!("Starting subquery {:?}", idx);
                        tree.indices_map.insert(idx, None);
                        stack.push(idx);
                        println!("inserting to tree");
                        tree.insert(idx);
                        level += 1;
                        println!("Updating level from {} to {}", level - 1, level);
                    }
                }
                _ => {}
            },
            Mode::StringDQ => {
                if cur == DOUBLE_QUOTE {
                    mode = Mode::Normal
                }
            }
            Mode::StringSQ => {
                if cur == SINGLE_QUOTE {
                    mode = Mode::Normal
                }
            }
            Mode::Ended(end) => {
                subquery_end = Some(end);
            }
        }
    }

    // let map = tree.indices_map.clone();
    // let a = tree
    //     .map(|node| map.get_key_value(&node.borrow().value))
    //     .collect::<Vec<_>>();
    // println!("Hello world: {:?}", a);
    if level != 0 {
        println!("Ended level != 0");
        return Err(ParseSubqueryError::new(query_str, ParseSubqueryErrorReason::UnexpectedEnd))
    }
    match subquery_end {
        Some(_) => Ok(tree),
        None => Err(ParseSubqueryError::new(query_str, ParseSubqueryErrorReason::UnexpectedEnd)),
    }
}
