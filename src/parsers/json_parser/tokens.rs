use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ValueToken {
    Object(ObjectToken),
    Array(ArrayToken),
    String(StringToken),
    Number(NumberToken),
    Null(NullToken),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FieldToken {
    pub name: String,
    pub value: Box<ValueToken>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ObjectToken {
    pub fields: Vec<FieldToken>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ArrayToken {
    pub elements: Vec<ValueToken>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StringToken {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NumberToken {
    pub value: isize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NullToken;
