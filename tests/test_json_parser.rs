#[macro_use]
extern crate lazy_static;

use cursortanium::{parsers::json_parser, Cursor, Test};
use ron::de;

fn run_parser_test<
    T: Fn(&mut Cursor) -> Option<json_parser::Value>,
>(
    input: &str,
    expect: &str,
    parse: T,
) {
    let capture_result = Test {
        no_label: true,
        prefix: String::from('*'),
    }
    .capture(input);

    let mut iter = capture_result.into_iter();

    let cursor = iter.next();
    let target = iter.next();

    assert!(cursor.is_some());
    assert!(target.is_some());

    let mut cursor = cursor.unwrap();

    let ast = parse(&mut cursor);

    let expect: Option<json_parser::Value> =
        de::from_str::<Option<json_parser::Value>>(&expect)
            .unwrap();

    assert_eq!(ast, expect);
}

#[test]
fn test_parse_string() {
    run_parser_test(
        r#"
            *"Autumn shows us how beautiful it is to let thing go."*
        "#,
        r#"
            Some(
                String("Autumn shows us how beautiful it is to let thing go.")
            )
        "#,
        |cursor: &mut Cursor| json_parser::parse(&mut *cursor),
    );
}

#[test]
fn test_parse_number() {
    run_parser_test(
        r#"
            *1234*
        "#,
        r#"
            Some(Number(1234))
        "#,
        |cursor: &mut Cursor| json_parser::parse(&mut *cursor),
    );
}

#[test]
fn test_parse_array() {
    run_parser_test(
        r#"
            *[1, 2, 3, 4]*
        "#,
        r#"
            Some(
                Array([
                        Number(1),
                        Number(2),
                        Number(3),
                        Number(4),
                ])
            )
        "#,
        |cursor: &mut Cursor| json_parser::parse(&mut *cursor),
    );
}

#[test]
fn test_parse_object() {
    run_parser_test(
        r#"
            *{
                "name": "Tim Carousel",
                "age": 24
            }*
        "#,
        r#"
            Some(
                Object([
                    (
                        "name",
                        String("Tim Carousel"),
                    ),
                    (
                        "age",
                        Number(24),
                    )
                ])
            )
        "#,
        |cursor: &mut Cursor| json_parser::parse(&mut *cursor),
    );
}
