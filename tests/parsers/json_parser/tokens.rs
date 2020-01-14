#[derive(PartialEq, Debug)]
pub struct Field {
    name: String,
    value: Box<Token>,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Object { fields: Vec<Box<Field>> },
    Array { elements: Vec<Token> },
    String { value: String },
    Number { value: isize },
    Null,
}
