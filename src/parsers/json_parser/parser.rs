use regex::Regex;

use crate::Cursor;

use super::super::utils;
use super::tokens::{
    ArrayToken, FieldToken, NullToken, NumberToken,
    ObjectToken, StringToken, ValueToken,
};

pub fn parse(cursor: &mut Cursor) -> Option<ValueToken> {
    parse_null(&mut *cursor)
        .or_else(|| parse_string(&mut *cursor))
        .or_else(|| parse_number(&mut *cursor))
        .or_else(|| parse_array(&mut *cursor))
        .or_else(|| parse_object(&mut *cursor))
}

pub fn skip_ws(cursor: &mut Cursor) {
    while cursor.one_of(&[" ", "\t", "\r", "\n"]) != None {
        cursor.next(1);
    }
}

pub fn parse_object(cursor: &mut Cursor) -> Option<ValueToken> {
    let checkpoint = cursor.clone();

    if !cursor.starts_with("{") {
        None
    } else {
        cursor.next(1);

        skip_ws(&mut *cursor);

        let mut fields: Vec<FieldToken> = vec![];

        while !cursor.starts_with("}") && !cursor.is_eof() {
            skip_ws(&mut *cursor);

            if let Some(field) = parse_field(&mut *cursor) {
                fields.push(field);

                skip_ws(&mut *cursor);

                if cursor.starts_with("}") {
                    break;
                };

                if cursor.starts_with(",") {
                    cursor.next(1);
                };
            } else {
                break;
            };
        }

        if !cursor.starts_with("}") {
            None
        } else {
            cursor.next(1);
            Some(ValueToken::Object(ObjectToken { fields }))
        }
    }
    .or_else(|| {
        cursor.move_to(&checkpoint);

        None
    })
}

pub fn parse_array(cursor: &mut Cursor) -> Option<ValueToken> {
    let checkpoint = cursor.clone();

    if !cursor.starts_with("[") {
        None
    } else {
        cursor.next(1);

        skip_ws(&mut *cursor);

        let mut elements: Vec<ValueToken> = vec![];

        while !cursor.starts_with("]") && !cursor.is_eof() {
            skip_ws(&mut *cursor);

            if let Some(element) = parse(&mut *cursor) {
                elements.push(element);
                skip_ws(&mut *cursor);

                if cursor.starts_with("]") {
                    break;
                };

                if cursor.starts_with(",") {
                    cursor.next(1);
                };
            } else {
                break;
            };
        }

        if !cursor.starts_with("]") {
            None
        } else {
            cursor.next(1);

            Some(ValueToken::Array(ArrayToken { elements }))
        }
    }
    .or_else(|| {
        cursor.move_to(&checkpoint);

        None
    })
}

pub fn parse_field(cursor: &mut Cursor) -> Option<FieldToken> {
    let checkpoint = cursor.clone();

    utils::parse_string(&mut *cursor)
        .and_then(|name| {
            skip_ws(&mut *cursor);

            if !cursor.starts_with(":") {
                return None;
            };

            cursor.next(1);

            skip_ws(&mut *cursor);

            parse(&mut *cursor).map(|value| {
                skip_ws(&mut *cursor);

                FieldToken {
                    name,
                    value: Box::new(value),
                }
            })
        })
        .or_else(|| {
            cursor.move_to(&checkpoint);

            None
        })
}

pub fn parse_string(cursor: &mut Cursor) -> Option<ValueToken> {
    utils::parse_string(&mut *cursor)
        .map(|value| ValueToken::String(StringToken { value }))
}

lazy_static! {
    static ref NUMBER_REGEX: Regex =
        Regex::new("[0-9]+").unwrap();
}

pub fn parse_number(cursor: &mut Cursor) -> Option<ValueToken> {
    let checkpoint = cursor.clone();

    cursor
        .clone()
        .r#match(&NUMBER_REGEX)
        .and_then(|mat| {
            let value = mat.as_str();

            cursor.next(value.chars().count());

            let value: isize = value.parse().ok()?;

            Some(ValueToken::Number(NumberToken { value }))
        })
        .or_else(|| {
            cursor.move_to(&checkpoint);

            None
        })
}

lazy_static! {
    static ref NULL_REGEX: Regex = Regex::new("null").unwrap();
}

pub fn parse_null(cursor: &mut Cursor) -> Option<ValueToken> {
    let checkpoint = cursor.clone();

    cursor
        .clone()
        .r#match(&NULL_REGEX)
        .and_then(|_| {
            cursor.next(4);

            Some(ValueToken::Null(NullToken))
        })
        .or_else(|| {
            cursor.move_to(&checkpoint);

            None
        })
}
