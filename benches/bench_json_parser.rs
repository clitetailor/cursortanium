#![feature(test)]

extern crate test;

extern crate pest;
extern crate pest_grammars;

use cursortanium::{parsers::json_parser, Cursor};
use pest::Parser;
use pest_grammars::json::*;
use serde_json;
use serde_json::Value;
use test::Bencher;

const DATA: &str = include_str!("../assets/canada.json");

#[bench]
fn bench_cursortanium_json_parser(b: &mut Bencher) {
    b.iter(|| {
        let ast = json_parser::parse(&mut Cursor::from(DATA));

        ast
    });
}

#[bench]
fn bench_serde_json_parser(b: &mut Bencher) {
    b.iter(|| {
        let value: Option<Value> =
            serde_json::from_str(DATA).ok();

        value
    });
}

#[bench]
fn bench_pest_json_parser(b: &mut Bencher) {
    b.iter(|| {
        let tokens: Vec<_> =
            JsonParser::parse(Rule::json, DATA)
                .unwrap()
                .collect();

        tokens
    });
}
