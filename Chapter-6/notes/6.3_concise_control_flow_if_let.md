## Concise Control Flow with if let
The `if let` syntax combines `if` and `let` into a less verbose way that lets us handle values that match one pattern while ignoring the rest. This means that you lose the exhaustive checking that `match` enforces. Choosing between `match` and `if let` depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.
- `if let` is syntactic sugar for a `match` that runs code when the value matches one pattern and then ignores all other values

We can also include an `else` with an `if let`. The block of code that goes with the `else` is like the block of code that would go with the `_` case in the `match` expression.

Example without `if let`:
```rust
fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,  // this arm executes
    }
}
```

Example with `if let` and `else`:
```rust
fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {                // if let <pattern> = var
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
```