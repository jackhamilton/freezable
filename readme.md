Available via cargo:

```zsh
cargo add freezable-trait
```


Provides a wrapper around serde that eliminates a lot of the boilerplate I usually have to write.

```rust
#[derive(Debug, PartialEq)]
#[freezable] // <- here it is!
struct Example {
    pub field1: String,

    #[default(7)]
    pub field2: i8,

    #[default(true)]
    pub field3: bool,

    pub field4: f32,
}
```

Freezable is an attribute marker (you just stick '#\[freezable\] on top of any struct to get it') which makes that struct a serde object with special per-field default initialization. It also provides some helper functions.


You can annotate fields with \#\[default(<default value here>)\] to set a default, otherwise it'll fall back to Default::default().

Freezable provides the following helper functions as trait members:
```rust
write_to_file(&self, path: PathBuf)
// Serializes the struct and writes it to a new file at the given Path.
```


```rust
write_to_file_str(&self, path: String)
// Serializes the struct and writes it to a new file at the given path string.
// Supports '~' and other relative positioning os-specifics via the shellexpand crate.
```


```rust
from_file(&self, path: PathBuf)
// Deserializes the struct from the file at the given path. If no file exists,
// you get Config::default(). If some fields are missing, present ones are initialized
// from the file and missing ones are initialized based on default values.
```


```rust
from_file_str(&self, path: &str)
// Does the above, but using a shellexpanded str path instead as described in write_to_file_str.
```


```rust
freeze_to_string(&self) -> String
// Returns the struct as a json string.
```


```rust
unfreeze_from_string(from: String)
// Deserializes the type from a given json string.
```

