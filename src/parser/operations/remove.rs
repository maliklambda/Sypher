use crate::{
    constants::{command_kws::REMOVE_STR, keywords::remove::MODE_STR},
    parser::{
        errors::{ParseErrorReason, ParseQueryError},
        objects::{
            ObjectKind,
            remove::{RemoveMode, RemoveNodeQO, RemoveQO, RemoveRelationshipQO},
        },
        query::Query,
        utils::get_object_kind,
    },
};

pub fn parse_remove(query: &mut Query) -> Result<RemoveQO, ParseQueryError> {
    println!("Parsing remove: {query}");
    let remove_query_object = {
        match get_object_kind(query)? {
            ObjectKind::Node => RemoveQO::Node(parse_remove_node(query)?),
            ObjectKind::Relationship => RemoveQO::Relationship(parse_remove_relationship(query)?),
            // _ => todo!("Parse remove of other objects"),
        }
    };
    Ok(remove_query_object)
}

fn parse_remove_node(query: &mut Query) -> Result<RemoveNodeQO, ParseQueryError> {
    println!("parsing remove node: {query}");
    let remove_id = query
        .to_next_space()
        .ok_or(ParseQueryError::new(
            crate::parser::errors::ParseErrorReason::MissingValue {
                for_keyword: REMOVE_STR.to_string(),
            },
        ))?
        .parse()
        .map_err(|err| ParseQueryError::new(ParseErrorReason::ParseID(err)))?;

    println!("we got id: {remove_id}");
    println!("{query}");
    let remove_mode = get_remove_mode(query)?;

    Ok(RemoveNodeQO {
        id: remove_id,
        mode: remove_mode,
    })
}

fn parse_remove_relationship(query: &mut Query) -> Result<RemoveRelationshipQO, ParseQueryError> {
    println!("parsing remove node: {query}");
    let remove_id = query
        .to_end()
        .parse()
        .map_err(|err| ParseQueryError::new(ParseErrorReason::ParseID(err)))?;

    Ok(RemoveRelationshipQO { id: remove_id })
}

fn get_remove_mode(query: &mut Query) -> Result<RemoveMode, ParseQueryError> {
    query.trim_left();
    if query
        .to_next_space()
        .ok_or(ParseQueryError::new(ParseErrorReason::MissingKeyword {
            expected: MODE_STR.to_string(),
        }))?
        != MODE_STR
    {
        return Err(ParseQueryError::new(ParseErrorReason::MissingKeyword {
            expected: MODE_STR.to_string(),
        }));
    }
    println!("now = {query}");
    let remove_mode_str = query.to_end().trim_end();
    println!("removemode = {remove_mode_str}");
    let mode = RemoveMode::from_str(remove_mode_str)
        .ok_or(ParseQueryError::new(ParseErrorReason::UnknownRemoveMode))?;
    Ok(mode)
}
