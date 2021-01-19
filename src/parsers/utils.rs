use crate::Cursor;

pub fn parse_string(cursor: &mut Cursor) -> Option<String> {
    let last_pos = cursor.clone();

    if cursor.starts_with("\"") {
        cursor.next(&1);
    } else {
        *cursor = last_pos;

        return None;
    };

    let mut temp_index: usize = cursor.get_index();
    let mut label: String = String::from("");

    while !cursor.starts_with("\"") && !cursor.is_eof() {
        if cursor.starts_with("\\") {
            label.push_str(cursor.read_from(&temp_index));
            cursor.next(&1);

            if cursor.starts_with("n") {
                label.push_str("\n");
            } else if cursor.starts_with("t") {
                label.push_str("\t");
            } else {
                label.push_str(cursor.lookahead(&1));
            }

            cursor.next(&1);
            temp_index = cursor.get_index();

            cursor.lookahead(&10);
        } else {
            cursor.next(&1);
        };
    }

    label.push_str(cursor.read_from(&temp_index));

    if cursor.starts_with("\"") {
        cursor.next(&1);
    } else {
        *cursor = last_pos;

        return None;
    };

    Some(label)
}
