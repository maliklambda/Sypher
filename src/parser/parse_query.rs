use super::operations::*;
use super::utils::*;
use crate::constants::special_chars::*;
use crate::parser::errors::*;
use crate::parser::objects::*;
use crate::parser::operations::ops::Operation;
use crate::parser::query::Query;

pub fn parse_query(query: Query) -> Result<QueryObject, ParseQueryError> {
    println!("Parsing: {query}");
    let mut query = prepare_query(query);
    let operation = get_operation(&mut query)?;
    let query_object: QueryObject = match operation {
        Operation::Add => QueryObject::ADD(add::parse_add(&mut query)?),
        Operation::Remove => QueryObject::REMOVE(remove::parse_remove(&mut query)?),
        Operation::Get => QueryObject::GET(get::parse_get(&mut query)?),
        _ => todo!("Other operations of Operation"),
    };
    Ok(query_object)
}

fn prepare_query(query: Query) -> Query {
    let new_query_str = query
        .current
        .strip_suffix(SEMICOLON)
        .unwrap_or(query.current);
    Query::from_str(new_query_str)
}
