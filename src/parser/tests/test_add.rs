use std::collections::HashMap;

use crate::parser::{
    errors::{ParseErrorReason, ParseKeyValueError, ParseKeyValueErrorReason, ParseQueryError},
    objects::{
        QueryObject,
        add::{AddNodeQO, AddQO, AddRelationshipQO},
    },
    parse_query::parse_query,
    query::Query,
};

#[test]
fn test_add_node() {
    let expected_qo = QueryObject::ADD(AddQO::Node(AddNodeQO {
        identifier: "n1".to_string(),
        type_name: "Person".to_string(),
        properties: HashMap::from([
            ("name".to_string(), "Edos".to_string()),
            ("age".to_string(), "20".to_string()),
        ]),
    }));
    let query = Query::from_str("ADD NODE n1 TYPE Person PROPERTIES name='Edos', age=20");
    assert_eq!(parse_query(query).unwrap(), expected_qo);
    let query = Query::from_str("ADD NODE n1 TYPE Person PROPERTIES name='Edos', age=20;");
    assert_eq!(parse_query(query).unwrap(), expected_qo);
    let query = Query::from_str("ADD NODE n1 TYPE Person PROPERTIES age=20, name='Edos'");
    assert_eq!(parse_query(query).unwrap(), expected_qo);
}

#[test]
fn test_add_node_fails() {
    let invalid_query = Query::from_str("ADD NODE TYPE");
    assert!(parse_query(invalid_query).is_err());

    let invalid_query = Query::from_str("ADD NODE n1 TYPE PersonPROPERTIES name='Edos', age=20");
    // type_name = "PersonPROPERTIES" => "PROPERTIES" is missing after type_name
    let res = parse_query(invalid_query);
    match res {
        Err(ParseQueryError {
            reason: ParseErrorReason::ParseKeyValuePairs(ParseKeyValueError { reason: r }),
        }) => assert_eq!(r, ParseKeyValueErrorReason::MissingPropertyStr),
        _ => panic!("Expected query \"{invalid_query}\" to return an error, but it passed"),
    }
}

#[test]
fn test_add_relationship() {
    let expected_qo = QueryObject::ADD(AddQO::Relationship(AddRelationshipQO {
        identifier: "r1".to_string(),
        type_name: "LOVES".to_string(),
        from: 12345,
        to: 54321,
        properties: HashMap::from([
            ("name".to_string(), "Edos".to_string()),
            ("age".to_string(), "20".to_string()),
        ]),
    }));
    let query = Query::from_str(
        "ADD RELATIONSHIP r1 TYPE LOVES FROM 12345 TO 54321 PROPERTIES name='Edos', age=20",
    );
    assert_eq!(parse_query(query).unwrap(), expected_qo);
}

#[test]
fn test_add_relationship_fails() {
    let invalid_query = Query::from_str("ADD RELATIONSHIP TYPE");
    assert!(parse_query(invalid_query).is_err());

    let invalid_query = Query::from_str("ADD RELATIONSHIP n1 PersonPROPERTIES name='Edos', age=20");
    let res = parse_query(invalid_query);
    println!("hello {:?}", res);
    match res {
        Err(ParseQueryError { reason: r }) => {
            assert_eq!(r, ParseErrorReason::IdentifierMissingType)
        }
        _ => panic!("Expected query \"{invalid_query}\" to return an error, but it passed"),
    }
}
