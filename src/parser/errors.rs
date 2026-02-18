use std::num::ParseIntError;

#[derive(Debug)]
pub struct ParseQueryError {
    pub reason: ParseErrorReason,
}

impl ParseQueryError {
    pub fn new(reason: ParseErrorReason) -> ParseQueryError {
        ParseQueryError { reason }
    }

    pub fn default() -> ParseQueryError {
        ParseQueryError {
            reason: ParseErrorReason::Default,
        }
    }
}

impl From<ParseKeyValueError> for ParseQueryError {
    fn from(value: ParseKeyValueError) -> Self {
        ParseQueryError::new(ParseErrorReason::ParseKeyValuePairs(value))
    }
}

impl std::error::Error for ParseQueryError {}
impl std::fmt::Display for ParseQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing query failed because of {}", self.reason,)
    }
}

pub type ErrorSection = (usize, usize);

#[derive(Debug, PartialEq)]
pub enum ParseErrorReason {
    // Invalid
    InvalidKeyword(String),
    InvalidObjectKind,
    InvalidUpdateOperation,

    // Missing
    MissingIdentifier,
    MissingKeyword { expected: String },
    MissingValue { for_keyword: String },
    MissingTypeName,
    MissingAssignment,
    IdentifierMissingType,

    // Parse
    ParseID(ParseIntError),
    ParseKeyValuePairs(ParseKeyValueError),

    // Other
    UnknownRemoveMode,
    TooLongIdentifier { got: usize, max_len: usize },
    Other,
    Default,
}

impl std::fmt::Display for ParseErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = {
            match self {
                // Invalid
                ParseErrorReason::InvalidKeyword(invalid_kw) => {
                    &format!("Invalid key word: {invalid_kw}")
                }
                ParseErrorReason::InvalidObjectKind => {
                    "Invalid object kind. Valid object kinds are 'Node' and 'Relationship'"
                }
                ParseErrorReason::InvalidUpdateOperation => {
                    "Invalid Update operation. Valid update operations are 'ADD', 'SET' and 'REMOVE'."
                }

                // Missing
                ParseErrorReason::MissingIdentifier => "Missing identifier",
                ParseErrorReason::MissingKeyword { expected } => {
                    &format!("Missing required keyword '{expected}'")
                }
                ParseErrorReason::MissingValue { for_keyword } => {
                    &format!("Missing value for keyword '{for_keyword}'")
                }
                ParseErrorReason::MissingTypeName => "Identifier missing type name",
                ParseErrorReason::MissingAssignment => "Missing Assignment Operator ('=')",
                ParseErrorReason::IdentifierMissingType => "Identifier missing type",

                // Parse
                ParseErrorReason::ParseKeyValuePairs(kv_err) => {
                    &format!("Parse key value pairs failed: {kv_err}")
                }
                ParseErrorReason::ParseID(err) => &format!("Parsing node failed: {err}"),

                // Other
                ParseErrorReason::UnknownRemoveMode => {
                    "Unknown remove mode. Valid remove modes are 'CASCADE' and 'SAFE'"
                }
                ParseErrorReason::TooLongIdentifier { got, max_len } => {
                    &format!("Identifier is too long. Max length: {max_len}, got {got}")
                }
                ParseErrorReason::Other => "Other",
                ParseErrorReason::Default => "Default",
            }
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseKeyValueError {
    pub reason: ParseKeyValueErrorReason,
}

impl std::error::Error for ParseKeyValueError {}
impl std::fmt::Display for ParseKeyValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.reason {
            ParseKeyValueErrorReason::MissingSpace => write!(f, "Missing space"),
            ParseKeyValueErrorReason::MissingAssignment => {
                write!(f, "Missing assignment operator ('=')")
            }
            ParseKeyValueErrorReason::MissingPropertyStr => write!(f, "Missing 'PROPERTY' string"),
            ParseKeyValueErrorReason::UnclosedSingleQuote => {
                write!(f, "Single quote (') was not closed")
            }
            ParseKeyValueErrorReason::UnclosedDoubleQuote => {
                write!(f, "Double quote (\") was not closed")
            }
            ParseKeyValueErrorReason::MissingValue { for_key } => {
                write!(f, "Missing value for key {for_key}")
            }
            ParseKeyValueErrorReason::Default => write!(f, "Default KV-error"),
        }
    }
}

impl ParseKeyValueError {
    pub fn new(reason: ParseKeyValueErrorReason) -> ParseKeyValueError {
        ParseKeyValueError { reason }
    }

    pub fn default() -> ParseKeyValueError {
        ParseKeyValueError {
            reason: ParseKeyValueErrorReason::Default,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseKeyValueErrorReason {
    MissingSpace,
    MissingAssignment,
    MissingPropertyStr,
    MissingValue { for_key: String },
    UnclosedSingleQuote,
    UnclosedDoubleQuote,
    Default,
}
