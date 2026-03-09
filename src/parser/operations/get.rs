use crate::{
    constants::keywords::supqueries::SUBQ_PATTERN, parser::{
        errors::{ParseErrorReason, ParseQueryError},
        objects::{ObjectKind, get::GetQO},
        query::Query,
        utils::get_object_kind,
    }, types::{NodeID, RelationshipID}
};

pub fn parse_get(query: &mut Query) -> Result<GetQO, ParseQueryError> {
    println!("Parsing get: {query}");
    let object_kind = get_object_kind(query)?;
    let get_query_object = {
        match object_kind {
            ObjectKind::Node => GetQO::Node(parse_get_node(query)?),
            ObjectKind::Relationship => GetQO::Relationship(parse_get_relationship(query)?),
        }
    };
    Ok(get_query_object)
}

fn parse_get_node(query: &mut Query) -> Result<NodeID, ParseQueryError> {
    parse_get_id(query)
}

fn parse_get_relationship(query: &mut Query) -> Result<RelationshipID, ParseQueryError> {
    parse_get_id(query)
}

fn parse_get_id(query: &mut Query) -> Result<RelationshipID, ParseQueryError> {
    let end = query.to_end();
    if !end.trim().starts_with(SUBQ_PATTERN) {
        let id = end
            .parse()
            .map_err(|err| ParseQueryError::new(ParseErrorReason::ParseID(err)))?;
        Ok(id)
    } else {
        todo!("Subquery as value");
    }
}
