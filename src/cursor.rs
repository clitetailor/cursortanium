use regex::{Match, Regex};

#[derive(Debug)]
pub struct Cursor<'a> {
    doc: &'a str,
    index: usize,
    end_index: usize,
}

impl<'a> From<&'a str> for Cursor<'a> {
    fn from(doc: &'a str) -> Self {
        let end_index = doc.len();

        Cursor {
            doc,
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

    pub fn next(&mut self, count: &usize) {
        self.move_to(&(self.index + *count));
    }

    pub fn starts_with(&self, test_str: &str) -> bool {
        let end_index = self.index + test_str.len();

        let end_index = if end_index > self.end_index {
            self.end_index
        } else {
            end_index
        };

        self.doc[self.index..end_index] == *test_str
    }

    pub fn one_of<'b>(
        &self,
        test_strs: &Vec<&'b str>,
    ) -> Option<&'b str> {
        for test_str in test_strs {
            if self.starts_with(test_str) {
                return Some(test_str);
            };
        }

        None
    }

    pub fn lookahead(&self, count: &usize) -> &str {
        let end_index = self.index + count;

        let end_index = if end_index > self.end_index {
            self.end_index
        } else {
            end_index
        };

        &self.doc[self.index..end_index]
    }

    pub fn read_from(&self, last_index: &usize) -> &str {
        &self.doc[*last_index..self.index]
    }

    pub fn move_to(&mut self, last_index: &usize) {
        self.index = if *last_index < self.end_index {
            *last_index
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
