> project '5.1' corresponds to these notes

Defining a struct: use keyword 'struct' and name the entire struct. It's name should describe the significance of the pieces of data being grouped together. Then, inside curly brackets, we define the names and types of the pieces of data called fields
```Rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

To create an instance of a struct, we use the `let` keyword followed by the name of the instance, and set that equal to the name of the struct followed by curly braces continaing *key: value* pairs. These pairs are the names of each field, and the value is the data we want to store in those fields.
```Rust
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"), // heap-alloc string
        username: String::from("someusername123"),  // heap-alloc string
        active: true,  // local stack variable
        sign_in_count: 1,  // local stack variable
    };
}
``` 
- to get specific values from the instance, use dot-notation: `user1.username`. Can change the values in each field if the instance is defined to be mutable. Know though that either the entire instance is mutable or immutable, certain fields cannot differ from the whole.
    ```rust
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    fn main() {
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        user1.email = String::from("anotheremail@example.com");
    }
    ```

### Using Field Init Shorthand
Naming function parameters to be the same name as the struct fields makes sense for functions that return an instance, but having to repeat the field names and variables can be annoying, so we can do this:
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

### Creating Instances from Other Instances with Struct Update Syntax
Without struct update syntax:
```Rust
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```
With struct update syntax:
```Rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // this line says that for the remaining fields, use the values of those fields from user1
    };
}
```
- The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
- Notice how struct update syntax uses `=`, this is b/c it moves data. In this example, `user1` has moved ownership to `user2` because of the transfer of `user1`'s `String` data from `username`. `String` types don't implement the `Copy` trait so it is transferred.
    - If we had instead implemented new `username` and `email` data for `user2`, `user1` could still be used after doing a struct update. This is b/c only `active` and `sign_in_count` would have been transfered, which implement the `Copy` trait.

### Using Tuple Structs Without Named Fields to Create Different Types
***Tuple structs*** have the meaning the `struct` keyword provides but only have the data types of their fields. Tuple structs' values can be accessed by way destructuring or by using `.` followed by an index, like regular tuples. They're useful when:
- you want to give a tuple a name but make it different from other tuples
- naming each field in a struct isn't necessary.
    ```rust
    struct RGBColor(u8, u8, u8);
    struct Point(u8, u8, u8);

    fn main() {
        let black = RGBColor(0, 0, 0);
        let RGBColor(r, g, b) = black; // destructuring

        let origin = Point(0, 0, 0);
        let Point(x,y,z) = origin; // destructuring

        println!("The point ({x}, {y}, {z}) is black, which has the RGB-value ({r}, {g}, {b})");
    }
    ```
    `black` and `origin` are instances of different tuple structs. Each tuple struct defined is its own type, regardless of whether or not their fields have the same type. Say function takes a `RGBColor` type, it would be unable to take a `Point` type as an argument.

### Unit-Like Structs Without Any Fields
***Unit-like structs*** are those with no fields. They behave similarly to the unit type `()`. Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. For example
```rust
struct AlwaysEqual;  // no need for () or {} when defining unit-like struct

fn main() {
    let subject = AlwaysEqual;  // no need for () or {} when instantiating unit-like struct
}
```

### Ownership of Struct Data
In the previous `User` struct examples we made it so each instance of `User` owns all of its data by defining each field as an owned type, like `String`. But it's also possible for structs to store borrowed data (immutable or mutable references), which requires the use of *lifetimes*. 

***Lifetimes*** ensure that the data referenced by a struct is valid for as long as the struct is. Storing a reference in a struct without specifying lifetimes will result in a compiler-error; the following code will complain that it needs lifetime specifiers
```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```

### Borrowing Fields of a Struct
Rust's borrow checker tracks O permissions at both the struct-level and field-level.

For example, `p` is an instance of a `Point` structre (w/ fields `x` and `y`). If we borrow a field `x` of a `Point` structure, then both `p` and `p.x` temporarily lose their permissions (but not `p.y`):
```rust
struct Point { x: i32, y: i32 }

let mut p = Point { x: 0, y: 0 };
let x = &mut p.x;  // p and p.x lose RWO permission, x gain RO permission, and *x has RW permission
*x += 1;
println!("{}, {}", p.x, p.y); // after previous line, p and p.x regain RWO permission.
```
now consider a situation where we try to use `p` while `p.x` is mutably borrowed. this will violate the Pointer Safety Principle and we'll get a compiler-error:
```rust
struct Point { x: i32, y: i32 }

fn print_point(p: &Point) {
    println!("{}, {}", p.x, p.y);
}

fn main() {
    let mut p = Point { x: 0, y: 0 };  // (1) p, p.x, and p.y have RWO permission
    let x = &mut p.x;  // (2) p and p.x lose RWO permission, x gains RO permission, *x has RW permission, p.y still has RWO permission
    print_point(&p);  // (3) p cannot be immutably referenced because x is still being mutable borrowed, therefore p has no permission at this point
    *x += 1; // (4) this program fails because p doesn't regain RWO until after this line executes, releasing permissions from x and *x
}
```

---
### Quiz Notes
1. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
```rust
struct Point {
  x: i32,
  y: i32,
}
fn main() {
  let mut a = Point { x: 1, y: 2 };
  a.x += 1;
  let b = Point { y: 1, ..a };
  a.x += 1;
  println!("{}", b.x);
}
```
- the program passes the compiler and prints "2". fields `x` and `y` of `Point` struct implement the `Copy` trait so when instance `b` uses `a` to fill `b.x`, the value of `a.x` is copied into `b.x`. Therefore not removing ownership from instance `a` and allowing `b.x` to printed.
- the second change to `a.x` doesn't affect `b.x` because it was copied, not referenced.

2. Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
```rust
struct Point {
  x: i32,
  y: i32,
}
fn main() {
  let mut p = Point { x: 1, y: 2 };
  let x = &mut p.x;
  let y = &mut p.y;
  *x += 1;
  *y += 1;
  println!("{} {}", p.x, p.y);
}
```
- program passes compiler and prints "2 3". the mutable reference to `p.x` and `p.y` are separate; they don't affect the permissions on the other. so they can both be read or written to simultaneously.