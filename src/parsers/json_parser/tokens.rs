use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Value {
    Object(Vec<Field>),
    Array(Vec<Value>),
    String(String),
    Number(String),
    Null,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Field(pub String, pub Box<Value>);
