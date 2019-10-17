mod helpers;

use cursortanium::Cursor;
use helpers::{run_parse_test, TestCase};

fn parse(cursor: &mut Cursor) -> Option<String> {
    let mut tokens = vec![];

    while let Some(token) = parse_token(&mut *cursor) {
        tokens.push(token);
    }

    Some(tokens.concat())
}

fn parse_token(cursor: &mut Cursor) -> Option<String> {
    match parse_hyphens(&mut *cursor) {
        None => parse_string(&mut *cursor),
        a => a,
    }
}

fn parse_hyphens(cursor: &mut Cursor) -> Option<String> {
    if !cursor.starts_with("-") {
        return None;
    }

    let marker = cursor.clone();

    while cursor.starts_with("-") {
        cursor.next_mut(1);
    }

    Some(marker.take_until(&cursor))
}

fn parse_string(cursor: &mut Cursor) -> Option<String> {
    let mut chunks = vec![];

    if cursor.starts_with("\"") {
        cursor.next_mut(1);
    } else {
        return None;
    };

    let mut marker = cursor.clone();

    while !cursor.starts_with("\"") && !cursor.is_eof() {
        if cursor.starts_with("\\") {
            chunks.push(marker.take_until(&cursor));
            cursor.next_mut(1);
            marker = cursor.clone();
            cursor.next_mut(1);
        } else {
            cursor.next_mut(1);
        }
    }

    chunks.push(marker.take_until(&cursor));

    if cursor.starts_with("\"") {
        cursor.next_mut(1);
    };

    Some(chunks.concat())
}

#[test]
fn test_parse_string() {
    let mut test_cases = vec![];

    test_cases.push(TestCase(
        String::from(
            r#"
                🌖----"Hello, \"World\""----🌖
            "#,
        ),
        String::from(r#"----Hello, "World"----"#),
    ));

    for test_case in test_cases {
        run_parse_test(parse, test_case);
    }
}
