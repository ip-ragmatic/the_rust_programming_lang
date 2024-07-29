## Defining an Enum
enums give you a way of saying a value is one of a possible set of values
- For example, we may want to say that `Rectangle` is one of a set of possible shapes that also includes `Circle` and `Triangle`. To do this, Rust allows us to encode these possibilities as an enum.

Currently, two major standards are used for IP addresses: version four and version six. Any IP address can be either a version four or a version six address, but not both at the same time. That property of IP addresses makes the enum data structure appropriate because an enum value can only be one of its variants. Both version four and version six addresses are still fundamentally IP addresses, so they should be treated as the same type when the code is handling situations that apply to any kind of IP address. 
```rust
enum IpAddrKind {
    V4,
    V6,
}
```
`IpAddrKind` is now a custom data type that can be implemented elswhere.

### Enum Values
We can create instances of each of the two variants of `IpAddrKind` like this:
```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```
- variants of the enum are namespaced under its identifier.

We can place enums inside of structs' fields:
```rust
fn main() {
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"), // heap-alloc string
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"), // heap-alloc string
};
}
```
or we can do the above more concisely using just enums, by expecting what the values of the enum variants will be:
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1")); // heap-alloc string

let loopback = IpAddr::V6(String::from("::1")); // heap-alloc string
```
- Now the variants of `IpAddr` can act as a constructor function.

An advantage to using an enum over a struct is that each variant can have different types and amounts of data:. v4 IP addresses will always have four numeric components that'll be between 0 and 255; we can represent this using an enum:
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1); // main() stack-frame local variable. home -> V4 [127|0|0|1]

let loopback = IpAddr::V6("::1"); // heap-alloc string
```

We can also define methods for enums just like we can for structs:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

### The Option Enum and Its Advantages Over Null Values
The `Option` type (an enum in Rust) encodes the very common scenario in which a value could be something or it could be nothing.
```rust
enum Option<T> {
    None,
    Some(T),
}
```
- `Option<T>` enum is a regular enum, and `Some(T)` and `None` are variants of type `Option<T>`
- `<T>` syntax is a generic type parameter. `<T>` means that the `Some` variant of the `Option` enum can hold a piece of data that's of any type
- types `Option<T>` and `T` are entirely different types
- if we are to assign `None` to a variable, we must type annotate the overall `Option` type. Rust can't infer the type of a `Some` variant looking only at `None`
- the compiler won’t let us use an `Option<T>` value as if it were *valid* value. For example:
    ```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
    ```
    ```
    $ cargo run
       Compiling enums v0.1.0 (file:///projects/enums)
    error[E0277]: cannot add `Option<i8>` to `i8`
     --> src/main.rs:5:17
      |
    5 |     let sum = x + y;
      |                 ^ no implementation for `i8 + Option<i8>`
      |
      = help: the trait `Add<Option<i8>>` is not implemented for `i8`
      = help: the following other types implement trait `Add<Rhs>`:
                <&'a i8 as Add<i8>>
                <&i8 as Add<&i8>>
                <i8 as Add<&i8>>
                <i8 as Add>

    For more information about this error, try `rustc --explain E0277`.
    error: could not compile `enums` due to previous error
    ```

To perform operations on `T` when it's inside an `Option<T>`, we have to convert `Option<T>` to `T`. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is
- In general, in order to use an `Option<T>` value, there needs to be code that can handle each possible variant. That is, have specific code run when there's a value `Some(T)` that has access to the inner `T`, and separate code that runs when there's a `None` value, which doesn't have access to `T`.
- recall the `match` expression, it does exactly this when used in combination with enums
