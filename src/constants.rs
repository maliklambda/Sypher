pub mod keywords {
    pub const NODE_STR: &str = "NODE";
    pub const RELATIONSHIP_STR: &str = "RELATIONSHIP";
    pub const TYPE_STR: &str = "TYPE";
    pub const PROPERTIES_STR: &str = "PROPERTIES";

    pub mod add {
        pub const FROM_STR: &str = "FROM";
        pub const TO_STR: &str = "TO";
    }

    pub mod remove {
        pub const MODE_STR: &str = "MODE";
        pub const CASCADE_STR: &str = "CASCADE";
        pub const SAFE_STR: &str = "SAFE";
    }

    pub mod update {
        pub const ADD_STR: &str = "ADD";
        pub const SET_STR: &str = "SET";
        pub const REMOVE_STR: &str = "REMOVE";
        pub const VALUE_STR: &str = "VALUE";
    }

    pub mod condition {
        pub const WHERE_STR: &str = "WHERE";
        pub const AND_STR: &str = "AND";
        pub const OR_STR: &str = "OR";
        pub const CONDITION_GROUP_START: char = '(';
        pub const CONDITION_GROUP_END: char = ')';
    }

    pub mod parse_match {
        pub const RETURN_STR: &str = "RETURN";
    }

    pub mod supqueries {
        pub const SUBQ_PATTERN: &str = "SUBQ";
    }
}

pub mod command_kws {
    pub const ADD_STR: &str = "ADD";
    pub const REMOVE_STR: &str = "REMOVE";
    pub const GET_STR: &str = "GET";
    pub const FIND_STR: &str = "FIND";
    pub const MATCH_STR: &str = "MATCH";
    pub const UPDATE_STR: &str = "UPDATE";
}

pub mod special_chars {
    pub const SPACE: char = ' ';
    pub const SPACE_LEN: usize = 1;
    pub const QUERY_SEPARATOR: char = ';';
    pub const KV_PAIR_SEPARATOR: char = ',';
    pub const UPDATE_OPERATION_SEPARATOR: char = ',';
    pub const DOT: char = '.';
    pub const ASSIGNMENT: char = '=';
    pub const DOUBLE_QUOTE: char = '"';
    pub const SINGLE_QUOTE: char = '\'';

    pub mod subqueries {
        pub const SUBQ_START: char = '{';
        pub const SUBQ_END: char = '}';
    }

    // MATCH
    pub mod parse_match {
        pub const MATCH_NODE_START: char = '(';
        pub const MATCH_NODE_END: char = ')';
        pub const MATCH_REL_START: char = '[';
        pub const MATCH_REL_END: char = ']';
        pub const MATCH_REL_DIRECTION_LEFT: &str = "<-";
        pub const MATCH_REL_DIRECTION_RIGHT: &str = "->";
        pub const MATCH_REL_TAIL: char = '-';
        pub const MATCH_TYPE_SEPARATOR: char = ':';
    }
}

pub mod limits {
    pub const MAX_IDENTIFIER_LEN: usize = 128;
}
