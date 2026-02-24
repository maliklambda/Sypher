use std::collections::HashMap;

use crate::parser::{objects::{QueryObject, parse_match::{MatchQO, ReturnValue}}, query::Query, tests::get_root_qo};



#[test]
fn test_match_ingoing() {
    // let expected_qo = QueryObject::Match(MatchQO{filters: vec![], match_objects:, return_values: ReturnValue});
    // let query = Query::from_str("MATCH (p:Person) -[]-> (f:Food)");
    // assert_eq!(get_root_qo(query), expected_qo);
}

#[test]
fn test_match_fails() {
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

