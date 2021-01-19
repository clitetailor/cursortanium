use crate::cursor::Cursor;
use crate::test::capture_result::CaptureResult;
use crate::utils::parse_label;

pub struct Test {
    pub no_label: bool,
    pub prefix: String,
}

impl Test {
    pub fn new() -> Test {
        Test {
            no_label: true,
            prefix: String::from("🧀"),
        }
    }

    pub fn capture(&self, input: &str) -> CaptureResult {
        let mut doc: String = String::from("");
        let mut indices: Vec<(String, usize)> = vec![];
        let mut offset: usize = 0;

        let mut cursor = Cursor::from(input);
        let mut last_index = cursor.get_index();

        let prefix_len = self.prefix.chars().count();

        while !cursor.is_eof() {
            if cursor.starts_with(&self.prefix) {
                doc.push_str(cursor.read_from(&last_index));
                last_index = cursor.get_index();

                cursor.next(&prefix_len);

                let label = if self.no_label {
                    String::from("")
                } else {
                    parse_label(&mut cursor)
                };
                offset =
                    offset + cursor.get_index() - last_index;

                indices
                    .push((label, cursor.get_index() - offset));

                last_index = cursor.get_index();
            } else {
                cursor.next(&1);
            };
        }

        doc.push_str(cursor.read_from(&last_index));

        CaptureResult { doc, indices }
    }
}
