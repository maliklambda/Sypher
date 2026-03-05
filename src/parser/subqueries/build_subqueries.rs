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
#[derive(Debug)]
pub enum IterMode {
    Normal,
    StringDQ,
    StringSQ,
    Skip(usize),
    Ended(usize),
}

/*
* Returns QueryTree in unparsed state
* -> tree.indices_map holds only the end indices
*/
pub fn build_indexed_query_tree(query_str: &str) -> Result<QueryTree, ParseSubqueryError> {
    let mut has_subqueries = false;
    let mut subquery_end: Option<usize> = None;
    let mut mode = IterMode::Normal;
    let chars = &query_str.chars();
    let mut level = 0;
    let mut tree = QueryTree::new(0);
    tree.indices_map
        .insert(0, Some(SubqueryPayload::new(query_str.len())));
    let mut stack: Vec<usize> = vec![];

    for (idx, cur) in chars.clone().enumerate() {
        match mode {
            IterMode::Normal => match cur {
                DOUBLE_QUOTE => mode = IterMode::StringDQ,
                SINGLE_QUOTE => mode = IterMode::StringSQ,
                SUBQ_END => {
                    subquery_end = Some(idx);
                    if level == 0 {
                        mode = IterMode::Ended(idx)
                    } else {
                        let rm_val = stack.pop().unwrap();
                        let v = tree.indices_map.get_mut(&rm_val).unwrap();
                        *v = Some(SubqueryPayload::new(idx + 1));
                        level -= 1;
                    }
                }
                'S' => {
                    if &query_str[idx..idx + SUBQ_PATTERN.len()] == SUBQ_PATTERN && idx > 0 {
                        has_subqueries = true;
                        tree.indices_map.insert(idx, None);
                        stack.push(idx);
                        tree.insert(idx);
                        level += 1;
                    }
                }
                _ => {}
            },
            IterMode::StringDQ => {
                if cur == DOUBLE_QUOTE {
                    mode = IterMode::Normal
                }
            }
            IterMode::StringSQ => {
                if cur == SINGLE_QUOTE {
                    mode = IterMode::Normal
                }
            }
            IterMode::Ended(end) => {
                subquery_end = Some(end);
            }
            _ => todo!("handle edge case: {:?}", mode),
        }
    }
    tree.clear_current_nodes();
    tree.clear_queue();
    if level != 0 {
        return Err(ParseSubqueryError::new(
            query_str,
            ParseSubqueryErrorReason::NonZeroLevel,
        ));
    }
    match subquery_end {
        Some(_) => Ok(tree),
        None => {
            if !has_subqueries {
                return Ok(tree);
            }
            Err(ParseSubqueryError::new(
                query_str,
                ParseSubqueryErrorReason::UnexpectedEnd,
            ))
        }
    }
}
