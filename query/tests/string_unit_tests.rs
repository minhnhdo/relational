#[test]
fn simple_string() {
    let value = query::grammar::StringParser::new()
        .parse("'this is a string'")
        .unwrap();
    assert_eq!(format!("{:?}", value), "'this is a string'");
}

#[test]
fn escaped_single_quote_string() {
    let value = query::grammar::StringParser::new()
        .parse("'Dianne''s horse'")
        .unwrap();
    assert_eq!(format!("{:?}", value), "'Dianne's horse'");
}
