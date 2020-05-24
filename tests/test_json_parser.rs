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
            🧀1234🧀
        "#,
        r#"
            Some(
                Number(1234)
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
            🧀{
                "name": "Tim Carousel",
                "age": 24
            }🧀
        "#,
        r#"
            Some(
                Object([
                    ("name", String("Tim Carousel")),
                    ("age", Number(24))
                ])
            )
        "#,
        |cursor: &mut Cursor| json_parser::parse(&mut *cursor),
    );
}
