use cursortanium::Cursor;

pub fn parse_string(cursor: &mut Cursor) -> Option<String> {
    let checkpoint = cursor.clone();

    if cursor.starts_with("\"") {
        cursor.next(1);
    } else {
        cursor.move_to(&checkpoint);

        return None;
    };

    let mut marker = cursor.clone();
    let mut chunks: Vec<String> = vec![];

    marker.move_to(&cursor);

    while !cursor.starts_with("\"") && !cursor.is_eof() {
        if cursor.starts_with("\\") {
            chunks.push(marker.take_until(&cursor));
            cursor.next(1);

            if cursor.starts_with("n") {
                chunks.push(String::from("\n"));
            } else if cursor.starts_with("t") {
                chunks.push(String::from("\t"));
            } else {
                chunks.push(cursor.lookahead(1));
            }

            cursor.next(1);
            marker.move_to(&cursor);

            cursor.lookahead(10);
        } else {
            cursor.next(1);
        };
    }

    chunks.push(marker.take_until(&cursor));

    if cursor.starts_with("\"") {
        cursor.next(1);
    } else {
        cursor.move_to(&checkpoint);

        return None;
    };

    Some(chunks.join(""))
}
