#[test]
fn simple_string() {
    let value = query::grammar::StringParser::new()
        .parse("'this is a string'")
        .unwrap();
    assert_eq!(format!("{:?}", value), "'this is a string'");
}
