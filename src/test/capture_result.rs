use std::iter::IntoIterator;
use std::vec::IntoIter;

use crate::cursor::Cursor;

pub struct CaptureResult {
    pub(in crate::test) doc: String,
    pub(in crate::test) indices: Vec<(String, usize)>,
}

impl<'a> IntoIterator for &'a CaptureResult {
    type Item = Cursor<'a>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.indices
            .iter()
            .map(|index| {
                Cursor::from_string_at(&self.doc, index.1)
            })
            .collect::<Vec<Cursor>>()
            .into_iter()
    }
}
