use cursortanium::{parsers::json_parser, Cursor};

const DATA: &str = include_str!("../assets/canada.json");

fn main() {
    let _ast = json_parser::parse(&mut Cursor::from(DATA));
}
