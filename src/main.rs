use crate::parser::{parse_query::parse_query, query::Query};

mod constants;
mod parser;
mod types;

fn main() {
    let update_relationship_query =
        Query::from_str("UPDATE NODE 1234 SET name = 'Delcos', REMOVE age, ADD age VALUE 21");
    let update_node_query = Query::from_str(
        "UPDATE NODE 1234 REMOVE height, SET name = 'Delcos', REMOVE age, ADD age VALUE 21",
    );
    let get_relationship_query = Query::from_str("GET RELATIONSHIP 7364");
    let get_node_query = Query::from_str("GET NODE 7364");
    let remove_relationship_query = Query::from_str("REMOVE RELATIONSHIP 12345");
    let remove_node_query = Query::from_str("REMOVE NODE 12345 MODE CASCADE");
    let add_rel_query = Query::from_str(
        "ADD RELATIONSHIP r1 TYPE LOVES FROM 893641 TO 324218436 PROPERTIES since = 2012, reason = 'natural' ",
    );
    let add_node_query =
        Query::from_str("ADD NODE n1 TYPE Person PROPERTIES name = 'Malik', age = 20");

    match parse_query(update_node_query) {
        Ok(result) => println!("Query parsed successfully. Query object: {:?}", result),
        Err(err) => println!("Error parsing query: {:?}", err),
    }
}
