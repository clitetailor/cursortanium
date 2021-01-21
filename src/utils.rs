use crate::cursor::Cursor;

pub(in crate) fn parse_label<'a, 'b: 'a>(
    cursor: &'b Cursor<'a>,
) -> (&'b Cursor<'a>, String) {
    if cursor.starts_with("(") {
        return (&cursor, String::from(""));
    };

    let mut cursor = &cursor.next(&1);

    let last_index = cursor.get_index();

    while !cursor.starts_with(")") && !cursor.is_eof() {
        cursor = &cursor.next(&1);
    }

    let name = cursor.read_from(&last_index).into();

    if !cursor.is_eof() {
        cursor = &cursor.next(&1);
    }

    (&cursor, name)
}
