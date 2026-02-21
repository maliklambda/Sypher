use crate::{
    constants::{
        keywords::supqueries::SUBQ_PATTERN,
        special_chars::{DOUBLE_QUOTE, QUERY_SEPARATOR, SINGLE_QUOTE, subqueries::SUBQ_END},
    },
    parser::{
        errors::ParseQueryError,
        objects::{QueryObject, Subquery},
        operations::{add, get, ops::Operation, parse_match, remove, update},
        query::Query,
        utils::get_operation,
    },
};

pub fn parse_query(query: Query) -> Result<QueryObject, ParseQueryError> {
    println!("Parsing: {query}");
    let mut query = prepare_query(query);
    let operation = get_operation(&mut query)?;
    let query_object: QueryObject = match operation {
        Operation::Add => QueryObject::Add(add::parse_add(&mut query)?),
        Operation::Remove => QueryObject::Remove(remove::parse_remove(&mut query)?),
        Operation::Get => QueryObject::Get(get::parse_get(&mut query)?),
        Operation::Match => QueryObject::Match(parse_match::parse_match(&mut query)?),
        Operation::Update => QueryObject::Update(update::parse_update(&mut query)?),
    };
    Ok(query_object)
}

fn prepare_query(query: Query) -> Query {
    let new_query_str = query
        .current
        .strip_suffix(QUERY_SEPARATOR)
        .unwrap_or(query.current)
        .trim();
    let subqueries = get_subqueries(new_query_str);
    Query::from_str(new_query_str)
}

fn get_subqueries(query: &str) -> Vec<Subquery> {
    let mut new_query_str = query.to_string();
    let subquery_idc: Vec<usize> = new_query_str
        .match_indices(SUBQ_PATTERN)
        .map(|(idx, _)| idx)
        .collect();
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
    let mut pos = subquery_start;
    let mut subquery_end: Option<usize> = None;
    let mut pos = 0;
    let mut mode = Mode::Normal;
    let chars = &query_str[subquery_start..].chars();
    for (idx, cur) in chars.clone().enumerate() {
        println!("{cur}");
        match mode {
            Mode::Normal => match cur {
                DOUBLE_QUOTE => mode = Mode::StringDQ,
                SINGLE_QUOTE => mode = Mode::StringSQ,
                SUBQ_END => mode = Mode::Ended(idx),
                'S' => {
                    if &query_str[subquery_start + idx..subquery_start + idx + SUBQ_PATTERN.len()]
                        == SUBQ_PATTERN
                        && idx > 0
                    {
                        todo!("Found recursive subquery...");
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
                break;
            }
        }
    }
    match subquery_end {
        Some(end) => Some(&query_str[subquery_start..=subquery_start + end]),
        None => None,
    }
}
