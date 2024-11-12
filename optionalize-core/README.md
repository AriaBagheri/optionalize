
# OptionalizeTrait

`OptionalizeTrait` is a Rust trait designed to generate an "optionalized" version of a struct, where all fields can be wrapped in `Option`. This is useful for situations where only some fields of a struct need to be modified, such as when performing partial updates.

## Features

- Allows for the creation of a new struct with fields wrapped in `Option`.
- Ideal for cases where only certain fields need to be updated or made optional.

## Usage

Define your struct and implement `OptionalizeTrait` for it. Define an associated `Optional` type, where each field is an `Option<T>`.

### Example

Here’s a simple usage example:

```rust
use optionalize::OptionalizeTrait;

struct MyStruct {
    pub id: i32,
    pub name: String,
}

impl OptionalizeTrait for MyStruct {
    type Optional = MyStructOptional;
}

struct MyStructOptional {
    pub id: Option<i32>,
    pub name: Option<String>,
}
```

### Implementing `OptionalizeTrait`

You can implement `OptionalizeTrait` manually, or use a procedural macro (e.g., `Optionalize`) to generate the optionalized struct automatically.

## Development and Testing

Unit tests for `OptionalizeTrait` should verify that the optionalized struct behaves as expected. Below is a sample test case.

### Sample Test Case

```rust
use optionalize::OptionalizeTrait;

struct TestStruct {
    pub id: i32,
    pub name: String,
}

impl OptionalizeTrait for TestStruct {
    type Optional = TestStructOptional;
}

struct TestStructOptional {
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[test]
fn test_optionalize_trait() {
    let original = TestStruct {
        id: 1,
        name: "example".to_string(),
    };
    
    let optionalized = TestStructOptional {
        id: Some(original.id),
        name: Some(original.name),
    };

    assert_eq!(optionalized.id, Some(1));
    assert_eq!(optionalized.name, Some("example".to_string()));
}
```

Run tests with:

```bash
cargo test
```

## License

This project is licensed under the MIT License.
