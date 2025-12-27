use query::ast::{Expression, Literal, ResultColumn, SelectStatement};

#[test]
fn simple_select() {
    let value = query::grammar::SelectStatementParser::new()
        .parse("SELECT 'foo'")
        .unwrap();
    assert_eq!(
        value,
        SelectStatement {
            result_columns: vec![ResultColumn::Expression(Expression::Literal(
                Literal::String("foo".to_string())
            ))]
        }
    );
}

#[test]
fn select_concatenated_string() {
    let value = query::grammar::SelectStatementParser::new()
        .parse("SELECT 'foo'\n'bar'")
        .unwrap();
    assert_eq!(
        value,
        SelectStatement {
            result_columns: vec![ResultColumn::Expression(Expression::Literal(
                Literal::String("foobar".to_string())
            ))]
        }
    );
}

#[test]
fn select_multiple_expressions() {
    let value = query::grammar::SelectStatementParser::new()
        .parse("SELECT 'foo', 'bar'")
        .unwrap();
    assert_eq!(
        value,
        SelectStatement {
            result_columns: vec![
                ResultColumn::Expression(Expression::Literal(Literal::String("foo".to_string()))),
                ResultColumn::Expression(Expression::Literal(Literal::String("bar".to_string())))
            ]
        }
    );
}
