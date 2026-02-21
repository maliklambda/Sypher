use crate::parser::{parse_query::parse_query, query::Query};

mod constants;
mod examples;
mod parser;
mod types;

fn main() {
    let query = Query::from_str(examples::MATCH_QUERY_INGOING);

    match parse_query(query) {
        Ok(result) => println!("Query parsed successfully. Query object: {:?}", result),
        Err(err) => println!("Error parsing query: {:?}", err),
    }
}
