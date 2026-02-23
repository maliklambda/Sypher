use crate::parser::{
    errors::ParseQueryError,
    objects::{QueryObject, query_tree::QueryTree},
    operations::{add, get, ops::Operation, parse_match, remove, update},
    query::Query,
    subqueries::build_subqueries::build_subquery_index_tree,
    utils::get_operation,
};

pub fn parse_query(mut query: Query) -> Result<QueryTree, ParseQueryError> {
    println!("Parsing: {query}");
    query.prepare();
    let sqit = build_subquery_index_tree(query.current)?;
    println!("Got subquery index tree: {:?}", sqit);
    println!("{query}");
    let query_tree = QueryTree::from_raw_tree(sqit, &query)?;
    // for query in query_tree {
    //     parse_single_query(query)?;
    // }
    Ok(query_tree)
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
