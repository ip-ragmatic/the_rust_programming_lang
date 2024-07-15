Defining a struct: use keyword 'struct' and name the entire struct. It's name should describe the significance of the pieces of
data being grouped together. Then, inside curly brackets, we define the names and types of the pieces of data called fields

```Rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
