use crate::cursor::Cursor;

pub(in crate) fn parse_label(cursor: &mut Cursor) -> String {
    if cursor.starts_with("(") {
        return String::from("");
    };
    cursor.next(1);

    let marker = cursor.clone();

    while !cursor.starts_with(")") && !cursor.is_eof() {
        cursor.next(1);
    }

    let name = marker.take_until(&cursor);

    if !cursor.is_eof() {
        cursor.next(1);
    }

    name
}
