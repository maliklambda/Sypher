use crate::{
    constants::{
        command_kws::{ADD_STR, UPDATE_STR},
        keywords::{NODE_STR, update::VALUE_STR},
        special_chars::{ASSIGNMENT, UPDATE_OPERATION_SEPARATOR},
    },
    parser::{
        errors::{ParseErrorReason, ParseQueryError},
        objects::{
            ObjectKind,
            update::{
                UpdateNodeQO, UpdateOperation, UpdateOperationKind, UpdateQO, UpdateRelationshipQO,
            },
        },
        query::Query,
        utils::{get_object_kind, kv_get_value},
    },
    types::NodeID,
};

pub fn parse_update(query: &mut Query) -> Result<UpdateQO, ParseQueryError> {
    println!("{query}");
    let update_query_object = {
        match get_object_kind(query)? {
            ObjectKind::Node => UpdateQO::Node(parse_update_node(query)?),
            ObjectKind::Relationship => UpdateQO::Relationship(parse_update_relationship(query)?),
        }
    };
    Ok(update_query_object)
}

fn parse_update_node(query: &mut Query) -> Result<UpdateNodeQO, ParseQueryError> {
    let id: NodeID = query
        .to_next_space()
        .ok_or(ParseQueryError::new(
            crate::parser::errors::ParseErrorReason::MissingValue {
                for_keyword: format!("{UPDATE_STR} {NODE_STR}"),
            },
        ))?
        .parse()
        .map_err(|err| ParseQueryError::new(ParseErrorReason::ParseID(err)))?;

    let ops = parse_update_operations(query)?;

    Ok(UpdateNodeQO {
        id,
        operations: ops,
    })
}

fn parse_update_relationship(_query: &mut Query) -> Result<UpdateRelationshipQO, ParseQueryError> {
    todo!("parse update relationship");
}

fn parse_update_operations(query: &mut Query) -> Result<Vec<UpdateOperation>, ParseQueryError> {
    let mut update_operations: Vec<UpdateOperation> = vec![];
    while query.current.len() >= 2 {
        update_operations.push(parse_single_update_operation(query)?);
        query.trim_left_char(UPDATE_OPERATION_SEPARATOR);
    }
    Ok(update_operations)
}

fn parse_single_update_operation(query: &mut Query) -> Result<UpdateOperation, ParseQueryError> {
    let update_operation: UpdateOperation = {
        match get_update_operation_kind(query).ok_or(ParseQueryError::new(
            ParseErrorReason::InvalidUpdateOperation,
        ))? {
            UpdateOperationKind::ADD => parse_update_operation_add(query)?,
            UpdateOperationKind::SET => parse_update_operation_set(query)?,
            UpdateOperationKind::REMOVE => parse_update_operation_remove(query),
        }
    };
    Ok(update_operation)
}

fn get_update_operation_kind(query: &mut Query) -> Option<UpdateOperationKind> {
    query.trim_left();
    UpdateOperationKind::from_str(query.to_next_space()?)
}

fn parse_update_operation_add(query: &mut Query) -> Result<UpdateOperation, ParseQueryError> {
    println!("{query}");
    let property = query
        .to_next_space()
        .ok_or(ParseQueryError::new(ParseErrorReason::MissingValue {
            for_keyword: ADD_STR.to_string(),
        }))?
        .to_string();
    if query
        .to_next_space()
        .ok_or(ParseQueryError::new(ParseErrorReason::MissingValue {
            for_keyword: ADD_STR.to_string(),
        }))?
        != VALUE_STR
    {
        return Err(ParseQueryError::new(ParseErrorReason::MissingKeyword {
            expected: VALUE_STR.to_string(),
        }));
    }
    let value = if let Some(re) = query.to_next_char(UPDATE_OPERATION_SEPARATOR) {
        re
    } else {
        query.to_end()
    }
    .to_string();
    Ok(UpdateOperation::Set { property, value })
}

fn parse_update_operation_set(query: &mut Query) -> Result<UpdateOperation, ParseQueryError> {
    let property = query
        .to_next_char(ASSIGNMENT)
        .ok_or(ParseQueryError::new(ParseErrorReason::MissingAssignment))?
        .to_string();
    query.trim_left();
    let value = kv_get_value(query, &property)
        .map_err(|err| ParseQueryError::new(ParseErrorReason::ParseKeyValuePairs(err)))?;
    Ok(UpdateOperation::Set { property, value })
}

fn parse_update_operation_remove(query: &mut Query) -> UpdateOperation {
    let remove_property = if let Some(re) = query.to_next_char(UPDATE_OPERATION_SEPARATOR) {
        re
    } else {
        query.to_end()
    };
    UpdateOperation::Remove {
        property: remove_property.to_string(),
    }
}
