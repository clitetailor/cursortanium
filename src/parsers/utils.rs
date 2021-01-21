use crate::Cursor;

pub fn parse_string(
    cursor: Cursor,
) -> (Cursor, Option<String>) {
    let checkpoint = cursor;

    let mut cursor = cursor;

    if cursor.starts_with("\"") {
        cursor = cursor.next(&1);
    } else {
        return (checkpoint, None);
    };

    let mut temp_index: usize = cursor.get_index();
    let mut label: String = String::from("");

    while !cursor.starts_with("\"") && !cursor.is_eof() {
        if cursor.starts_with("\\") {
            label.push_str(cursor.read_from(&temp_index));
            cursor = cursor.next(&1);

            if cursor.starts_with("n") {
                label.push_str("\n");
            } else if cursor.starts_with("t") {
                label.push_str("\t");
            } else {
                label.push_str(cursor.lookahead(&1));
            }

            cursor = cursor.next(&1);
            temp_index = cursor.get_index();
        } else {
            cursor = cursor.next(&1);
        };
    }

    label.push_str(cursor.read_from(&temp_index));

    if cursor.starts_with("\"") {
        cursor = cursor.next(&1);
    } else {
        return (checkpoint, None);
    };

    (cursor, Some(label))
}
