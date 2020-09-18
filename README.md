# cursortanium

A solid cursor for Rust

## Why cursortanium?

While regexp and PEGs are declarative and elegant, it can be hard control when coming to much more complex problems.

cursortanium is a general purpose parser. It provides imperative and programmatic interface to interact with the input document. Which means that you can customize and run the parser your own way without relying on any additional syntax or grammar. This makes cursortanium much more powerful in comparison with prior libraries.

## Example

The following function takes an input cursor then extracts the string value, which may contain escape sequences, from the document and returns the corresponding value:

```rust
fn parse_string(cursor: &mut Cursor) -> Option<String> {
    if cursor.starts_with("\"") {
        cursor.next_mut(1);
    } else {
        return None;
    };

    let mut chunks = vec![];
    let mut marker = cursor.clone();

    while !cursor.starts_with("\"") && !cursor.is_eof() {
        if cursor.starts_with("\\") {
            chunks.push(marker.take_until(&cursor));
            cursor.next_mut(1);
            marker.move_to_mut(&cursor)
            cursor.next_mut(1);
        } else {
            cursor.next_mut(1);
        };
    };

    chunks.push(marker.take_until(&cursor));

    if cursor.starts_with("\"") {
        cursor.next_mut(1);
    } else {
        return None;
    };

    Some(chunks.concat())
}
```

To use this function, we will create a cursor at the start of the string and passing it as mutable parameter:

```rust
use cursortanium::Cursor;

fn main() {
    let mut cursor = Cursor::from(String::from(r#""Hello, \"World\"!"#));

    if let Some(value) = parse_string(&mut cursor) {
        println!("{}", cursor.is_eof())
        println!("{}", value);
    };
}
```

Run the above code with `cargo run` will give the following output:

```bash
$ cargo run
true
Hello, "World"!
```

## License

[MIT License](LICENSE)
