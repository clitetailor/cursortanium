use cursortanium::Cursor;

use super::tokens::Token;

pub fn parse(cursor: &mut Cursor) -> Option<Token> {
    parse_string(&mut *cursor)
}

pub fn parse_string(cursor: &mut Cursor) -> Option<Token> {
    let checkpoint = cursor.clone();

    if cursor.starts_with("\"") {
        cursor.next(1);
    } else {
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

    Some(Token::String {
        value: chunks.join(""),
    })
}
