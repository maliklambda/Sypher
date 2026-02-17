use crate::parser::parse_query::parse_query;


mod parser;
mod types;
mod constants;


fn main() {
    let query = "ADD NODE n1 TYPE Person PROPERTIES name=1, age=20".to_string();
    match parse_query(query) {
        Ok(result) => println!("Query parsed successfully. Query object: {:?}", result),
        Err(err) => println!("Error parsing query: {:?}", err)
    }
}



