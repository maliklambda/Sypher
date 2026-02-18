use std::collections::HashMap;

use crate::constants::limits::MAX_IDENTIFIER_LEN;
use crate::parser::objects::*;
use crate::parser::errors::*;
use crate::constants::special_chars::*;
use crate::constants::keywords::*;
use crate::parser::operations::keywords::Operation;
use crate::parser::query::Query;




pub fn parse_query (query: Query) -> Result<QueryObject, ParseQueryError> {
    println!("Parsing: {query}");
    let mut query = prepare_query(query);
    let operation = get_operation(&mut query)?;
    let query_object: QueryObject = match operation {
        Operation::Add => QueryObject::ADD(parse_add(&mut query)?),
        Operation::Remove => QueryObject::REMOVE(parse_remove(&mut query)?),
        _ => todo!("Other operations of Operation"),
    };
    Ok(query_object)
}


fn prepare_query(query: Query) -> Query {
    let new_query_str = query.current.strip_suffix(SEMICOLON).unwrap_or(query.current);
    Query::from_str(new_query_str)
}



fn get_operation (query: &mut Query) -> Result<Operation, ParseQueryError> {
    let keyword = query.to_next_space()
        .ok_or(ParseQueryError::new(ParseErrorReason::InvalidKeyword(
            // query.current.to_string()
            "keyword".to_string()
        )))?;
    let operation = Operation::from_str(keyword)
        .ok_or(ParseQueryError {
            reason: ParseErrorReason::InvalidKeyword(keyword.to_string()),
        })?;
    Ok(operation)
}



pub fn parse_add (query: &mut Query) -> Result<AddQO, ParseQueryError> {
    println!("Parsing add: {query}");
    let object_kind = get_object_kind(query)?;
    let add_query_object = {
        match object_kind {
            ObjectKind::Node => AddQO::Node(parse_add_node(query)?),
            ObjectKind::Relationship => AddQO::Relationship(parse_add_relationship(query)?),
        }
    };
    println!("object kind is {:?}", object_kind);
    Ok(add_query_object)
}



pub fn parse_remove (query: &mut Query) -> Result<RemoveQO, ParseQueryError> {
    todo!("parse remove");
}



