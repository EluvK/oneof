# OneOf

`OneOf` is a procedural macro for Rust that ensures exactly one `Option<_>` field in a struct is `Some`. This is useful for validating structs where only one field should be set at a time.

It is particularly helpful when transitioning from an `enum` to a `struct` and needing a way to enforce that only one field is active at a time.

## Features

- Validates that exactly one `Option<_>` field in a struct is `Some`.
- Provides a `validate_oneof` method to check this condition.
- Provides a `oneof_count` method to count the number of `Option<_>` fields that are `Some`.

## Usage

Add `oneof` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
oneof = { path = "path/to/oneof" }
```

Derive the `OneOf` macro for your struct:

```rust
use oneof::OneOf;

#[derive(OneOf)]
struct MyStruct {
    field1: Option<String>,
    field2: Option<i32>,
    field3: Option<f64>,
}

fn main() {
    let instance = MyStruct {
        field1: Some("Hello".to_string()),
        field2: None,
        field3: None,
    };

    assert!(instance.validate_oneof().is_ok());
}
```

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
