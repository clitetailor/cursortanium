use crate::cursor::Cursor;

pub(in crate) fn parse_label(cursor: &mut Cursor) -> String {
    if cursor.starts_with("(") {
        return String::from("");
    };
    cursor.next(&1);

    let last_index = cursor.get_index();

    while !cursor.starts_with(")") && !cursor.is_eof() {
        cursor.next(&1);
    }

    let name = cursor.read_from(&last_index).into();

    if !cursor.is_eof() {
        cursor.next(&1);
    }

    name
}
