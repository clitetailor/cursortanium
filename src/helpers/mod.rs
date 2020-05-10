use crate::parsers::json_parser;
use crate::test::capture;
use crate::Cursor;

use ron::de;

pub fn run_parser_test<
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
