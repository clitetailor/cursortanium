use std::rc::Rc;

use crate::cursor::Cursor;
use crate::test::capture_result::CaptureResult;
use crate::utils::parse_label;

pub struct Test {
    pub no_label: bool,
    pub prefix: Rc<String>,
}

impl Test {
    pub fn new() -> Test {
        Test {
            no_label: true,
            prefix: Rc::new(String::from("🧀")),
        }
    }

    pub fn capture(&self, input: &Rc<String>) -> CaptureResult {
        let mut chunks: Vec<String> = vec![];
        let mut indices: Vec<(String, usize)> = vec![];
        let mut offset: usize = 0;

        let mut cursor = Cursor::from(input);
        let mut marker = cursor.clone();

        let prefix_len = self.prefix.chars().count();

        while !cursor.is_eof() {
            if cursor.starts_with(&self.prefix) {
                chunks.push(marker.take_until(&cursor));
                marker = cursor.clone();

                cursor.next(prefix_len);

                let label = if self.no_label {
                    String::from("")
                } else {
                    parse_label(&mut cursor)
                };
                offset = offset + cursor.get_index()
                    - marker.get_index();

                indices
                    .push((label, cursor.get_index() - offset));
                marker = cursor.clone();
            } else {
                cursor.next(1);
            };
        }

        chunks.push(marker.take_until(&cursor));

        let doc = chunks.concat().into();

        CaptureResult { doc, indices }
    }
}
