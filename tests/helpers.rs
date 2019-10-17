use cursortanium::{test::capture, Cursor};

pub struct TestCase(pub String, pub String);

pub fn run_parse_test<F: Fn(&mut Cursor) -> Option<String>>(
    f: F,
    test_case: TestCase,
) {
    let mut iter = capture(test_case.0.into()).iter();

    let token = iter.next().and_then(|cursor| {
        iter.next().and_then(|marker| {
            let mut cursor = cursor.clone();
            let token = f(&mut cursor);

            assert!(cursor.is_at(&marker));

            token
        })
    });

    match token {
        Some(token) => assert_eq!(token, test_case.1),
        None => assert!(false),
    }
}
