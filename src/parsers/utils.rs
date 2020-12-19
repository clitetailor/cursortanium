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
    let mut chunks: Vec<String> = vec![];

    while !cursor.starts_with("\"") && !cursor.is_eof() {
        if cursor.starts_with("\\") {
            chunks.push(cursor.read_from(&temp_index).into());
            cursor.next(&1);

            if cursor.starts_with("n") {
                chunks.push(String::from("\n"));
            } else if cursor.starts_with("t") {
                chunks.push(String::from("\t"));
            } else {
                chunks.push(cursor.lookahead(&1).into());
            }

            cursor.next(&1);
            temp_index = cursor.get_index();

            cursor.lookahead(&10);
        } else {
            cursor.next(&1);
        };
    }

    chunks.push(cursor.read_from(&temp_index).into());

    if cursor.starts_with("\"") {
        cursor.next(&1);
    } else {
        *cursor = last_pos;

        return None;
    };

    Some(chunks.join(""))
}
