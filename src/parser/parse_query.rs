use crate::parser::{
    errors::ParseQueryError,
    objects::QueryObject,
    operations::{add, get, ops::Operation, parse_match, remove, update},
    query::Query,
    subqueries::{build_subqueries::build_uparsed_query_tree, tree::QueryTree},
    utils::get_operation,
};

pub fn parse_query(mut query: Query) -> Result<QueryTree, ParseQueryError> {
    println!("Parsing: {query}");
    query.prepare();
    let query_tree = build_uparsed_query_tree(query.current)?;
    println!("Got subquery index tree: {:?}", query_tree);
    println!("{query}");
    // let parsed_sqit = unparsed_sqit.fill();
    for query_node in query_tree {
        println!("Query node: {:?}", query_node);
    }
    todo!("Return QueryTree from parse_query");
}

fn parse_single_query<'a>(query: &'a mut Query) -> Result<QueryObject, ParseQueryError<'a>> {
    let operation = get_operation(query)?;
    let query_object: QueryObject = match operation {
        Operation::Add => QueryObject::Add(add::parse_add(query)?),
        Operation::Remove => QueryObject::Remove(remove::parse_remove(query)?),
        Operation::Get => QueryObject::Get(get::parse_get(query)?),
        Operation::Match => QueryObject::Match(parse_match::parse_match(query)?),
        Operation::Update => QueryObject::Update(update::parse_update(query)?),
    };
    Ok(query_object)
}
