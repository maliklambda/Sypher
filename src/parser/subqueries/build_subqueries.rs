use std::collections::HashMap;

use crate::{
    constants::{
        keywords::supqueries::SUBQ_PATTERN,
        special_chars::{DOUBLE_QUOTE, SINGLE_QUOTE, subqueries::SUBQ_END},
    },
    parser::{objects::Subquery, subqueries::parse_subqueries::SubqueryTree},
};

// pub fn get_subqueries(query: &str) -> SubqueryTree {
pub fn get_subqueries(query: &str) -> Vec<Subquery> {
    let mut new_query_str = query.to_string();

    let subquery = parse_subquery(&new_query_str, 0).expect("Subquery was not closed properly");
    println!("{subquery}");
    todo!("finish here");

    let subquery_idc: Vec<usize> = new_query_str
        .match_indices(SUBQ_PATTERN)
        .map(|(idx, _)| idx)
        .collect();
    println!("Subquery-idcs: {:?}", subquery_idc);
    for idx in subquery_idc {
        println!("Subquery @{idx}");
        let subquery =
            parse_subquery(&new_query_str, idx).expect("Subquery was not closed properly");
        println!("{subquery}");
    }
    vec![]
}

enum Mode {
    Normal,
    StringDQ,
    StringSQ,
    Ended(usize),
}

fn parse_subquery(query_str: &str, subquery_start: usize) -> Option<&str> {
    let mut subquery_end: Option<usize> = None;
    let mut mode = Mode::Normal;
    let chars = &query_str[subquery_start..].chars();
    let mut level = 0;
    let mut indices_map: HashMap<usize, Option<usize>> = HashMap::new();
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
                        println!("Subquery {:?} is done", rm_val);
                        let v = indices_map.get_mut(&rm_val).unwrap();
                        *v = Some(idx);
                        level -= 1;
                        println!("Decrementing level from {} to {}", level + 1, level);
                    }
                }
                'S' => {
                    if &query_str[subquery_start + idx..subquery_start + idx + SUBQ_PATTERN.len()]
                        == SUBQ_PATTERN
                        && idx > 0
                    {
                        println!("Starting subquery {:?}", idx);
                        indices_map.insert(idx, None);
                        stack.push(idx);
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
    if level != 0 {
        println!("Ended level != 0");
        return None;
    }
    println!("subquery end: {:?}", subquery_end);
    println!("ending with this map: {:?}", indices_map);
    match subquery_end {
        Some(end) => Some(&query_str[subquery_start..=subquery_start + end]),
        None => None,
    }
}
