use cursortanium::{
    helpers::run_parser_test, parsers::json_parser, Cursor,
};

#[test]
fn test_parse_string() {
    run_parser_test(
        r#"
            🧀"Autumn shows us how beautiful it is to let thing go."🧀
        "#,
        r#"
            Some(
                String((
                    value: "Autumn shows us how beautiful it is to let thing go."
                ))
            )
        "#,
        |cursor: &mut Cursor| json_parser::parse(&mut *cursor),
    );
}

#[test]
fn test_parse_number() {
    run_parser_test(
        r#"
            🧀1234🧀
        "#,
        r#"
            Some(
                Number((
                    value: 1234,
                ))
            )
        "#,
        |cursor: &mut Cursor| json_parser::parse(&mut *cursor),
    );
}

#[test]
fn test_parse_array() {
    run_parser_test(
        r#"
            🧀[1, 2, 3, 4]🧀
        "#,
        r#"
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
        "#,
        |cursor: &mut Cursor| json_parser::parse(&mut *cursor),
    );
}

#[test]
fn test_parse_object() {
    run_parser_test(
        r#"
            🧀{
                "name": "Tim Carousel",
                "age": 24
            }🧀
        "#,
        r#"
            Some(
                Object((
                    fields: [
                        FieldToken(
                            name: "name",
                            value: String((
                                value: "Tim Carousel",
                            )),
                        ),
                        FieldToken(
                            name: "age",
                            value: Number((
                                value: 24,
                            )),
                        )
                    ],
                ))
            )
        "#,
        |cursor: &mut Cursor| json_parser::parse(&mut *cursor),
    );
}
