use crate::parser::{
    objects::{GetQO, QueryObject},
    parse_query::parse_query,
    query::Query,
};

#[test]
fn test_get_node() {
    let expected_qo = QueryObject::GET(GetQO::Node(76390));
    let query = Query::from_str("GET NODE 76390");
    assert_eq!(parse_query(query).unwrap(), expected_qo);
}

fn test_get_node_fails() {
    let invalid_query = Query::from_str("GET NODE ");
    assert!(parse_query(invalid_query).is_err());
}

#[test]
fn test_get_relationship() {
    let expected_qo = QueryObject::GET(GetQO::Relationship(54321));
    let query = Query::from_str("GET RELATIONSHIP 54321");
    assert_eq!(parse_query(query).unwrap(), expected_qo);
}

fn test_get_relationship_fails() {
    let invalid_query = Query::from_str("GET RELATIONSHIP ");
    assert!(parse_query(invalid_query).is_err());
}
