#[macro_use]
extern crate lazy_static;

mod parsers;

use cursortanium::{capture, Cursor};
use parsers::json_parser;
use ron::de;

fn run_parser_test<
    T: Fn(&mut Cursor) -> Option<json_parser::ValueToken>,
>(
    input: &str,
    expect: &str,
    parse: T,
) {
    let mut iter = capture(input).into_iter();

    let cursor = iter.next();
    let target = iter.next();

    assert!(cursor.is_some());
    assert!(target.is_some());

    let mut cursor = cursor.unwrap();

    let ast = parse(&mut cursor);

    let expect: Option<json_parser::ValueToken> =
        de::from_str::<Option<json_parser::ValueToken>>(
            &expect,
        )
        .unwrap();

    assert_eq!(ast, expect);
}

#[test]
fn test_parse_string() {
    run_parser_test(
        r###"
            🍁"Autumn shows us how beautiful it is to let thing go."🍁
        "###,
        r###"
            Some(
                String((
                    value: "Autumn shows us how beautiful it is to let thing go."
                ))
            )
        "###,
        |cursor: &mut Cursor| json_parser::parse(&mut *cursor),
    );
}

#[test]
fn test_parse_number() {
    run_parser_test(
        r###"
            🍁1234🍁
        "###,
        r###"
            Some(
                Number((
                    value: 1234,
                ))
            )
        "###,
        |cursor: &mut Cursor| json_parser::parse(&mut *cursor),
    );
}

#[test]
fn test_parse_array() {
    run_parser_test(
        r###"
            🍁[1, 2, 3, 4]🍁
        "###,
        r###"
            Some(
                Array((
                    elements: [
                        Number((
                            value: 1,
                        )),
                        Number((
                            value: 2,
                        )),
                        Number((
                            value: 3,
                        )),
                        Number((
                            value: 4,
                        )),
                    ],
                ))
            )
        "###,
        |cursor: &mut Cursor| json_parser::parse(&mut *cursor),
    );
}

#[test]
fn test_parse_object() {
    run_parser_test(
        r###"
            🍁{ "name":"John", "age":30, "car":null }🍁
        "###,
        r###"
            Some(
                Object((
                    fields: [
                        FieldToken(
                            name: "name",
                            value: String((
                                value: "John",
                            )),
                        ),
                        FieldToken(
                            name: "age",
                            value: Number((
                                value: 30,
                            )),
                        ),
                        FieldToken(
                            name: "car",
                            value: Null(()),
                        ),
                    ],
                ))
            )
        "###,
        |cursor: &mut Cursor| json_parser::parse(&mut *cursor),
    );
}