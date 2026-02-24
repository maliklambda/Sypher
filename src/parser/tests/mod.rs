use crate::parser::{objects::QueryObject, parse_query::parse_query, query::Query};

mod test_add;
mod test_get;
mod test_remove;
mod test_update;
mod test_match;
mod test_subquery;

pub fn get_root_qo(query: Query) -> QueryObject {
    let query_tree = parse_query(query).unwrap();
    assert_eq!(query_tree.bfs().len(), 1);
    query_tree.get_root_query_object().unwrap()
}
