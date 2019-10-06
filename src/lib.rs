pub mod test;

pub struct Cursor<'a> {
    doc: &'a String,
    index: usize,
    end_index: usize,
}

impl<'a> Cursor<'a> {
    pub fn from(doc: &'a String) -> Cursor<'a> {
        Cursor {
            doc,
            index: 0,
            end_index: doc.len(),
        }
    }

    pub fn get_doc(&self) -> &'a String {
        self.doc
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
        let assumed_end = start + test_str.len();

        let end = if assumed_end < self.end_index {
            assumed_end
        } else {
            self.end_index
        };

        self.doc[start..end] == *test_str
    }

    pub fn lookahead(&mut self, count: usize) -> String {
        let start = self.index;
        let end = self.index + count;

        self.doc[start..end].to_owned()
    }

    pub fn mark(&mut self) -> Cursor<'a> {
        Cursor {
            doc: &self.doc,
            index: self.index,
            end_index: self.end_index,
        }
    }

    pub fn take_until(&self, cursor: &Cursor) -> String {
        let start = self.index;
        let end = cursor.get_index();

        self.doc[start..end].to_owned()
    }
}
