## Ownership Inventory #1

### 1. If you tried to compile this function from Program 1, what best describes the compiler error you would get?
```rust
/// PROGRAM 1
/// Makes a string to separate lines of text, 
/// returning a default if the provided string is blank
fn make_separator(user_str: &str) -> &str {
    if user_str == "" {
        let default = "=".repeat(10);
        &default
    } else {
        user_str
    }
}
```
#### Answer: cannot return reference to local variable default
- `default` is a local variable defined within `make_separator` stack and `&default` points to it. An error arises due to trying to return `&default`. Once `make_separator` returns, `default` is freed from memory and so `&default` points to deallocated memory
---
<br/>

### 2. Assume that the compiler doesn't reject Program 1. Which (if any) programs (1) pass the compiler, and (2) possibly cause undefined behavior if executed?

### Normally if you try to compile Program 1, the compiler returns the following error:
```
error[E0515]: cannot return reference to local variable `default`
 --> test.rs:6:9
  |
  |         &default
  |         ^^^^^^^^ returns a reference to data owned by the current function
```
#### Answer: 
```rust
CORRECT ✅

let s = make_separator("");
println!("{s}");
```
- passing an empty string to `make_separator` is what triggers the if-condition, returning a pointer to freed memory (aka a dangling pointer)
---
<br/>

### 3. What fix to Program 1 best satisfies these three criteria: (1) the fixed function passes the Rust compiler, (2) the fixed function preserves the intention of the original code, and (3) the fixed function does not introduce unnecessary inefficiencies
#### Answer:
```rust
CORRECT ✅

fn make_separator(user_str: &str) -> String {
    if user_str == "" {
        let default = "=".repeat(10);
        default
    } else {
        user_str.to_string()        
    }
}
```
- there isn't a way to return a pointer to a stack-allocated variable. So a simple but efficient solution is to turn the return type to a `String` instead. To do so, change `&default` to `default` (an owned string), and convert `user_str` from a `&str` to a `String` using the method `to_string()`. This makes it so the caller doesn't have to input heap-allocated strings and can continue using string literals.
---
<br/>

### 4. If you tried to compile this function from Program 2, what best describes the compiler error you would get?
```rust
/// PROGRAM 2
/// Gets the string out of an option if it exists,
/// returning a default otherwise
fn get_or_default(arg: &Option<String>) -> String {
    if arg.is_none() {
        return String::new();
    }
    let s = arg.unwrap();
    s.clone()
}
```
#### Answer: cannot move out of `arg` in `arg.unwrap()`
- `Option.unwrap()` expects `self`, which means that it's expecting ownership of `arg`. But because `arg` is an immutable reference to an option (`&Option<String>`), it can't provide the required ownership for `unwrap()` to work, and therefore can't retrieve the value contained in option `arg`.

---
<br/>

### 5. Assume that the compiler doesn't reject Program 2. Which (if any) programs (1) pass the compiler, and (2) possibly cause undefined behavior if executed?

### Normally if you try to compile Program 2, the compiler returns the following error:
```
error[E0507]: cannot move out of `*arg` which is behind a shared reference
   --> test.rs:7:13
    |
    |     let s = arg.unwrap();
    |             ^^^^--------
    |             |   |
    |             |   `*arg` moved due to this method call
    |             help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents
    |             move occurs because `*arg` has type `Option<String>`, which does not implement the `Copy` trait
```
#### Answer:
```rust
CORRECT ✅

let opt = Some(String::from("Rust"));
get_or_default(&opt);

let opt = Some(String::from("Rust"));
get_or_default(&opt);
println!("{:?}", opt);

let opt = Some(String::from("Rust"));
let s = get_or_default(&opt);
println!("{}", s);
```
- Each program causes a double-free, violating memory safety. If the compiler allowed `arg.unwrap()` here, then ownership of the string `String::from("Rust")` would be assumed by `s`. After `get_or_default` returns, then the string would be freed. But `opt` also owns the string, so the string would be freed a second time on behalf of opt. Thus, a double-free.
---
<br/>


### 6. What fix to Program 2 best satisfies these three criteria: (1) the fixed function passes the Rust compiler, (2) the fixed function preserves the intention of the original code, and (3) the fixed function does not introduce unnecessary inefficiencies
#### Answer:
```rust
CORRECT ✅

fn get_or_default(arg: &Option<String>) -> String {
    match arg {
        None => String::new(),
        Some(s) => s.clone()
    }
}
```
- A `match` combines the two functionalities between `is_none` and `unwrap()` and automatically deals with pushing the reference `&Option` into `s`, making its type `&String`. So the `match` solution is the most idiomatic, passes the compiler, and preserves the intention of the original code.
    - when you `match` a reference, the outer reference gets pushed into the inner data when there's an arm trying to access that data. In the case of the `Some(s)` arm, `s` is of type `&String` since we're matching an `&Option<String>`. catch-all match on `&Option<String>` --> `Some(&String)`
- The solution of changing &Option to Option is not desirable because it requires the caller to provide ownership of their option, which is a far more restrictive API.