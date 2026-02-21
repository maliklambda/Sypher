use std::collections::HashMap;

use crate::{
    constants::{
        keywords::{condition::WHERE_STR, parse_match::RETURN_STR},
        special_chars::{DOT, RETURN_VALUE_SEPARATOR, SPACE, parse_match::*},
    },
    parser::{
        errors::{ParseErrorReason, ParseMatchError, ParseMatchErrorReason, ParseQueryError},
        objects::parse_match::{
            IdentifierData, MatchObject, MatchQO, RelationshipDirection, ReturnValue,
        },
        query::Query,
    },
    types::IdentifierName,
};

pub fn parse_match(query: &mut Query) -> Result<MatchQO, ParseQueryError> {
    println!("query: {query}");
    let pattern = query.to_next_str(WHERE_STR).ok_or(ParseQueryError::new(
        ParseErrorReason::MissingKeyword {
            expected: WHERE_STR.to_string(),
        },
    ))?;
    println!("pattern: {pattern}");
    // parse pattern
    let match_objects = parse_pattern(pattern)?;
    println!("Parsed match objects: {:?}", match_objects);

    let conditions_str = query.to_next_str(RETURN_STR).ok_or(ParseQueryError::new(
        ParseErrorReason::MissingKeyword {
            expected: RETURN_STR.to_string(),
        },
    ))?;
    println!("conditions: {conditions_str}");
    // parse conditions
    let filters = HashMap::new();

    let return_values_str = query.to_end();
    println!("return values: {return_values_str}");
    // parse return values
    let return_values = parse_return_values(return_values_str)?;

    Ok(MatchQO {
        match_objects,
        filters,
        return_values,
    })
}

fn parse_pattern(
    pattern_str: &str,
) -> Result<HashMap<IdentifierName, MatchObject>, ParseMatchError> {
    let pattern_str = pattern_str.replace(SPACE, "");
    let mut pattern = Query::from_str(&pattern_str);
    let mut match_objects: HashMap<IdentifierName, MatchObject> = HashMap::new();
    let leftmost_node = parse_node(&mut pattern)?;
    let mut prev_node_name = leftmost_node.name.clone();
    println!("Leftmost node is: {:?}", leftmost_node);
    println!("Pattern after parsing lmnode: {pattern}");
    match_objects.insert(leftmost_node.name.clone(), leftmost_node);
    while !pattern.current.is_empty() {
        let mut new_rel = parse_relationship(&mut pattern)?;
        println!("new rel is: {:?}", new_rel);
        let mut new_node = parse_node(&mut pattern)?;
        println!("new node is: {:?}", new_node);

        match new_rel.data {
            IdentifierData::Relationship {
                ref direction,
                ref mut start,
                ref mut end,
            } => {
                if *direction == RelationshipDirection::Outgoing {
                    // adjust prev node: set prev_node.rel to new_rel -> consider in- vs. outgoing
                    let prev_node = match_objects.get_mut(&prev_node_name).unwrap();
                    prev_node.data = IdentifierData::Node {
                        outgoing: Some(new_rel.name.clone()),
                        ingoing: None,
                    };

                    // adjust new rel: set (start, end) to (prev_node, new_node) -> consider in- vs. outgoing
                    *start = Some(prev_node_name.clone());
                    *end = Some(new_node.name.clone());

                    // adjust new node: set new_node.rel to new_rel
                    new_node.data = IdentifierData::Node {
                        outgoing: None,
                        ingoing: Some(new_rel.name.clone()),
                    }
                } else {
                    // adjust prev node: set prev_node.rel to new_rel -> consider in- vs. outgoing
                    let prev_node = match_objects.get_mut(&prev_node_name).unwrap();
                    prev_node.data = IdentifierData::Node {
                        outgoing: None,
                        ingoing: Some(new_rel.name.clone()),
                    };

                    // adjust new rel: set (start, end) to (prev_node, new_node) -> consider in- vs. outgoing
                    *start = Some(new_node.name.clone());
                    *end = Some(prev_node_name.clone());

                    // adjust new node: set new_node.rel to new_rel
                    new_node.data = IdentifierData::Node {
                        outgoing: Some(new_rel.name.clone()),
                        ingoing: None,
                    }
                }
            }
            _ => panic!(
                "Newly parsed relationship should have the type 'Relationship' and not 'Node'"
            ),
        }
        match_objects.insert(new_rel.name.clone(), new_rel);
        prev_node_name = new_node.name.clone();
        match_objects.insert(new_node.name.clone(), new_node);
    }
    println!("MATCH OBJECTS::::::{:?}", match_objects);
    Ok(match_objects)
}

