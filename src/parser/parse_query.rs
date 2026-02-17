use std::collections::HashMap;
use std::hash::Hash;

use crate::constants::limits::MAX_IDENTIFIER_LEN;
use crate::parser::objects::*;
use crate::parser::errors::*;
use crate::constants::special_chars::*;
use crate::constants::keywords::*;
use crate::parser::operations::keywords::Operation;




pub fn parse_query (query: String) -> Result<QueryObject, ParseQueryError> {
    println!("Parsing: {query}");
    let query = prepare_query(query);
    let (operation, query_rest) = get_operation(&query)?;
    let query_object: QueryObject = match operation {
        Operation::Add => QueryObject::ADD(parse_add(query_rest)?),
        Operation::Remove => QueryObject::REMOVE(parse_remove(query_rest)?),
        _ => todo!("Other operations of Operation"),
    };
    Ok(query_object)
}


fn prepare_query(query: String) -> String {
    let query = query.strip_suffix(SEMICOLON).unwrap_or(&query);

    query.to_owned()
}



fn get_operation (query: &str) -> Result<(Operation, &str), ParseQueryError> {
    let (keyword_str, query_rest) = query.split_once(SPACE)
        .ok_or(ParseQueryError::new(ParseErrorReason::InvalidKeyword(query.to_string()), 0, query.len()))?;
    let operation = Operation::from_str(keyword_str)
        .ok_or(ParseQueryError {
            reason: ParseErrorReason::InvalidKeyword(keyword_str.to_string()),
            error_section: (0, keyword_str.len()),
        })?;
    Ok((operation, query_rest))
}



pub fn parse_add (query_part: &str) -> Result<AddQO, ParseQueryError> {
    println!("Parsing add: {query_part}");
    let (object_kind, query_rest) = get_object_kind(query_part)?;
    let add_query_object = {
        match object_kind {
            ObjectKind::Node => AddQO::Node(parse_add_node(query_rest)?),
            ObjectKind::Relationship => AddQO::Relationship(parse_add_relationship(query_rest)?),
        }
    };
    println!("object kind is {:?}", object_kind);
    Ok(add_query_object)
}



pub fn parse_remove (query_part: &str) -> Result<RemoveQO, ParseQueryError> {
    todo!("parse remove");
}



fn get_object_kind (query: &str) -> Result<(ObjectKind, &str), ParseQueryError> {
    let (object_kind_str, query_rest) = query.split_once(SPACE)
        .ok_or(
            ParseQueryError::new(
                ParseErrorReason::InvalidObjectKind(query.to_string()), 0, query.len()
            )
        )?;
    let object_kind = ObjectKind::from_str(object_kind_str)
        .ok_or(
            ParseQueryError::new(
                ParseErrorReason::InvalidObjectKind(query.to_string()), 0, object_kind_str.len()
            )
        )?;
    Ok((object_kind, query_rest))
}




#[derive(Clone, Debug)]
pub enum ObjectKind {
    Node,
    Relationship,
}

impl ObjectKind {
    const STRINGS: &'static [(&'static str, Self)] = &[
        (NODE_STR, ObjectKind::Node),
        (RELATIONSHIP_STR, ObjectKind::Relationship),
    ];

    pub fn from_str (s: &str) -> Option<ObjectKind> {
        let (_, kind) = Self::STRINGS.iter()
            .find(|(value, _)| value == &s)?;
        Some(kind.clone())
    }
}




fn parse_add_node (query: &str) -> Result<AddNodeQO, ParseQueryError> {
    println!("parsing add node: {query}");
    let (identifier, query) = get_identifier(query)?;
    println!("identifier: {identifier}");
    let (type_name, query) = get_type_name(query)?;
    println!("typename: {type_name}");
    println!("query after type name: {query}");
    let properties = parse_properties(query)?;
    Ok(AddNodeQO { 
        identifier: identifier.to_string(),
        type_name: type_name.to_string(),
        properties,
    })
}


