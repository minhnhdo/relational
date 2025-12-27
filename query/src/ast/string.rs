#[derive(Copy, Clone)]
pub enum StringParseError {
    InvalidEscapeSequence,
    ExpectedEscapedSingleQuote,
}

impl StringParseError {
    pub fn message(&self) -> &'static str {
        match self {
            StringParseError::InvalidEscapeSequence => "invalid escape sequence",
            StringParseError::ExpectedEscapedSingleQuote => "expected escaped single quote",
        }
    }
}

enum ParseState {
    SimpleCharacter,
    EscapedSingleQuote,
}

pub fn parse_sql_string(s: &str) -> Result<String, StringParseError> {
    let mut result = String::new();
    let mut parse_state = ParseState::SimpleCharacter;
    for c in s.chars() {
        match parse_state {
            ParseState::SimpleCharacter => match c {
                '\'' => parse_state = ParseState::EscapedSingleQuote,
                _ => result.push(c),
            },
            ParseState::EscapedSingleQuote => match c {
                '\'' => {
                    parse_state = ParseState::SimpleCharacter;
                    result.push('\'');
                }
                _ => return Err(StringParseError::ExpectedEscapedSingleQuote),
            },
        }
    }
    Ok(result)
}
