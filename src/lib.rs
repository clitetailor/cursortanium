mod helpers;
pub mod test;

use std::borrow::Cow;

pub struct Cursor<'a> {
    doc: Cow<'a, str>,
    index: usize,
    end_index: usize,
}

impl<'a> From<String> for Cursor<'a> {
    fn from(doc: String) -> Self {
        let end_index = doc.chars().count();

        Cursor {
            doc: doc.into(),
            index: 0,
            end_index,
        }
    }
}

impl<'a> From<&'a String> for Cursor<'a> {
    fn from(doc: &'a String) -> Self {
        let end_index = doc.chars().count();

        Cursor {
            doc: doc.into(),
            index: 0,
            end_index,
        }
    }
}

impl<'a> From<Cow<'a, str>> for Cursor<'a> {
    fn from(doc: Cow<'a, str>) -> Self {
        let end_index = doc.chars().count();

        Cursor {
            doc,
            index: 0,
            end_index,
        }
    }
}

impl<'a> Cursor<'a> {
    pub fn from_string_at(
        doc: Cow<'a, str>,
        index: usize,
    ) -> Cursor<'a> {
        let end_index = doc.chars().count();

        Cursor {
            doc,
            index,
            end_index,
        }
    }

    pub fn get_doc(&self) -> &Cow<'a, str> {
        &self.doc
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_end_index(&self) -> usize {
        self.end_index
    }

    pub fn is_at(&self, cursor: &Cursor) -> bool {
        self.index == cursor.index
    }

    pub fn is_eof(&self) -> bool {
        self.index == self.end_index
    }

    pub fn set_index_mut(&mut self, index: usize) {
        if index > self.end_index {
            self.index = self.end_index;
        } else {
            self.index = index;
        };
    }

    pub fn next_mut(&mut self, count: usize) {
        self.set_index_mut(self.index + count);
    }

    pub fn starts_with(&self, test_str: &str) -> bool {
        let start = self.index;
        let count = test_str.chars().count();

        self.doc
            .chars()
            .skip(start)
            .take(count)
            .collect::<String>()
            == test_str
    }

    pub fn one_of<'b>(
        &self,
        test_strs: &'b Vec<String>,
    ) -> Option<&'b String> {
        for test_str in test_strs {
            if self.starts_with(test_str) {
                return Some(test_str);
            };
        }

        return None;
    }

    pub fn lookahead(&self, count: usize) -> String {
        let start = self.index;

        self.doc.chars().skip(start).take(count).collect()
    }

    pub fn take_until(&self, cursor: &Cursor) -> String {
        let start = self.index;
        let count = cursor.get_index() - start;

        self.doc.chars().skip(start).take(count).collect()
    }

    pub fn move_to_mut(&mut self, cursor: &Cursor) {
        self.index = if cursor.index < self.end_index {
            cursor.index
        } else {
            self.end_index
        };
    }
}

impl<'a> Clone for Cursor<'a> {
    fn clone(&self) -> Self {
        Cursor {
            doc: self.doc.to_owned(),
            index: self.index,
            end_index: self.end_index,
        }
    }
}
