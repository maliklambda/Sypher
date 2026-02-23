use crate::{
    constants::{
        keywords::supqueries::SUBQ_PATTERN,
        special_chars::{DOUBLE_QUOTE, SINGLE_QUOTE, subqueries::SUBQ_END},
    },
    parser::{
        errors::{ParseSubqueryError, ParseSubqueryErrorReason},
        subqueries::tree::{QueryTree, SubqueryPayload},
    },
};

/*
* This mode is used only during the iteration of subquery-parsing.
* It Shows which state the parsed query is currently in.
*/
enum Mode {
    Normal,
    StringDQ,
    StringSQ,
    Ended(usize),
}

/*
* Returns SubqueryIndexTree in unparsed state
* -> tree.indices_map holds only the end indices
*/
pub fn build_uparsed_query_tree(query_str: &str) -> Result<QueryTree, ParseSubqueryError> {
    let mut subquery_end: Option<usize> = None;
    let mut mode = Mode::Normal;
    let chars = &query_str.chars();
    let mut level = 0;
    let mut tree = QueryTree::new(0);
    tree.indices_map.insert(0, Some(SubqueryPayload::new(query_str.len())));
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
                        *v = Some(SubqueryPayload::new(idx));
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
    tree.clear_current_nodes();
    tree.clear_queue();
    println!("Tree == {:?}", tree);
    if level != 0 {
        return Err(ParseSubqueryError::new(
            query_str,
            ParseSubqueryErrorReason::UnexpectedEnd,
        ));
    }
    match subquery_end {
        Some(_) => Ok(tree),
        None => Err(ParseSubqueryError::new(
            query_str,
            ParseSubqueryErrorReason::UnexpectedEnd,
        )),
    }
}
