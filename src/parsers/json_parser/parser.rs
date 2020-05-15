use crate::Cursor;

use super::super::utils;
use super::tokens::{
    ArrayToken, FieldToken, NullToken, NumberToken,
    ObjectToken, StringToken, ValueToken,
};

pub fn parse(cursor: &mut Cursor) -> Option<ValueToken> {
    if cursor.starts_with("[") {
        return parse_array(&mut *cursor);
    }

    if cursor.starts_with("{") {
        return parse_object(&mut *cursor);
    }

    if cursor.starts_with("n") {
        return parse_null(&mut *cursor);
    }

    if cursor.starts_with("\"") {
        return parse_string(&mut *cursor);
    }

    return parse_number(&mut *cursor);
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

const DIGITS: &[&str] =
    &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

pub fn is_number(cursor: &Cursor) -> bool {
    if cursor.one_of(&DIGITS).is_some() {
        true
    } else {
        false
    }
}

pub fn parse_number(cursor: &mut Cursor) -> Option<ValueToken> {
    let checkpoint = cursor.clone();

    None.or_else(|| {
        if cursor.starts_with("-") {
            cursor.next(1);
        };

        if is_number(&cursor) {
            cursor.next(1);
        } else {
            return None;
        }

        while is_number(&cursor) {
            cursor.next(1);
        }

        if cursor.starts_with(".") {
            cursor.next(1);

            if is_number(&cursor) {
                cursor.next(1);
            } else {
                return None;
            }

            while is_number(&*cursor) {
                cursor.next(1);
            }
        }

        let value: f64 =
            checkpoint.take_until(&cursor).parse().ok()?;

        Some(ValueToken::Number(NumberToken { value }))
    })
    .or_else(|| {
        cursor.move_to(&checkpoint);

        None
    })
}

pub fn parse_null(cursor: &mut Cursor) -> Option<ValueToken> {
    if cursor.starts_with("null") {
        cursor.next(4);

        return Some(ValueToken::Null(NullToken));
    }

    return None;
}
