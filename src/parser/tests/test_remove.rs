use std::num::ParseIntError;

use crate::parser::{
    errors::{ParseErrorReason, ParseQueryError},
    objects::{
        QueryObject,
        remove::{RemoveMode, RemoveNodeQO, RemoveQO, RemoveRelationshipQO},
    },
    parse_query::parse_query,
    query::Query,
};

#[test]
fn test_remove_node() {
    let expected_qo = QueryObject::REMOVE(RemoveQO::Node(RemoveNodeQO {
        id: 1234,
        mode: RemoveMode::CASCADE,
    }));
    let query = Query::from_str("REMOVE NODE 1234 MODE CASCADE");
    assert_eq!(parse_query(query).unwrap(), expected_qo);
    let query = Query::from_str("REMOVE NODE 1234 MODE CASCADE;");
    assert_eq!(parse_query(query).unwrap(), expected_qo);
    let query = Query::from_str("REMOVE NODE 1234 MODE CASCADE");
    assert_eq!(parse_query(query).unwrap(), expected_qo);
}

#[test]
fn test_remove_node_fails() {
    let invalid_query = Query::from_str("REMOVE NODE 1234 MODE WRONG");
    let res = parse_query(invalid_query);
    match res {
        Err(ParseQueryError { reason: r }) => assert_eq!(r, ParseErrorReason::UnknownRemoveMode),
        _ => panic!("Expected query \"{invalid_query}\" to return an error, but it passed"),
    }
}

#[test]
fn test_remove_relationship() {
    let expected_qo =
        QueryObject::REMOVE(RemoveQO::Relationship(RemoveRelationshipQO { id: 62348 }));
    let query = Query::from_str("REMOVE RELATIONSHIP 62348");
    assert_eq!(parse_query(query).unwrap(), expected_qo);
}

#[test]
fn test_remove_relationship_fails() {
    let invalid_query = Query::from_str("REMOVE RELATIONSHIP");
    let res = parse_query(invalid_query);
    match res {
        Err(ParseQueryError { reason: r }) => assert_eq!(r, ParseErrorReason::InvalidObjectKind),
        _ => panic!("Expected query \"{invalid_query}\" to return an error, but it passed"),
    }
}
