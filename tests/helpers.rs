use cursortanium::{parsers::json_parser, Cursor, Test};
use ron::de;

pub fn run_parser_test<
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
