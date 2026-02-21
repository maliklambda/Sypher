use crate::parser::{
    errors::ParseQueryError,
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

pub fn parse_add(query: &mut Query) -> Result<AddQO, ParseQueryError> {
    println!("Parsing add: {query}");
    let add_query_object = {
        match get_object_kind(query)? {
            ObjectKind::Node => AddQO::Node(parse_add_node(query)?),
            ObjectKind::Relationship => AddQO::Relationship(parse_add_relationship(query)?),
        }
    };
    Ok(add_query_object)
}

fn parse_add_node(query: &mut Query) -> Result<AddNodeQO, ParseQueryError> {
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

fn parse_add_relationship(query: &mut Query) -> Result<AddRelationshipQO, ParseQueryError> {
    println!("parsing add relationship: {query}");
    let identifier = get_identifier(query)?;
    println!("identifier: {identifier}");
    let type_name = get_type_name(query)?;
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
