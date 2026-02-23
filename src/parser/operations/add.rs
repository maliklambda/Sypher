use crate::parser::{
    errors::{ParseErrorReason, ParseQueryError, ParseSubqueryErrorReason},
    objects::{
        NodeTuple, ObjectKind,
        add::{AddNodeQO, AddQO, AddRelationshipQO},
    },
    query::Query,
    utils::{
        get_identifier, get_nodes_for_relationship, get_object_kind, get_type_name,
        parse_properties,
    },
};

pub fn parse_add<'a>(query: &'a mut Query) -> Result<AddQO, ParseQueryError<'a>> {
    println!("Parsing add: {query}");
    let add_query_object = {
        match get_object_kind(query)? {
            ObjectKind::Node => AddQO::Node(parse_add_node(query)?),
            ObjectKind::Relationship => AddQO::Relationship(parse_add_relationship(query)?),
        }
    };
    Ok(add_query_object)
}

fn parse_add_node<'a>(query: &'a mut Query) -> Result<AddNodeQO, ParseQueryError<'a>> {
    println!("parsing add node: {query}");
    let identifier = get_identifier(query).map_err(|err|
        match err {
            ParseErrorReason::MissingIdentifier => ParseQueryError::new(ParseErrorReason::MissingIdentifier),
            ParseErrorReason::TooLongIdentifier { got, max_len } => ParseQueryError::new(ParseErrorReason::TooLongIdentifier { got, max_len }),
            _ => todo!("Make get_identifier error pretty"),
        }
    )?;
    println!("identifier: {identifier}");

    let type_name = get_type_name(query).map_err(|err|
        match err {
            ParseErrorReason::IdentifierMissingType => ParseQueryError::new(ParseErrorReason::MissingIdentifier),
            ParseErrorReason::MissingValue { for_keyword } => ParseQueryError::new(ParseErrorReason::MissingValue { for_keyword }),
            _ => todo!("Make get_type_name error pretty"),
        }
    )?;
    println!("typename: {type_name}");

    let properties = parse_properties(query)?;
    Ok(AddNodeQO {
        identifier,
        type_name,
        properties,
    })
}

fn parse_add_relationship<'a>(query: &'a mut Query) -> Result<AddRelationshipQO, ParseQueryError<'a>> {
    println!("parsing add relationship: {query}");
    let identifier = get_identifier(query).map_err(|err|
        match err {
            ParseErrorReason::MissingIdentifier => ParseQueryError::new(ParseErrorReason::MissingIdentifier),
            ParseErrorReason::TooLongIdentifier { got, max_len } => ParseQueryError::new(ParseErrorReason::TooLongIdentifier { got, max_len }),
            _ => todo!("Make get_identifier error pretty"),
        }
    )?;
    println!("identifier: {identifier}");
    let type_name = get_type_name(query).map_err(|err|
        match err {
            ParseErrorReason::IdentifierMissingType => ParseQueryError::new(ParseErrorReason::MissingIdentifier),
            ParseErrorReason::MissingValue { for_keyword } => ParseQueryError::new(ParseErrorReason::MissingValue { for_keyword }),
            _ => todo!("Make get_type_name error pretty"),
        }
    )?;
    println!("typename: {type_name}");
    println!("query after type name: {query}");
    let NodeTuple { from, to } = get_nodes_for_relationship(query)?;
    let properties = parse_properties(query)?;
    Ok(AddRelationshipQO {
        identifier: identifier.to_string(),
        type_name: type_name.to_string(),
        from,
        to,
        properties,
    })
}


