
#[derive(Debug)]
pub struct ParseQueryError {
    pub reason: ParseErrorReason,
    pub error_section: ErrorSection,
}

impl ParseQueryError {
    pub fn new (reason: ParseErrorReason, err_section_start: usize, err_section_end: usize) -> ParseQueryError {
        ParseQueryError {
            reason,
            error_section: (err_section_start, err_section_end)
        }
    }


    pub fn default () -> ParseQueryError {
        ParseQueryError {
            reason: ParseErrorReason::Default, 
            error_section: (0,1),
        }
    }
}



impl std::error::Error for ParseQueryError {}
impl std::fmt::Display for ParseQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing query failed because of {} at ({} - {})", 
            self.reason,
            self.error_section.0,
            self.error_section.1,
        )
    }
}


pub type ErrorSection = (usize, usize);


#[derive(Debug)]
pub enum ParseErrorReason {
    InvalidKeyword (String),
    InvalidObjectKind (String),
    IdentifierMissingType,
    MissingTypeName,
    ParseKeyValuePairs,
    TooLongIdentifier (usize, usize),
    Other,
    Default,
}

impl std::fmt::Display for ParseErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = {
            match self {
                ParseErrorReason::InvalidKeyword (invalid_kw) => &format!("Invalid key word: {invalid_kw}"),
                ParseErrorReason::InvalidObjectKind(invalid_obj_kind) => &format!("Invalid object kind: {invalid_obj_kind}"),
                ParseErrorReason::IdentifierMissingType => "Identifier missing type",
                ParseErrorReason::MissingTypeName => "Identifier missing type name",
                ParseErrorReason::ParseKeyValuePairs => "Parse key value pairs failed",
                ParseErrorReason::TooLongIdentifier(actual, max_len) => &format!("Identifier is too long. Max length: {max_len}, got {actual}"),
                ParseErrorReason::Other => "Other",
                ParseErrorReason::Default => "Default",
            }
        };
        write!(f, "{s}")
    }
}



#[derive(Debug)]
pub struct ParseKeyValueError {

}

impl ParseKeyValueError {
    pub fn new () -> ParseKeyValueError {
        ParseKeyValueError {  }
    }
}
