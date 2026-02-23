use crate::{
    constants::special_chars::QUERY_SEPARATOR,
    parser::{
        errors::ParseQueryError,
        objects::QueryObject,
        operations::{add, get, ops::Operation, parse_match, remove, update},
        query::Query,
        subqueries::build_subqueries::get_subqueries,
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
