# Optionalize

`optionalize` is a procedural macro crate for Rust that generates a new struct with all fields wrapped in `Option`. If a field is already an `Option<T>`, it remains as `Option<T>`; otherwise, it’s wrapped as `Option<T>`. This is useful for creating structs for partial updates, where only some fields may need to be modified.

## Features

- Automatically generates an "optionalized" version of a struct.
- Retains `Option` types if they are already present in the original struct.
- Can be useful for partial updates or optional struct fields.

## Installation

Add `optionalize` to your `Cargo.toml`:

```toml
[dependencies]
optionalize = { git = "https://github.com/AriaBagheri/optionalize.git" }
```

> **Note:** Ensure this path points to where `optionalize` is located in your project. You can adjust it based on your directory structure.

## Usage

To use the `Optionalize` macro, simply derive it on your struct. A new struct will be generated with the same name, appended with `Optional`, where all fields are wrapped in `Option`.

### Example

Here’s a basic example demonstrating how to use `Optionalize`.

```rust
use optionalize::Optionalize;

#[derive(Optionalize)]
pub struct MyStruct {
    #[optionalize_ignore]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

// The generated struct will be:
// pub struct MyStructOptional {
//     pub id: i32,
//     pub name: Option<String>,
//     pub description: Option<String>,
// }
```

### Example Usage in Code

With `Optionalize`, you can create an "optionalized" struct for scenarios where only certain fields are updated:

```rust
use optionalize::Optionalize;

#[derive(Optionalize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
}

fn main() {
    // Original struct
    let user = User {
        id: 1,
        username: "user123".to_string(),
        email: Some("user@example.com".to_string()),
    };

    // Partial update with optional fields
    let user_update = UserOptional {
        id: None,  // We don't want to update `id`
        username: Some("new_user123".to_string()),  // Update `username`
        email: None,  // No change to `email`
    };

    // Now, `user_update` can be used for partial updates where only certain fields are modified
}
```

## How It Works

The `Optionalize` macro inspects each field in your struct:

- If the field is already an `Option<T>`, it keeps it as `Option<T>`.
- If the field is of type `T`, it wraps it as `Option<T>`.

This allows for flexible use cases where you only want to update a subset of fields in your struct without needing to specify every field explicitly.

## Limitations

- The `Optionalize` macro only works with structs and does not support enums.
- It requires the `syn` and `quote` crates for parsing and generating Rust code.

## License

This project is licensed under the MIT License.
