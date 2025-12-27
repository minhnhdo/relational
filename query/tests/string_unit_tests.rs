use query::ast::Literal;

#[test]
fn simple_string() {
    let value = query::grammar::StringParser::new()
        .parse("'this is a string'")
        .unwrap();
    assert_eq!(value, Literal::String("this is a string".to_string()));
}

#[test]
fn escaped_single_quote_string() {
    let value = query::grammar::StringParser::new()
        .parse("'Dianne''s horse'")
        .unwrap();
    assert_eq!(value, Literal::String("Dianne's horse".to_string()));
}

#[test]
fn concatenated_string() {
    let value = query::grammar::StringParser::new()
        .parse("'Dianne''s horse' ' is a good horse'")
        .unwrap();
    assert_eq!(
        value,
        Literal::String("Dianne's horse is a good horse".to_string())
    );
}
