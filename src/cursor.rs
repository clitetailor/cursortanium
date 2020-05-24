use std::rc::Rc;

#[derive(Debug)]
pub struct Cursor {
    doc: Rc<str>,
    index: usize,
    end_index: usize,
}

impl<'a, T> From<T> for Cursor
where
    T: AsRef<str>,
{
    fn from(doc: T) -> Self {
        let doc = doc.as_ref();
        let end_index = doc.len();

        Cursor {
            doc: Rc::from(doc),
            index: 0,
            end_index,
        }
    }
}

impl Cursor {
    pub fn from_string_at<T: AsRef<str>>(
        doc: T,
        index: usize,
    ) -> Cursor {
        let doc = doc.as_ref();
        let end_index = doc.len();

        Cursor {
            doc: Rc::from(doc),
            index,
            end_index,
        }
    }

    pub fn get_char(&self) -> Option<char> {
        self.doc[self.index..].chars().next()
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
        let count = test_str.len();

        &self.doc[start..(start + count)] == test_str
    }

    pub fn one_of<'b>(
        &self,
        test_strs: &'b [&str],
    ) -> Option<&'b str> {
        for test_str in test_strs {
            if self.starts_with(test_str) {
                return Some(test_str);
            };
        }

        None
    }

    pub fn lookahead(&self, count: usize) -> String {
        let start = self.index;

        self.doc[start..(start + count)].to_owned()
    }

    pub fn take_until(&self, cursor: &Cursor) -> String {
        let start = self.index;
        let count = cursor.get_index() - start;

        self.doc[start..(start + count)].to_owned()
    }

    pub fn move_to(&mut self, cursor: &Cursor) {
        self.index = if cursor.index < self.end_index {
            cursor.index
        } else {
            self.end_index
        };
    }
}

impl<'a> Clone for Cursor {
    fn clone(&self) -> Self {
        Cursor {
            doc: Rc::clone(&self.doc),
            index: self.index,
            end_index: self.end_index,
        }
    }
}
