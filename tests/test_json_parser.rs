mod parsers;

use cursortanium::capture;
use parsers::json_parser;

#[test]
fn test_json_parser() {
    let mut iter = capture(String::from(r###"

        🍁"Autumn shows us how beautiful it is to let thing go."🍁

    "###)).into_iter();

    let cursor = iter.next();
    let target = iter.next();

    assert!(cursor.is_some());
    assert!(target.is_some());

    if let (Some(mut cursor), Some(target)) = (cursor, target) {
        assert_eq!(json_parser::parse(&mut cursor), Some(json_parser::Token::String{
            value: String::from("Autumn shows us how beautiful it is to let thing go.")
        }));

        assert!(cursor.is_at(&target));
    };
}
