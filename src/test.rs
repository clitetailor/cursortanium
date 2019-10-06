use super::Cursor;

pub struct Test<'a> {
    noLabel: bool,
    prefix: &'a str,
}

impl<'a> Test<'a> {
    pub fn new() -> Test<'a> {
        Test {
            noLabel: true,
            prefix: "🚂",
        }
    }

    pub fn capture(&self, input_str: &str) -> String {
        let mut chunks: Vec<String> = vec![];

        let doc = String::from(input_str);

        let mut cursor = Cursor::from(&doc);
        let mut marker = cursor.mark();

        while !cursor.is_eof() {
            if cursor.starts_with(&self.prefix) {
                chunks.push(marker.take_until(&cursor));
                parse_label(&mut cursor);

                marker = cursor.mark();
            } else {
                cursor.next_mut(1);
            };
        }

        chunks.push(marker.take_until(&cursor));

        chunks.join("")
    }
}

pub fn capture(input_str: &str) -> String {
    Test::new().capture(input_str)
}

fn parse_label(cursor: &mut Cursor) -> String {
    if cursor.starts_with("(") {
        return String::from("");
    };
    cursor.next_mut(1);

    let marker = cursor.mark();

    while !cursor.starts_with(")") && !cursor.is_eof() {
        cursor.next_mut(1);
    }

    let name = marker.take_until(&cursor);

    if !cursor.is_eof() {
        cursor.next_mut(1);
    }

    name
}
