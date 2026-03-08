use crate::parser::{
    errors::ParseQueryError,
    objects::QueryObject,
    operations::{add, get, ops::Operation, parse_match, remove, update},
    query::Query,
    subqueries::{self, build_subqueries::build_indexed_query_tree, tree::QueryTree},
    utils::get_operation,
};

pub fn parse_query(mut query: Query) -> Result<QueryTree, ParseQueryError> {
    println!("Parsing: {query}");
    query.prepare();
    let mut query_tree = build_indexed_query_tree(query.current)?;
    println!("Got subquery index tree: {:?}", query_tree);
    let mut query_nodes = query_tree.clone().collect::<Vec<_>>();
    /*
     * TODO: remove .clone() call and iterate only over references
     * All references are currently stored in root.children
     * As of now, every step in the traversal clones the entire subtree under the node.
     */
    query_nodes.reverse();
    for query_node in query_nodes {
        let start_idx = query_node.borrow().value;
        let map_entry = query_tree
            .indices_map
            .get_mut(&start_idx)
            .expect("Value has not been set")
            .as_mut()
            .unwrap();
        let mut current_query = subqueries::remove_subquery_str(Query::from_str(
            &query.current[start_idx..map_entry.query_section_end],
        ));
        println!("Ready to parse {current_query}");
        map_entry.query_object = Some(parse_single_query(&mut current_query)?);
    }
    Ok(query_tree)
}

pub fn parse_single_query(query: &mut Query) -> Result<QueryObject, ParseQueryError> {
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
