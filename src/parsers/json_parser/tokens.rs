use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Value {
    Object(Vec<(String, Box<Value>)>),
    Array(Vec<Value>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}