fn get_object_kind (query: &mut Query) -> Result<ObjectKind, ParseQueryError> {
    let (object_kind_str, query_rest) = query.current.split_once(SPACE)
        .ok_or(
            ParseQueryError::new(
                ParseErrorReason::InvalidObjectKind(query.to_string())
            )
        )?;
    let object_kind = ObjectKind::from_str(object_kind_str)
        .ok_or(
            ParseQueryError::new(
                ParseErrorReason::InvalidObjectKind(query.to_string())
            )
        )?;
    query.current = query_rest;
    query.offset += object_kind_str.len() + SPACE_LEN;
    Ok(object_kind)
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




fn parse_add_node (query: &mut Query) -> Result<AddNodeQO, ParseQueryError> {
    println!("parsing add node: {query}");
    let identifier = get_identifier(query)?;
    println!("identifier: {identifier}");
    let type_name = get_type_name(query)?;
    println!("typename: {type_name}");
    // println!("query after type name: {query}");
    let properties = parse_properties(query)?;
    Ok(AddNodeQO { 
        identifier: identifier.to_string(),
        type_name: type_name.to_string(),
        properties,
    })
}


fn parse_add_relationship (query: &mut Query) -> Result<AddRelationshipQO, ParseQueryError> {
    println!("parsing add relationship: {query}");
    let identifier = get_identifier(query)?;
    println!("identifier: {identifier}");
    let type_name = get_type_name(query)?;
    println!("typename: {type_name}");
    println!("query after type name: {query}");
    let (from, to) = get_nodes_for_relationship(query).unwrap();
    let properties = parse_properties(query)?;
    Ok(AddRelationshipQO {
        identifier: identifier.to_string(),
        type_name: type_name.to_string(),
        from,
        to, 
        properties,
    })
}



fn get_identifier (query: &mut Query) -> Result<String, ParseQueryError> {
    let identifier = query.to_next_space()
        .ok_or(ParseQueryError::new(ParseErrorReason::MissingIdentifier))?;
    if identifier.len() > MAX_IDENTIFIER_LEN {
        return Err(ParseQueryError::new(ParseErrorReason::TooLongIdentifier{got: identifier.len(), max_len: MAX_IDENTIFIER_LEN}));
    }
    Ok(identifier.to_string())
}


fn get_type_name (query: &mut Query) -> Result<String, ParseQueryError> {
    println!("query: {query}");
    let (expected_type, query_rest) = query.current.split_once(SPACE)
        .ok_or(ParseQueryError::new(ParseErrorReason::IdentifierMissingType))?;
    if expected_type != TYPE_STR {
        return Err(ParseQueryError::new(ParseErrorReason::IdentifierMissingType))
    }
    let (type_name, query_rest) = query_rest.split_once(SPACE)
        .ok_or(ParseQueryError::new(ParseErrorReason::IdentifierMissingType))?;
    query.current = query_rest;
    query.offset += TYPE_STR.len() + SPACE_LEN + type_name.len() + SPACE_LEN;
    Ok(type_name.to_string())
}

fn parse_properties (query: &mut Query) -> Result<HashMap<String, String>, ParseQueryError> {
    let q = query.current.to_string();
    println!("1query = {}", q);
    query.trim_left();
    if query.to_next_space()
        .ok_or(ParseKeyValueError::new(ParseKeyValueErrorReason::MissingPropertyStr))? 
    != PROPERTIES_STR {
        return Err(ParseKeyValueError::new(ParseKeyValueErrorReason::MissingPropertyStr).into());
    }
    let mut properties: HashMap<String, String> = HashMap::new();
    println!("Parsing properties for {query}");
    while query.current.trim().len() > 1 {
        println!("Start parsing with this {query}");
        parse_kv_pair(query, &mut properties)?;
    }
    Ok(properties)
}



/* 
* Parse key value pairs
* Example: name = 'Malik', age = 20, occupation = 'SWE', ...
* Expects the current query to start WITHOUT A COMMA!
* Whitespace is okay.
*/
fn parse_kv_pair (query: &mut Query, properties: &mut HashMap<String, String>) -> Result<(), ParseKeyValueError> {
    let key = get_key(query)?;
    query.trim_left();
    let value_str = get_value(query, &key)?;

    println!("key = {key}, value_str = '{value_str}'");
    properties.insert(key, value_str.to_string());
    query.trim_left_char(COMMA);

    println!("Remaining query: {:?}", query);
    Ok(())
}


fn get_key (query: &mut Query) -> Result<String, ParseKeyValueError> {
    let key = query.to_next_char(ASSIGNMENT)
        .ok_or(ParseKeyValueError::new(ParseKeyValueErrorReason::MissingAssignment))?;
    let key = key.trim();
    Ok(key.to_string())
}


fn get_value(query: &mut Query, key: &str) -> Result<String, ParseKeyValueError> {
    assert!(!query.current.is_empty());
    match query.current.chars().next().unwrap(){
        DOUBLE_QUOTE => {
            println!("Got String value double quotes");
            query.trim_left_char(DOUBLE_QUOTE).unwrap(); // trim first DOUBLE_QUOTE
            Ok(
                query.to_next_char(DOUBLE_QUOTE)
                    .ok_or(ParseKeyValueError::new(ParseKeyValueErrorReason::UnclosedDoubleQuote))?
                    .to_string()
            )
        },
        SINGLE_QUOTE => {
            println!("Got String value single quotes");
            query.trim_left_char(SINGLE_QUOTE).unwrap(); // trim first SINGLE_QUOTE
            Ok(
                query.to_next_char(SINGLE_QUOTE)
                    .ok_or(ParseKeyValueError::new(ParseKeyValueErrorReason::UnclosedSingleQuote))?
                    .to_string()
            )
        },
        _ => {
            println!("Value other than string");
            println!("query: {query}");
            if let Some(value) = query.to_next_char(COMMA){
                Ok(value.to_string())
            } 
            else if query.current.find(ASSIGNMENT).is_none(){
                Ok(query.to_end().to_string())
            } 
            else {
                return Err(ParseKeyValueError::new(
                    ParseKeyValueErrorReason::MissingValue { for_key: key.to_string() }
                ))
            }
        },
    }
}


fn get_nodes_for_relationship(query: &mut Query) -> Result<(u32, u32), String> {
    todo!("parse get_nodes_for_relationship");
}
