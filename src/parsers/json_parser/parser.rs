use crate::Cursor;

use super::super::utils;
use super::tokens::Value;

lazy_static! {
    static ref WHITESPACES: std::vec::Vec<&'static str> =
        Vec::from([" ", "\t", "\r", "\n"]);
    static ref DIGITS: Vec<&'static str> = Vec::from([
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ]);
}

pub fn parse(cursor: &mut Cursor) -> Option<Value> {
    parse_null(&mut *cursor)
        .or_else(|| parse_string(&mut *cursor))
        .or_else(|| parse_number(&mut *cursor))
        .or_else(|| parse_array(&mut *cursor))
        .or_else(|| parse_object(&mut *cursor))
        .or_else(|| parse_boolean(&mut *cursor))
}

pub fn skip_ws(cursor: &mut Cursor) {
    while cursor.one_of(&WHITESPACES) != None {
        cursor.next(&1);
    }
}

pub fn parse_object(cursor: &mut Cursor) -> Option<Value> {
    let last_pos = cursor.clone();

    if !cursor.starts_with("{") {
        return None;
    }

    cursor.next(&1);

    skip_ws(&mut *cursor);

    let mut fields: Vec<(String, Box<Value>)> = vec![];

    while !cursor.starts_with("}") && !cursor.is_eof() {
        skip_ws(&mut *cursor);

        if let Some(field) = parse_field(&mut *cursor) {
            fields.push(field);

            skip_ws(&mut *cursor);

            if cursor.starts_with("}") {
                break;
            };

            if cursor.starts_with(",") {
                cursor.next(&1);
            };
        } else {
            break;
        };
    }

    if !cursor.starts_with("}") {
        *cursor = last_pos;

        return None;
    }

    cursor.next(&1);

    Some(Value::Object(fields))
}

pub fn parse_array(cursor: &mut Cursor) -> Option<Value> {
    let last_pos = cursor.clone();

    if !cursor.starts_with("[") {
        *cursor = last_pos;

        return None;
    }

    cursor.next(&1);

    skip_ws(&mut *cursor);

    let mut elements: Vec<Value> = vec![];

    while !cursor.starts_with("]") && !cursor.is_eof() {
        skip_ws(&mut *cursor);

        if let Some(element) = parse(&mut *cursor) {
            elements.push(element);
            skip_ws(&mut *cursor);

            if cursor.starts_with("]") {
                break;
            };

            if cursor.starts_with(",") {
                cursor.next(&1);
            };
        } else {
            break;
        };
    }

    if !cursor.starts_with("]") {
        *cursor = last_pos;

        return None;
    }

    cursor.next(&1);

    Some(Value::Array(elements))
}

pub fn parse_field(
    cursor: &mut Cursor,
) -> Option<(String, Box<Value>)> {
    let last_pos = cursor.clone();

    let name = match utils::parse_string(&mut *cursor) {
        Some(name) => name,
        None => {
            *cursor = last_pos;

            return None;
        }
    };

    skip_ws(&mut *cursor);

    if !cursor.starts_with(":") {
        *cursor = last_pos;

        return None;
    };

    cursor.next(&1);

    skip_ws(&mut *cursor);

    let value = match parse(&mut *cursor) {
        Some(value) => value,
        None => {
            *cursor = last_pos;

            return None;
        }
    };

    skip_ws(&mut *cursor);

    Some((name, Box::new(value)))
}

pub fn parse_string(cursor: &mut Cursor) -> Option<Value> {
    match utils::parse_string(&mut *cursor) {
        Some(value) => Some(Value::String(value)),
        None => None,
    }
}

pub fn is_number(cursor: &Cursor) -> bool {
    cursor.one_of(&DIGITS).is_some()
}

pub fn parse_number(cursor: &mut Cursor) -> Option<Value> {
    let last_pos = cursor.clone();

    if cursor.starts_with("-") {
        cursor.next(&1);
    };

    if is_number(&cursor) {
        cursor.next(&1);
    } else {
        *cursor = last_pos;

        return None;
    }

    while is_number(&cursor) {
        cursor.next(&1);
    }

    if cursor.starts_with(".") {
        cursor.next(&1);
        if is_number(&cursor) {
            cursor.next(&1);
        } else {
            *cursor = last_pos;

            return None;
        }

        while is_number(&*cursor) {
            cursor.next(&1);
        }
    }

    let value: f64 = last_pos
        .take_until(&cursor.get_index())
        .parse()
        .ok()?;

    Some(Value::Number(value))
}

pub fn parse_boolean(cursor: &mut Cursor) -> Option<Value> {
    if cursor.starts_with("true") {
        cursor.next(&4);

        return Some(Value::Boolean(true));
    }

    if cursor.starts_with("false") {
        cursor.next(&5);

        return Some(Value::Boolean(false));
    }

    return None;
}

pub fn parse_null(cursor: &mut Cursor) -> Option<Value> {
    if cursor.starts_with("null") {
        cursor.next(&4);

        return Some(Value::Null);
    }

    return None;
}
