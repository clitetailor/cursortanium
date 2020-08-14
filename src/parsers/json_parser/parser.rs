use crate::Cursor;

use super::super::utils;
use super::tokens::{Field, Value};

pub fn parse(cursor: &mut Cursor) -> Option<Value> {
    let ch = cursor.get_char();

    match ch {
        Some('[') => parse_array(&mut *cursor),
        Some('{') => parse_object(&mut *cursor),
        Some('n') => parse_null(&mut *cursor),
        Some('"') => parse_string(&mut *cursor),
        _ => parse_number(&mut *cursor),
    }
}

const WS: &[char] = &[' ', '\t', '\r', '\n'];

pub fn skip_ws(cursor: &mut Cursor) {
    while match cursor.get_char() {
        Some(ch) => WS.contains(&ch),
        None => false,
    } {
        cursor.next(1);
    }
}

pub fn parse_object(cursor: &mut Cursor) -> Option<Value> {
    let checkpoint = cursor.clone();

    None.or_else(|| {
        if cursor.get_char() != Some('{') {
            return None;
        }

        cursor.next(1);
        skip_ws(&mut *cursor);

        let mut fields: Vec<Field> = vec![];

        while match cursor.get_char() {
            Some('}') => false,
            _ => true,
        } && !cursor.is_eof()
        {
            skip_ws(&mut *cursor);

            match parse_field(&mut *cursor) {
                Some(field) => {
                    fields.push(field);
                    skip_ws(&mut *cursor);
                    if cursor.get_char() == Some('}') {
                        break;
                    };
                    if cursor.get_char() == Some(',') {
                        cursor.next(1);
                    };
                }
                _ => break,
            };
        }

        match cursor.get_char() {
            Some('}') => {
                cursor.next(1);
                Some(Value::Object(fields))
            }
            _ => None,
        }
    })
    .or_else(|| {
        cursor.move_to(&checkpoint);

        None
    })
}

pub fn parse_array(cursor: &mut Cursor) -> Option<Value> {
    let checkpoint = cursor.clone();

    match cursor.get_char() {
        Some('[') => {
            cursor.next(1);

            skip_ws(&mut *cursor);

            let mut elements: Vec<Value> = vec![];

            while cursor.get_char() != Some(']')
                && !cursor.is_eof()
            {
                skip_ws(&mut *cursor);

                match parse(&mut *cursor) {
                    Some(element) => {
                        elements.push(element);
                        skip_ws(&mut *cursor);
                        if cursor.get_char() == Some(']') {
                            break;
                        };
                        if cursor.get_char() == Some(',') {
                            cursor.next(1);
                        };
                    }
                    None => {
                        break;
                    }
                };
            }

            match cursor.get_char() {
                Some(']') => {
                    cursor.next(1);
                    Some(Value::Array(elements))
                }
                _ => None,
            }
        }
        _ => None,
    }
    .or_else(|| {
        cursor.move_to(&checkpoint);

        None
    })
}

pub fn parse_field(cursor: &mut Cursor) -> Option<Field> {
    let checkpoint = cursor.clone();

    utils::parse_string(&mut *cursor)
        .and_then(|name| {
            skip_ws(&mut *cursor);

            if cursor.get_char() != Some(':') {
                return None;
            }
            cursor.next(1);
            skip_ws(&mut *cursor);

            parse(&mut *cursor).map(|value| {
                skip_ws(&mut *cursor);
                Field(name.to_owned(), Box::new(value))
            })
        })
        .or_else(|| {
            cursor.move_to(&checkpoint);

            None
        })
}

pub fn parse_string(cursor: &mut Cursor) -> Option<Value> {
    utils::parse_string(&mut *cursor)
        .map(|value| Value::String(value.to_owned()))
}

pub fn is_number(cursor: &Cursor) -> bool {
    match cursor.get_char() {
        Some(ch) => ch.is_digit(10),
        None => false,
    }
}

pub fn parse_number(cursor: &mut Cursor) -> Option<Value> {
    let checkpoint = cursor.clone();

    None.or_else(|| {
        if cursor.get_char() == Some('-') {
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

        if cursor.get_char() == Some('.') {
            cursor.next(1);

            if is_number(&cursor) {
                cursor.next(1);
            } else {
                return None;
            }

            while is_number(&cursor) {
                cursor.next(1);
            }
        }

        let value: &str = checkpoint.take_until(&cursor);

        Some(Value::Number(value.to_owned()))
    })
    .or_else(|| {
        cursor.move_to(&checkpoint);

        None
    })
}

pub fn parse_null(cursor: &mut Cursor) -> Option<Value> {
    if cursor.starts_with("null") {
        cursor.next(4);

        return Some(Value::Null);
    }

    return None;
}
