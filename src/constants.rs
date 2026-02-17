
pub mod keywords {
    pub const NODE_STR: &str = "NODE";
    pub const RELATIONSHIP_STR: &str = "RELATIONSHIP";
    pub const TYPE_STR: &str = "TYPE";
    pub const PROPERTIES_STR: &str = "PROPERTIES";
}

pub mod command_kws {
    pub const ADD_STR: &str = "ADD";
    pub const REMOVE_STR: &str = "REMOVE";
    pub const GET_STR: &str = "GET";
    pub const FIND_STR: &str = "FIND";
    pub const UPDATE_STR: &str = "UPDATE";
}

pub mod special_chars {
    pub const SPACE: char = ' ';
    pub const SEMICOLON: char = ';';
    pub const COMMA: char = ',';
    pub const DOT: char = '.';
    pub const ASSIGNMENT: char = '=';
    pub const DOUBLE_QUOTE: char = '"';
    pub const SINGLE_QUOTE: char = '\'';
}

pub mod limits {
    pub const MAX_IDENTIFIER_LEN: usize = 128;
}



