## The match Control Flow Construct
`match` is similar to the `if` expression, but instead of the condition needing to evaluate to a boolean (like in `if`), `match` can use any type as its pattern (or condition)
- the `match`'s arms have first a pattern and then some code, separated by the `=>` operator.
- when `match` executes, it compares the resultant value against the pattern of each arm in order
- if returning multiple lines of code in one arm, use `{}` and the comma following the code is optional
    ```rust
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    ```

### Patterns That Bind to Values
match arms can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants. For example:
```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
```
- here the `coin` argument is `Coin::Quarter(UsState::Alaska)`. `match` runs through each pattern to see what `coin` matches, eventually reaching `Coin::Quarter(state)`. The binding for `state` will be the value `UsState::Alaska`. We can then use that binding in the `println!` expression, thus getting the inner state value out of the `Coin` enum variant for `Quarter`

### Matching with `Option<T>`
Recall how the variants of the `Option<T>` enum are `Some(T)` or `None`. We can handle `Option<T>` using `match` when dealing with something that may or may not have a value. For example:
```rust
fn plus_one(int: Option<i32>) -> Option<i32> {
    match int {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let x = Some(41);
    let y = plus_one(x);
    let z = plus_one(None);

    println!("x is {}\ny is {}\nz is {:?}", x.unwrap(), y.unwrap(), z);
}
```
- notice the use of `unwrap()` in the final line of the program. This is another way to extract the value `T` inside `Some(T)`, but also observe how `z` doesn't use `unwrap()`. This is because the program would panic if `unwrap` was called on `z`. Check out [`Option`](https://doc.rust-lang.org/std/option/) documentation.

Combining `match` and enums is useful in many situations. This pattern appears a lot in Rust code: `match` against an `enum`, bind a variable to the data inside, and then execute code based on it

### Matches Are Exhaustive
When using `match`, the arms’ patterns must cover all possibilities. Meaning that matches in Rust are exhaustive: in order for the code to be valid, every possible outcome must be covered by the `match` arms. 
- in the case of `Option<T>`, this means making sure there's a `Some(T)` arm and a `None` arm.

### Catch-all Patterns and the _ Placeholder
Catch-all patterns are pretty much exactly what it sounds like, they catch all possibilities. They're patterns that are used with `match` arms to satisfy the exhaustiveness requirement. They are a hard-coded pattern than can be used in the code following `=>`. For example:
```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),  // b/c of the catch-all pattern 'other', we can use it's value in the code on that arm 
}
```

The `_` placeholder is a catch-all that doesn't let the value inside of it; that is, we can only use it on the left side of the arm. `_` is a special pattern that matches any value whle not binding to that value. For example:
```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),  // not using the value inside '_', so no unused variable error
}
```

### How Matches Interact with Ownership
Using the `_` catch-all in a `match` doesn't assume permission or ownership from the value passed into it since `_` doesn't bind to the value.

But, using a catch-all with a variable name does assume permissions from the value passed into it. This is because if we use this kind of catch-all, it's likely that we want to use the value bound to it, thus changing permissions on data. Of course, this depends on the type of ownership permissions the value being bound has, and how the data is handled. For instance the following doesn't compile:
```rust
fn main() {
let opt: Option<String> = 
    Some(String::from("Hello world"));   // opt: +R+O

match opt {
    // _ became s
    Some(s) => println!("Some: {}", s),  // opt: -R-O, s: +R+O. data moved from opt to s here
    None => println!("None!")
};                                       // opt -> X, s: -R-O. s is freed here

println!("{:?}", opt);                   // error here because opt points to freed memory
}
```
- `opt -> X` is supposed to depict `opt` pointing to freed memory. Freed memory is indicated by `X`
- the way ownership behaves during a `match` expression is no different than with function parameters or variables. The data passed into the `match` being a mutable or immutable reference or being owned, and the type of catch-all being used in the `match`, determines how permissions are transferred.
- if we changed the pattern being matched to an immutable reference, the above code would compile without difficulty