fn parse_node(query: &mut Query) -> Result<MatchObject, ParseMatchError> {
    query
        .trim_left_char(MATCH_NODE_START)
        .ok_or(ParseMatchError {
            reason: ParseMatchErrorReason::StartWithoutNode,
            pattern: query.current.to_string(),
        })?;

    let (id_name, type_name) = parse_name_type(query, MATCH_NODE_END.to_string().as_str())?;

    Ok(MatchObject {
        name: id_name.to_string(),
        object_type: type_name.to_string(),
        data: IdentifierData::Node {
            outgoing: None,
            ingoing: None,
        },
    })
}

fn parse_relationship(query: &mut Query) -> Result<MatchObject, ParseMatchError> {
    // may start with "-" (in case of "-[]->") or "<-" (in case of "<-[]-")
    let cur = query.current.to_string();
    match cur {
        s if s.starts_with(MATCH_REL_DIRECTION_LEFT) => parse_ingoing_rel(query),
        s if s.starts_with(MATCH_REL_TAIL) => parse_outgoing_rel(query),
        _ => Err(ParseMatchError {
            reason: ParseMatchErrorReason::StartWithoutNode,
            pattern: query.current.to_string(),
        }),
    }
}

/*
* Matching query looks something like this: "(n1:node)<-[r:TYPE]-(n2:node)"
*/
fn parse_ingoing_rel(pattern: &mut Query) -> Result<MatchObject, ParseMatchError> {
    println!("parsing ingoing rel");
    pattern.trim_left_str(MATCH_REL_DIRECTION_LEFT).unwrap();
    let (id_name, type_name) = parse_name_type(pattern, MATCH_REL_TAIL.to_string().as_str())?;
    let id_name = id_name.replace(MATCH_REL_START, "");
    let type_name = type_name.replace(MATCH_REL_END, "");
    println!("Pattern after parsing ingoing rels: {pattern}");
    Ok(MatchObject {
        name: id_name,
        object_type: type_name,
        data: IdentifierData::Relationship {
            start: None,
            end: None,
            direction: RelationshipDirection::Ingoing,
        },
    })
}

/*
* Matching query looks something like this: "(n1:node)-[r:TYPE]->(n2:node)"
*/
fn parse_outgoing_rel(pattern: &mut Query) -> Result<MatchObject, ParseMatchError> {
    println!("parsing ingoing rel");
    pattern.trim_left_char(MATCH_REL_TAIL).unwrap();
    let (id_name, type_name) = parse_name_type(pattern, MATCH_REL_DIRECTION_RIGHT)?;
    let id_name = id_name.replace(MATCH_REL_START, "");
    let type_name = type_name.replace(MATCH_REL_END, "");
    println!("Pattern after parsing outgoing rels: {pattern}");
    Ok(MatchObject {
        name: id_name,
        object_type: type_name,
        data: IdentifierData::Relationship {
            start: None,
            end: None,
            direction: RelationshipDirection::Outgoing,
        },
    })
}

fn parse_name_type<'a>(
    query: &'a mut Query,
    end_str: &str,
) -> Result<(&'a str, &'a str), ParseMatchError> {
    let cur = query.current.to_string();
    let [id_name, type_name] = query
        .to_next_str(end_str)
        .ok_or(ParseMatchError::new(
            ParseMatchErrorReason::ParseNameType,
            cur.clone(),
        ))?
        .split(MATCH_TYPE_SEPARATOR)
        .collect::<Vec<_>>()
        .as_slice()
        .try_into()
        .map_err(|_| ParseMatchError::new(ParseMatchErrorReason::ParseNameType, cur))?;
    Ok((id_name, type_name))
}

fn alphabetic_chars_only(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>()
}

fn parse_return_values(s: &str) -> Result<Vec<ReturnValue>, ParseMatchError> {
    let s = s.replace(SPACE, "");
    let mut values: Vec<ReturnValue> = vec![];
    for val in s.split(RETURN_VALUE_SEPARATOR) {
        println!("value: {:?}", val);
        let parts = val.split(DOT).collect::<Vec<&str>>();
        if parts.is_empty() {
            return Err(ParseMatchError {
                reason: ParseMatchErrorReason::ParseReturnValues,
                pattern: val.to_string(),
            });
        }
        let id_name = parts[0].to_string();
        let prop_name: Option<String> = {
            if parts.len() == 2 {
                Some(parts[1].to_string())
            } else {
                None
            }
        };
        values.push(ReturnValue::new(id_name, prop_name))
    }
    Ok(values)
}
