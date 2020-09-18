use regex::{Match, Regex};
use std::rc::Rc;

#[derive(Debug)]
pub struct Cursor {
    doc: Rc<String>,
    index: usize,
    end_index: usize,
}

impl From<&Rc<String>> for Cursor {
    fn from(doc: &Rc<String>) -> Self {
        let doc = doc.clone();
        let end_index = doc.chars().count();

        Cursor {
            doc,
            index: 0,
            end_index,
        }
    }
}

impl Cursor {
    pub fn from_string_at(
        doc: &Rc<String>,
        index: usize,
    ) -> Cursor {
        let doc = doc.clone();
        let end_index = doc.chars().count();

        Cursor {
            doc,
            index,
            end_index,
        }
    }

    pub fn get_doc(&self) -> Rc<String> {
        self.doc.clone()
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

    pub fn set_index(&mut self, index: usize) {
        if index > self.end_index {
            self.index = self.end_index;
        } else {
            self.index = index;
        };
    }

    pub fn next(&mut self, count: usize) {
        self.set_index(self.index + count);
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

    pub fn one_of<'a>(
        &self,
        test_strs: &'a Vec<String>,
    ) -> Option<&'a String> {
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

    pub fn move_to(&mut self, cursor: &Cursor) {
        self.index = if cursor.index < self.end_index {
            cursor.index
        } else {
            self.end_index
        };
    }

    pub fn find(&self, regex: &Regex) -> Option<Match> {
        regex.find_at(&self.doc, self.index)
    }

    pub fn r#match(&self, regex: &Regex) -> Option<Match> {
        self.find(&regex)
            .filter(|mat| mat.start() == self.index)
    }
}

impl Clone for Cursor {
    fn clone(&self) -> Self {
        Cursor {
            doc: self.doc.clone(),
            index: self.index,
            end_index: self.end_index,
        }
    }
}
