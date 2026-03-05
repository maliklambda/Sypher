use crate::parser::{parse_query::parse_query, query::Query};

mod constants;
mod examples;
mod parser;
mod types;

fn main() {
    let example_query = examples::MATCH_CONDITIONS;
    let query = Query::from_str(example_query);
    match parse_query(query) {
        Ok(result) => println!("Query parsed successfully. Query object: {:?}", result),
        Err(err) => println!("Error parsing query: {:?}", err),
    }

    // for example_query in EXAMPLE_QUERIES {
    //     let query = Query::from_str(example_query);
    //     match parse_query(query) {
    //         Ok(result) => println!("Query parsed successfully. Query object: {:?}", result),
    //         Err(err) => println!("Error parsing query: {:?}", err),
    //     }
    // }
}
