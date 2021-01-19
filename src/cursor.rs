use regex::{Match, Regex};
use std::clone::Clone;

#[derive(Debug)]
pub struct Cursor<'a> {
    doc: &'a str,
    curr_ref: &'a str,
    index: usize,
    end_index: usize,
}

impl<'a> From<&'a str> for Cursor<'a> {
    fn from(doc: &'a str) -> Self {
        let end_index = doc.len();

        Cursor {
            doc,
            curr_ref: doc,
            index: 0,
            end_index,
        }
    }
}

impl<'a> Cursor<'a> {
    pub fn from_string_at(doc: &str, index: usize) -> Cursor {
        let end_index = doc.len();

        Cursor {
            doc,
            curr_ref: &doc[index..],
            index,
            end_index,
        }
    }

    pub fn get_doc(&self) -> &str {
        self.doc
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_end_index(&self) -> usize {
        self.end_index
    }

    pub fn is_at(&self, index: usize) -> bool {
        self.index == index
    }

    pub fn is_eof(&self) -> bool {
        self.index == self.end_index
    }

    /*
    pub fn next<'b: 'a>(&'b self, count: &usize) -> Cursor {
        let remaining = self.end_index - self.index;

        if *count < remaining {
            Cursor {
                index: self.index + count,
                curr_ref: &self.curr_ref[*count..],
                ..*self
            }
        } else {
            Cursor {
                curr_ref: &self.curr_ref[remaining..],
                index: self.end_index,
                ..*self
            }
        }
    }
    */

    pub fn next(&mut self, count: &usize) {
        let remaining = self.end_index - self.index;

        if *count < remaining {
            self.curr_ref = &self.curr_ref[*count..];
            self.index += count;
        } else {
            self.curr_ref = &self.curr_ref[remaining..];
            self.index = self.end_index;
        };
    }

    pub fn starts_with(&self, test_str: &str) -> bool {
        self.curr_ref.starts_with(&test_str)
    }

    pub fn one_of<'b>(
        &self,
        test_strs: &Vec<&'b str>,
    ) -> Option<&'b str> {
        for test_str in test_strs {
            if self.curr_ref.starts_with(test_str) {
                return Some(test_str);
            };
        }

        None
    }

    pub fn lookahead(&self, count: &usize) -> &str {
        let remaining = self.end_index - self.index;

        if remaining < *count {
            self.curr_ref
        } else {
            &self.curr_ref[..*count]
        }
    }

    pub fn read_from(&self, last_index: &usize) -> &str {
        &self.doc[*last_index..self.index]
    }

    pub fn take_until(&self, index: &usize) -> &str {
        self.lookahead(&(*index - self.index))
    }

    pub fn find(&self, regex: &Regex) -> Option<Match> {
        regex.find(&self.curr_ref)
    }

    pub fn r#match(&self, regex: &Regex) -> Option<Match> {
        self.find(&regex)
            .filter(|mat| mat.start() == self.index)
    }
}

impl<'a> Clone for Cursor<'a> {
    fn clone(&self) -> Self {
        Cursor {
            doc: self.doc,
            curr_ref: self.curr_ref,
            index: self.index,
            end_index: self.end_index,
        }
    }
}
