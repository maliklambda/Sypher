use crate::constants::{command_kws::UPDATE_STR, keywords::NODE_STR};
use crate::parser::errors::ParseErrorReason;
use crate::parser::{
    errors::ParseQueryError,
    objects::{QueryObject, update::UpdateQO},
    parse_query::parse_query,
    query::Query,
};

// #[test]
// fn test_update_node() {
//     let expected_qo = QueryObject::UPDATE(UpdateQO::Node(crate::parser::objects::UpdateNodeQO {
//         id: 1234,
//         operations: vec![],
//     }));
//     let query = Query::from_str("UPDATE NODE 1234 SET a = b");
//     assert_eq!(parse_query(query).unwrap(), expected_qo);
// }

#[test]
fn test_update_node_fails() {
    let invalid_query = Query::from_str("UPDATE NODE 1234");
    match parse_query(invalid_query) {
        Err(ParseQueryError { reason: r }) => {
            println!("{:?}", r);
            assert_eq!(
                r,
                ParseErrorReason::MissingValue {
                    for_keyword: "UPDATE NODE".to_string(),
                }
            );
        }
        _ => panic!("Expected query \"{invalid_query}\" to return an error, but it passed"),
    }
}
