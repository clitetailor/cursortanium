use std::iter::IntoIterator;
use std::rc::Rc;
use std::vec::IntoIter;

use crate::cursor::Cursor;

pub struct CaptureResult {
    pub(in crate::test) doc: Rc<String>,
    pub(in crate::test) indices: Vec<(String, usize)>,
}

impl IntoIterator for CaptureResult {
    type Item = Cursor;
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
