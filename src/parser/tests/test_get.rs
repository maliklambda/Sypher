use crate::parser::{
    objects::{QueryObject, get::GetQO},
    parse_query::parse_query,
    query::Query,
    tests::get_root_qo,
};

#[test]
fn test_get_node() {
    let expected_qo = QueryObject::Get(GetQO::Node(76390));
    let query = Query::from_str("GET NODE 76390");
    assert_eq!(get_root_qo(query), expected_qo);
}

#[test]
fn test_get_node_fails() {
    let invalid_query = Query::from_str("GET NODE ");
    assert!(parse_query(invalid_query).is_err());
}

#[test]
fn test_get_relationship() {
    let expected_qo = QueryObject::Get(GetQO::Relationship(54321));
    let query = Query::from_str("GET RELATIONSHIP 54321");
    assert_eq!(get_root_qo(query), expected_qo);
}

#[test]
fn test_get_relationship_fails() {
    let invalid_query = Query::from_str("GET RELATIONSHIP ");
    assert!(parse_query(invalid_query).is_err());
}
