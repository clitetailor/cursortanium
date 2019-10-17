use super::helpers::parse_label;
use super::Cursor;
use std::borrow::Cow;
use std::iter::IntoIterator;
use std::vec::IntoIter;

pub struct Test<'a> {
    pub no_label: bool,
    pub prefix: &'a str,
}

impl<'a> Test<'a> {
    pub fn new() -> Test<'a> {
        Test {
            no_label: true,
            prefix: "🌖",
        }
    }

    pub fn capture<'b>(
        &self,
        input: Cow<'b, str>,
    ) -> CaptureResult<'b> {
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

                cursor.next_mut(prefix_len);

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
                cursor.next_mut(1);
            };
        }

        chunks.push(marker.take_until(&cursor));

        let doc = chunks.concat();

        CaptureResult {
            doc: doc.into(),
            indices,
        }
    }
}

pub struct CaptureResult<'a> {
    doc: Cow<'a, str>,
    indices: Vec<(String, usize)>,
}

impl<'a> CaptureResult<'a> {
    pub fn iter(&self) -> IntoIter<Cursor<'a>> {
        self.indices
            .iter()
            .map(|index| {
                Cursor::from_string_at(
                    self.doc.clone(),
                    index.1,
                )
            })
            .collect::<Vec<Cursor>>()
            .into_iter()
    }
}

impl<'a> IntoIterator for CaptureResult<'a> {
    type Item = Cursor<'a>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.indices
            .iter()
            .map(|index| {
                Cursor::from_string_at(
                    self.doc.clone(),
                    index.1,
                )
            })
            .collect::<Vec<Cursor>>()
            .into_iter()
    }
}

pub fn capture<'a>(input: Cow<'a, str>) -> CaptureResult {
    Test::new().capture(input)
}
