use crate::parser::objects::ObjectKind;
use crate::parser::{
    errors::ParseQueryError, objects::FindQO, query::Query, utils::get_object_kind,
};

pub fn parse_find(query: &mut Query) -> Result<FindQO, ParseQueryError> {
    println!("{query}");
    let find_query_object = match get_object_kind(query)? {
        ObjectKind::Node => FindQO::Node(),
        ObjectKind::Relationship => FindQO::Relationship(),
    };
    Ok(find_query_object)
}
