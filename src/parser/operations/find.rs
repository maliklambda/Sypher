use crate::parser::{errors::ParseQueryError, objects::FindQO, query::Query};

pub fn parse_find(query: &mut Query) -> Result<FindQO, ParseQueryError> {
    todo!("parse find");
}
