use std::num::ParseIntError;

use crate::constants::{
    self,
    keywords::conditions::{CONDITION_GROUP_END, CONDITION_GROUP_START},
};

#[derive(Debug, Clone)]
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

impl From<ParseMatchError> for ParseQueryError {
    fn from(value: ParseMatchError) -> Self {
        ParseQueryError::new(ParseErrorReason::ParseMatchError(value))
    }
}

impl From<ParseSubqueryError> for ParseQueryError {
    fn from(value: ParseSubqueryError) -> Self {
        ParseQueryError::new(ParseErrorReason::ParseSubquery(value))
    }
}

impl<'a> std::error::Error for ParseQueryError {}
impl<'a> std::fmt::Display for ParseQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing query failed because of {}", self.reason,)
    }
}

pub type ErrorSection = (usize, usize);

#[derive(Debug, PartialEq, Clone)]
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
    ParseMatchError(ParseMatchError),
    ParseSubquery(ParseSubqueryError),

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
                ParseErrorReason::ParseMatchError(err) => &format!("Parsing match failed: {err}"),
                ParseErrorReason::ParseSubquery(err) => &format!("Parsing subquery failed: {err}"),

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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum ParseKeyValueErrorReason {
    MissingSpace,
    MissingAssignment,
    MissingPropertyStr,
    MissingValue { for_key: String },
    UnclosedSingleQuote,
    UnclosedDoubleQuote,
    Default,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParseMatchError {
    pub reason: ParseMatchErrorReason,
    pub pattern: String,
}

impl std::error::Error for ParseMatchError {}
impl std::fmt::Display for ParseMatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.reason {
            ParseMatchErrorReason::StartWithoutNode => write!(
                f,
                "pattern '{}' does not start with a Node: '(name:type)'",
                self.pattern
            ),
            ParseMatchErrorReason::BadRelationship => write!(
                f,
                "Relationship was not closed properly in pattern {}",
                self.pattern
            ),
            ParseMatchErrorReason::ParseNameType => write!(
                f,
                "Parsing name & type failed for pattern: {}",
                self.pattern
            ),
            ParseMatchErrorReason::ParseConditions { err: err } => {
                write!(
                    f,
                    "Parsing conditions failed due to {:?} for pattern: {}",
                    err, self.pattern
                )
            }
            ParseMatchErrorReason::ParseExpression { err: err } => {
                write!(
                    f,
                    "Parsing expression failed due to {:?} for pattern: {}",
                    err, self.pattern
                )
            }
            ParseMatchErrorReason::ParseReturnValues => write!(
                f,
                "Parsing return values failed for pattern: {}",
                self.pattern
            ),
            ParseMatchErrorReason::UnknownIdentifierInReturnValues { unknown } => write!(
                f,
                "Unknown identifier '{unknown}' in return values for pattern: {}",
                self.pattern
            ),
        }
    }
}

impl ParseMatchError {
    pub fn new(reason: ParseMatchErrorReason, pattern: String) -> Self {
        ParseMatchError { reason, pattern }
    }
}

impl From<ParseConditionsError> for ParseMatchError {
    fn from(value: ParseConditionsError) -> Self {
        ParseMatchError {
            reason: ParseMatchErrorReason::ParseConditions { err: value.reason },
            pattern: value.pattern,
        }
    }
}

impl From<ExpressionError> for ParseMatchError {
    fn from(value: ExpressionError) -> Self {
        ParseMatchError {
            reason: ParseMatchErrorReason::ParseExpression { err: value.reason },
            pattern: value.pattern,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParseMatchErrorReason {
    StartWithoutNode,
    ParseNameType,
    BadRelationship,
    ParseConditions { err: ParseConditionsErrorReason },
    ParseExpression { err: ExpressionErrorReason },
    ParseReturnValues,
    UnknownIdentifierInReturnValues { unknown: String },
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParseSubqueryError {
    subquery: String,
    reason: ParseSubqueryErrorReason,
}

impl ParseSubqueryError {
    pub fn new(subquery: &str, reason: ParseSubqueryErrorReason) -> Self {
        ParseSubqueryError {
            subquery: subquery.to_string(),
            reason,
        }
    }
}
impl std::error::Error for ParseSubqueryError {}
impl std::fmt::Display for ParseSubqueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.reason {
            ParseSubqueryErrorReason::UnexpectedEnd => {
                write!(f, "Unexpected end in '{}'", self.subquery)
            }
            ParseSubqueryErrorReason::NonZeroLevel => {
                write!(f, "NonZeroLevel in '{}'", self.subquery)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParseSubqueryErrorReason {
    UnexpectedEnd,
    NonZeroLevel,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParseConditionsError {
    reason: ParseConditionsErrorReason,
    pattern: String,
}

impl ParseConditionsError {
    pub fn new(reason: ParseConditionsErrorReason, pattern: String) -> Self {
        Self { reason, pattern }
    }
}

impl std::error::Error for ParseConditionsError {}
impl std::fmt::Display for ParseConditionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.reason {
            ParseConditionsErrorReason::UnclosedGroupStart => write!(
                f,
                "Parsing conditions failed: Unclosed '{}' for pattern '{}'",
                CONDITION_GROUP_START, self.pattern
            ),
            ParseConditionsErrorReason::UnclosedGroupEnd => write!(
                f,
                "Parsing conditions failed: Unclosed '{}' for pattern '{}'",
                CONDITION_GROUP_END, self.pattern
            ),
            ParseConditionsErrorReason::LeftHandQuotes => write!(
                f,
                "Parsing conditions failed: Left hand quotes are not allowed. But found in pattern '{}'",
                self.pattern
            ),
            ParseConditionsErrorReason::MissingOperator => write!(
                f,
                "Parsing conditions failed due to missign operator for pattern '{}'",
                self.pattern
            ),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParseConditionsErrorReason {
    UnclosedGroupStart,
    UnclosedGroupEnd,
    LeftHandQuotes,
    MissingOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionError {
    reason: ExpressionErrorReason,
    pattern: String,
}

impl ExpressionError {
    pub fn new(reason: ExpressionErrorReason, pattern: String) -> Self {
        Self { reason, pattern }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionErrorReason {
    MissingExpectedChar(char),
    PropertyOfProperty,
    ParseConstant,
    InvalidConstant,
}
