use super::super::{objects::*, query::Query, utils::*};
use super::super::errors::ParseQueryError;
use super::super::objects::RemoveQO;




pub fn parse_remove (query: &mut Query) -> Result<RemoveQO, ParseQueryError> {
    println!("Parsing remove: {query}");
    let object_kind = get_object_kind(query)?;
    let remove_query_object = {
        match object_kind {
            ObjectKind::Node => RemoveQO::Node(parse_remove_node(query)?),
            ObjectKind::Relationship => RemoveQO::Relationship(parse_remove_relationship(query)?),
            _ => todo!("Parse remove of other objects")
        }
    };
    println!("object kind is {:?}", object_kind);
    Ok(remove_query_object)
}



fn parse_remove_node (query: &mut Query) -> Result<RemoveNodeQO, ParseQueryError> {
    todo!("parse remove node");
}



fn parse_remove_relationship (query: &mut Query) -> Result<RemoveRelationshipQO, ParseQueryError> {
    todo!("parse remove relationship");
}

