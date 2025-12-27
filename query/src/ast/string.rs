#[derive(Copy, Clone)]
pub enum StringParseError {
    ExpectedBeginningSingleQuote,
    ExpectedEscapedSingleQuote,
    ExpectedEndingSingleQuote,
}

impl StringParseError {
    pub fn message(&self) -> &'static str {
        match self {
            StringParseError::ExpectedBeginningSingleQuote => "expected beginning single quote",
            StringParseError::ExpectedEscapedSingleQuote => "expected escaped single quote",
            StringParseError::ExpectedEndingSingleQuote => "expected ending single quote",
        }
    }
}

enum ParseState {
    BeginningSingleQuote,
    SimpleCharacter,
    EscapedSingleQuote,
}

pub fn parse_sql_string(s: &str) -> Result<String, StringParseError> {
    let mut result = String::new();
    let mut parse_state = ParseState::BeginningSingleQuote;
    let mut last_char = '\'';
    for c in s.chars() {
        match parse_state {
            ParseState::BeginningSingleQuote => match c {
                '\'' => parse_state = ParseState::SimpleCharacter,
                _ => return Err(StringParseError::ExpectedBeginningSingleQuote),
            },
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
        last_char = c;
    }

    if last_char != '\'' {
        return Err(StringParseError::ExpectedBeginningSingleQuote);
    }
    Ok(result)
}
