use crate::constants::command_kws::*;

#[derive(Clone, Debug)]
pub enum Operation {
    Add,
    Remove,
    Get,
    Match,
    Update,
}

impl Operation {
    const STRINGS: &'static [(&'static str, Self)] = &[
        (ADD_STR, Operation::Add),
        (REMOVE_STR, Operation::Remove),
        (GET_STR, Operation::Get),
        (MATCH_STR, Operation::Match),
        (UPDATE_STR, Operation::Update),
    ];

    pub fn from_str(s: &str) -> Option<Operation> {
        let (_, operation) = Self::STRINGS.iter().find(|(value, _)| value == &s)?;
        Some(operation.clone())
    }
}