fn parse_add_relationship (query: &str) -> Result<AddRelationshipQO, ParseQueryError> {
    println!("parsing add relationship: {query}");
    todo!("finish parse add relationship");
}



fn get_identifier(query: &str) -> Result<(&str, &str), ParseQueryError> {
    let (identifier, query_rest) = query.split_once(SPACE)
        .ok_or(ParseQueryError::new(ParseErrorReason::IdentifierMissingType, 0, query.len()))?;
    if identifier.len() > MAX_IDENTIFIER_LEN {
        return Err(ParseQueryError::new(ParseErrorReason::TooLongIdentifier(identifier.len(), MAX_IDENTIFIER_LEN), 0, 1));
    }
    println!("Query after identifier: {query_rest}");
    Ok((identifier, query_rest))
}


fn get_type_name(query: &str) -> Result<(&str, &str), ParseQueryError> {
    println!("query: {query}");
    let (expected_type, query) = query.split_once(SPACE)
        .ok_or(ParseQueryError::new(ParseErrorReason::IdentifierMissingType, 0, query.len()))?;
    if expected_type != TYPE_STR {
        return Err(ParseQueryError::new(ParseErrorReason::IdentifierMissingType, 0, query.len()))
    }
    let (type_name, query_rest) = query.split_once(SPACE)
        .ok_or(ParseQueryError::new(ParseErrorReason::IdentifierMissingType, 0, query.len()))?;
    println!("end query: {query_rest}");
    Ok((type_name, query_rest))
}

fn parse_properties (query: &str) -> Result<HashMap<String, String>, ParseQueryError> {
    if !query.trim_start().starts_with(PROPERTIES_STR) {
        return Err(ParseQueryError::new(ParseErrorReason::ParseKeyValuePairs, 0, 1));
    }
    let mut query = query.trim_start().strip_prefix(PROPERTIES_STR).unwrap();

    if !query.starts_with(SPACE) {
        return Err(ParseQueryError::new(ParseErrorReason::ParseKeyValuePairs, 0, 1));
    }

    let mut properties: HashMap<String, String> = HashMap::new();
    println!("Parsing properties for <{query}>");
    while !query.trim_start().starts_with(SEMICOLON) && !query.trim().is_empty() {
        println!("query == <{query}>");
        query = parse_kv_pair(query, &mut properties)
            .map_err(|err| ParseQueryError::new(
                ParseErrorReason::ParseKeyValuePairs, 0, 1
            ))?;
    }
    Ok(properties)
}



// Parse key value pairs
// Example: name = 'Malik', age = 20, occupation = 'SWE', ...
// Expects the current query to start WITHOUT A COMMA!
// Whitespace is okay.
fn parse_kv_pair <'a>(query: &'a str, properties: &mut HashMap<String, String>) -> Result<&'a str, ParseKeyValueError> {
    let (key, query) = query.split_once(ASSIGNMENT)
        .ok_or(ParseKeyValueError::new())?;
    let key = key.trim();
    let query = query.trim_start();
    let (value_str, mut query) = {
        if query.starts_with(DOUBLE_QUOTE) {
            println!("Got String value double quotes");
            let end = *query.find(DOUBLE_QUOTE).iter().nth(2).ok_or(ParseKeyValueError::new())?;
            query.split_at(end)
        } else if query.starts_with(SINGLE_QUOTE) {
            println!("Got String value single quotes");
            let end = query[1..].find(SINGLE_QUOTE).ok_or(ParseKeyValueError::new())? +2;
            println!("end == {end}");
            query.split_at(end)
        } else {
            println!("query: {query}");
            let end = query.find(COMMA).unwrap_or(query.len());
            query.split_at(end)
        }
    };

    properties.insert(key.to_string(), value_str.to_string());
    if query.trim_start().starts_with(COMMA) {
        query = &query[1..];
    }

    println!("key = {key}, value_str = <{value_str}>");
    println!("Remaining query: {:?}", query);
    // todo!("Finish parsing single kv-pair");
    Ok(query)
}




