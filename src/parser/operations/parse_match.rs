use crate::parser::{errors::ParseQueryError, objects::MatchQO, query::Query};

pub fn parse_match(query: &mut Query) -> Result<MatchQO, ParseQueryError> {
    todo!("Parse match: {query}");
}
