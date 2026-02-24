use crate::{
    constants::{
        keywords::supqueries::SUBQ_PATTERN,
        special_chars::subqueries::{SUBQ_END, SUBQ_START},
    },
    parser::query::Query,
};

pub mod build_subqueries;
pub mod tree;

pub fn remove_subquery_str(mut query: Query) -> Query {
    query.trim_left_str(SUBQ_PATTERN);
    query.trim_left_char(SUBQ_START);
    if let Some(s) = query.current.strip_suffix(SUBQ_END) {
        query.current = s;
    }
    query
}
