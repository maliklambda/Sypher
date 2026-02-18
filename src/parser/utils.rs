use std::collections::HashMap;

use crate::constants::keywords::*;
use crate::constants::limits::MAX_IDENTIFIER_LEN;
use crate::constants::special_chars::*;
use crate::parser::errors::*;
use crate::parser::objects::*;
use crate::parser::operations::ops::Operation;
use crate::parser::query::Query;

pub fn get_identifier(query: &mut Query) -> Result<String, ParseQueryError> {
    let identifier = query
        .to_next_space()
        .ok_or(ParseQueryError::new(ParseErrorReason::MissingIdentifier))?;
    if identifier.len() > MAX_IDENTIFIER_LEN {
        return Err(ParseQueryError::new(ParseErrorReason::TooLongIdentifier {
            got: identifier.len(),
            max_len: MAX_IDENTIFIER_LEN,
        }));
    }
    Ok(identifier.to_string())
}

pub fn get_type_name(query: &mut Query) -> Result<String, ParseQueryError> {
    println!("query: {query}");
    let expected_type = query.to_next_space().ok_or(ParseQueryError::new(
        ParseErrorReason::IdentifierMissingType,
    ))?;
    if expected_type != TYPE_STR {
        return Err(ParseQueryError::new(
            ParseErrorReason::IdentifierMissingType,
        ));
    }
    let type_name =
        query
            .to_next_space()
            .ok_or(ParseQueryError::new(ParseErrorReason::MissingValue {
                for_keyword: TYPE_STR.to_string(),
            }))?;
    Ok(type_name.to_string())
}

pub fn parse_properties(query: &mut Query) -> Result<HashMap<String, String>, ParseQueryError> {
    let q = query.current.to_string();
    println!("1query = {}", q);
    query.trim_left();
    if query.to_next_space().ok_or(ParseKeyValueError::new(
        ParseKeyValueErrorReason::MissingPropertyStr,
    ))? != PROPERTIES_STR
    {
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
pub fn parse_kv_pair(
    query: &mut Query,
    properties: &mut HashMap<String, String>,
) -> Result<(), ParseKeyValueError> {
    let key = get_key(query)?;
    query.trim_left();
    let value_str = get_value(query, &key)?;

    println!("key = {key}, value_str = '{value_str}'");
    properties.insert(key, value_str.to_string());
    query.trim_left_char(COMMA);

    println!("Remaining query: {:?}", query);
    Ok(())
}

pub fn get_key(query: &mut Query) -> Result<String, ParseKeyValueError> {
    let key = query
        .to_next_char(ASSIGNMENT)
        .ok_or(ParseKeyValueError::new(
            ParseKeyValueErrorReason::MissingAssignment,
        ))?;
    let key = key.trim();
    Ok(key.to_string())
}

pub fn get_value(query: &mut Query, key: &str) -> Result<String, ParseKeyValueError> {
    assert!(!query.current.is_empty());
    match query.current.chars().next().unwrap() {
        DOUBLE_QUOTE => {
            println!("Got String value double quotes");
            query.trim_left_char(DOUBLE_QUOTE).unwrap(); // trim first DOUBLE_QUOTE
            Ok(query
                .to_next_char(DOUBLE_QUOTE)
                .ok_or(ParseKeyValueError::new(
                    ParseKeyValueErrorReason::UnclosedDoubleQuote,
                ))?
                .to_string())
        }
        SINGLE_QUOTE => {
            println!("Got String value single quotes");
            query.trim_left_char(SINGLE_QUOTE).unwrap(); // trim first SINGLE_QUOTE
            Ok(query
                .to_next_char(SINGLE_QUOTE)
                .ok_or(ParseKeyValueError::new(
                    ParseKeyValueErrorReason::UnclosedSingleQuote,
                ))?
                .to_string())
        }
        _ => {
            println!("Value other than string");
            println!("query: {query}");
            if let Some(value) = query.to_next_char(COMMA) {
                Ok(value.to_string())
            } else if query.current.find(ASSIGNMENT).is_none() {
                Ok(query.to_end().to_string())
            } else {
                return Err(ParseKeyValueError::new(
                    ParseKeyValueErrorReason::MissingValue {
                        for_key: key.to_string(),
                    },
                ));
            }
        }
    }
}

pub fn get_nodes_for_relationship(query: &mut Query) -> Result<NodeTuple, ParseQueryError> {
    assert_eq!(
        FROM_STR,
        query
            .to_next_space()
            .ok_or(ParseQueryError::new(ParseErrorReason::MissingKeyword {
                expected: FROM_STR.to_string()
            }))?
    );
    let from = query
        .to_next_space()
        .ok_or(ParseQueryError::new(ParseErrorReason::MissingValue {
            for_keyword: FROM_STR.to_string(),
        }))?
        .parse()
        .map_err(|err| ParseQueryError::new(ParseErrorReason::ParseID(err)))?;
    println!("current query: {query}");
    assert_eq!(
        TO_STR,
        query
            .to_next_space()
            .ok_or(ParseQueryError::new(ParseErrorReason::MissingKeyword {
                expected: TO_STR.to_string()
            }))?
    );
    let to = query
        .to_next_space()
        .ok_or(ParseQueryError::new(ParseErrorReason::MissingValue {
            for_keyword: TO_STR.to_string(),
        }))?
        .parse()
        .map_err(|err| ParseQueryError::new(ParseErrorReason::ParseID(err)))?;
    Ok(NodeTuple::new(to, from))
}

pub fn get_object_kind(query: &mut Query) -> Result<ObjectKind, ParseQueryError> {
    let (object_kind_str, query_rest) = query
        .current
        .split_once(SPACE)
        .ok_or(ParseQueryError::new(ParseErrorReason::InvalidObjectKind))?;
    let object_kind = ObjectKind::from_str(object_kind_str)
        .ok_or(ParseQueryError::new(ParseErrorReason::InvalidObjectKind))?;
    query.current = query_rest;
    query.offset += object_kind_str.len() + SPACE_LEN;
    Ok(object_kind)
}

pub fn get_operation(query: &mut Query) -> Result<Operation, ParseQueryError> {
    let keyword =
        query
            .to_next_space()
            .ok_or(ParseQueryError::new(ParseErrorReason::InvalidKeyword(
                // query.current.to_string()
                "keyword".to_string(),
            )))?;
    let operation = Operation::from_str(keyword).ok_or(ParseQueryError {
        reason: ParseErrorReason::InvalidKeyword(keyword.to_string()),
    })?;
    Ok(operation)
}
