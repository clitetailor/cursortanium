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

pub fn parse<'a, 'b: 'a>(
    cursor: &'b Cursor<'a>,
) -> (&'b Cursor<'a>, Option<Value>) {
    let cursor = match parse_null(cursor) {
        (cursor, None) => cursor,
        (cursor, value) => return (cursor, value),
    };

    let cursor = match parse_string(cursor) {
        (cursor, None) => cursor,
        (cursor, value) => return (cursor, value),
    };

    let cursor = match parse_number(cursor) {
        (cursor, None) => cursor,
        (cursor, value) => return (cursor, value),
    };

    let cursor = match parse_array(cursor) {
        (cursor, None) => cursor,
        (cursor, value) => return (cursor, value),
    };

    let cursor = match parse_object(cursor) {
        (cursor, None) => cursor,
        (cursor, value) => return (cursor, value),
    };

    let cursor = match parse_boolean(cursor) {
        (cursor, None) => cursor,
        (cursor, value) => return (cursor, value),
    };

    (cursor, None)
}

pub fn skip_ws<'a, 'b: 'a>(
    cursor: &'b Cursor<'a>,
) -> &'b Cursor<'a> {
    let mut cursor = cursor;

    while cursor.one_of(&WHITESPACES) != None {
        cursor = &cursor.next(&1);
    }

    cursor
}

pub fn parse_object<'a, 'b: 'a>(
    cursor: &'b Cursor<'a>,
) -> (&'b Cursor<'a>, Option<Value>) {
    let checkpoint = cursor;

    let mut cursor = cursor;

    if !cursor.starts_with("{") {
        return (checkpoint, None);
    }

    cursor = &cursor.next(&1);

    cursor = skip_ws(cursor);

    let mut fields: Vec<(String, Box<Value>)> = vec![];

    while !cursor.starts_with("}") && !cursor.is_eof() {
        cursor = skip_ws(cursor);

        if let (cursor, Some(field)) = parse_field(cursor) {
            fields.push(field);

            cursor = skip_ws(cursor);

            if cursor.starts_with("}") {
                break;
            };

            if cursor.starts_with(",") {
                cursor = &cursor.next(&1);
            };
        } else {
            break;
        };
    }

    if !cursor.starts_with("}") {
        cursor = checkpoint;

        return (cursor, None);
    }

    cursor = &cursor.next(&1);

    (cursor, Some(Value::Object(fields)))
}

pub fn parse_array<'a, 'b: 'a>(
    cursor: &'b Cursor<'a>,
) -> (&'b Cursor<'a>, Option<Value>) {
    let checkpoint = cursor;

    let mut cursor = cursor;

    if !cursor.starts_with("[") {
        return (checkpoint, None);
    }

    cursor = &cursor.next(&1);

    cursor = skip_ws(cursor);

    let mut elements: Vec<Value> = vec![];

    while !cursor.starts_with("]") && !cursor.is_eof() {
        cursor = skip_ws(cursor);

        if let (cursor, Some(element)) = parse(cursor) {
            elements.push(element);
            cursor = skip_ws(cursor);

            if cursor.starts_with("]") {
                break;
            };

            if cursor.starts_with(",") {
                cursor = &cursor.next(&1);
            };
        } else {
            break;
        };
    }

    if !cursor.starts_with("]") {
        return (checkpoint, None);
    }

    cursor = &cursor.next(&1);

    (cursor, Some(Value::Array(elements)))
}

pub fn parse_field<'a, 'b: 'a>(
    cursor: &'b Cursor<'a>,
) -> (&'b Cursor<'a>, Option<(String, Box<Value>)>) {
    let checkpoint = cursor;

    let mut cursor = cursor;

    let name = match utils::parse_string(cursor) {
        (_, Some(name)) => name,
        (_, None) => {
            return (checkpoint, None);
        }
    };

    cursor = skip_ws(cursor);

    if !cursor.starts_with(":") {
        return (checkpoint, None);
    };

    cursor = &cursor.next(&1);

    cursor = skip_ws(cursor);

    let (cursor, value) = match parse(cursor) {
        (cursor, Some(value)) => (cursor, value),
        (cursor, None) => {
            return (cursor, None);
        }
    };

    cursor = skip_ws(cursor);

    (cursor, Some((name, Box::new(value))))
}

pub fn parse_string<'a, 'b: 'a>(
    cursor: &'b Cursor<'a>,
) -> (&'b Cursor<'a>, Option<Value>) {
    match utils::parse_string(cursor) {
        (cursor, Some(value)) => {
            (cursor, Some(Value::String(value)))
        }
        (checkpoint, None) => (checkpoint, None),
    }
}

pub fn is_number(cursor: &Cursor) -> bool {
    cursor.one_of(&DIGITS).is_some()
}

pub fn parse_number<'a, 'b: 'a>(
    cursor: &'b Cursor<'a>,
) -> (&'b Cursor<'a>, Option<Value>) {
    let checkpoint = cursor;

    let mut cursor = cursor;

    if cursor.starts_with("-") {
        cursor = &cursor.next(&1);
    };

    if is_number(&cursor) {
        cursor = &cursor.next(&1);
    } else {
        cursor = checkpoint;

        return (cursor, None);
    }

    while is_number(&cursor) {
        cursor = &cursor.next(&1);
    }

    if cursor.starts_with(".") {
        cursor = &cursor.next(&1);
        if is_number(&cursor) {
            cursor = &cursor.next(&1);
        } else {
            return (checkpoint, None);
        }

        while is_number(&cursor) {
            cursor.next(&1);
        }
    }

    let value: Option<f64> =
        checkpoint.take_until(&cursor.get_index()).parse().ok();

    match value {
        None => (checkpoint, None),
        Some(value) => (cursor, Some(Value::Number(value))),
    }
}

pub fn parse_boolean<'a, 'b: 'a>(
    cursor: &'b Cursor<'a>,
) -> (&'b Cursor<'a>, Option<Value>) {
    if cursor.starts_with("true") {
        return (&cursor.next(&4), Some(Value::Boolean(true)));
    }

    if cursor.starts_with("false") {
        return (&cursor.next(&5), Some(Value::Boolean(false)));
    }

    return (cursor, None);
}

pub fn parse_null<'a, 'b: 'a>(
    cursor: &'b Cursor<'a>,
) -> (&'b Cursor<'a>, Option<Value>) {
    if cursor.starts_with("null") {
        return (&cursor.next(&4), Some(Value::Null));
    }

    return (cursor, None);
}
