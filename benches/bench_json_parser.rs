#![feature(test)]

extern crate test;

use cursortanium::{
    helpers::run_parser_test, parsers::json_parser, Cursor
};
use test::Bencher;

#[bench]
fn bench_json_parser(b: &mut Bencher) {
    b.iter(|| {
        run_parser_test(
            r#"
                🧀{
                    "name": "Lia",
                    "age": 18
                }🧀
            "#,
            r#"
                Some(
                    Object((
                        fields: [
                            FieldToken(
                                name: "name",
                                value: String((
                                    value: "Lia"
                                )),
                            ),
                            FieldToken(
                                name: "age",
                                value: Number((
                                    value: 18
                                )),
                            ),
                        ],
                    ))
                )
            "#,
            |cursor: &mut Cursor| {
                json_parser::parse(&mut *cursor)
            },
        );
    });
}
