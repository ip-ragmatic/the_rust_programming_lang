> project 'rectangles' correspnds to these notes

Methods are functions that are defined for specific structs, enums, or trait objects. A method's first parameter is always `self`, which represents the instance of the struct the method is being called on.
- Methods must have a parameter named `self` of type `Self` for their first parameter, so Rust lets you abbreviate this with only the name self in the first parameter spot.

`impl` (implementation) blocks for a specific struct allow us to define methods. Everything within the `impl` block will be associated with the particular struct type (for example, `Rectangle`).

### Defining Methods
Change the `area` function that has a `Rectangle` instance as a parameter and instead make an `area` method defined on the `Rectangle` struct

```rust
struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let r = Rectangle {
        length: 30,
        width: 50
    };

    println!("The area of an {}px × {}px rectangle is {}px²", r.length, r.width, r.area());
}
```
- `&self` is short for `self: &Self`. Within an `impl` block, the `Self` is an alias for the type the `impl` block is for; in this case, `Self` would be an alias for `Rectangle`.
- Note that we use the `&` in front of the `self` shorthand to indicate that this method borrows the `Self` instance.
- Methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably like it could any other parameter.

We can give a method the same name as one of the struct’s fields. For example
```rust
impl Rectangle {
    fn width(&self) -> bool {  // method name same as one of Rectangle's fields
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {     // checks if self.width greater than 0 
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```
- Rust is able to distinguish between between a method with the same name as a field by looking at whether or not there are () following the name.

*Getters* are methods given the same name as a struct's field that only returns that field's value and nothing else.
- Getters are useful for when you want to make a field private, but still provide read-only access to that field. You might do this to provide the read access to the field as part of the type's public API.

### Methods with More Parameters
Make an instance of `Rectangle` take another instance of `Rectangle` and return `true` if the second `Rectangle` can fit completely within the first `Rectangle` (`self`); otherwise, it should return `false`, 
```rust
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // should return true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // should return false
}
```
```
Output:
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

### Associated Functions
All functions defined inside an `impl` block are considered *associated functions* (methods are a subset of associated functions). *Associated functions* don't have `self` as their first parameter because they don't do anything to the instance they're called from
- for example `String::from`, the associated function is `from`

Associated functions that aren’t methods are often used for constructors, which return a new instance of the struct when called. These are often called `new`, but can be called other names as well. And to call associated functions, we use `::` syntax with the struct name.
- `::` syntax is used for both associated functions and namespaces created by modules. 

### Multiple impl Blocks
Structs are allowed to have multiple `impl` blocks, generally there's not really a need to do that. But it's useful occasionally.

### Method Calls are Syntactic Sugar for Function Calls
Say we have the following
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}
```
and that we have a `Rectangle` `r`. The method call and associated function call for `area` and `set_width` are the same, just differently written:
- `r.area() == Rectangle::area(&r)`
- `r.set_width(2) == Rectangle::set_width(&mut r, 2)`

Recall that Rust will insert however many references and dereferences needed to make the types match up for the self parameter. For example, two equivalent calls to `area` for a mutable reference to a boxed rectangle:
```rust
let r = &mut Box::new(Rectangle { 
    width: 1,
    height: 2
});
let area1 = r.area();
let area2 = Rectangle::area(&**r); // mutable reference to r is "downgraded" into an immutable reference
assert_eq!(area1, area2);
```

### Methods and Ownership
Methods, like functions, must be called on structs that have the proper permissions. That is, depending on how the struct instance is defined determines which functions can be called on it; additionally it also depends on any other potential inputs to the method.

#### Reads and Writes with &self and &mut self
Think about Rust's borrow checker. The same underlying principles apply to methods and instances. More specifically, we can't call mutating methods on an immutable or immutably referenced instance.

#### Moves with self
Calling a method that expects `self` as an input will move an input struct (unless the struct implements `Copy`).
- a method automatically accepts `self` as an input, even if it's not between the `()` of the method.

#### Good Moves and Bad Moves
Why does it matter if we move out of `*self`? In fact, for the case of `Rectangle` below, it actually is safe to move out of `*self`, even though Rust doesn't let you do it
```rust
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {    
  fn max(self, other: Self) -> Self {
    let w = self.width.max(other.width);
    let h = self.height.max(other.height);
    Rectangle { 
      width: w,
      height: h
    }
  }
    fn set_to_max(&mut self, other: Rectangle) {
        let max = self.max(other);
        *self = max;
    }
}

fn main() {
    let mut rect = Rectangle { width: 0, height: 1 };
    let other_rect = Rectangle { width: 1, height: 0 };
    rect.set_to_max(other_rect);
}
```
It's safe to move out of `*self` because `Rectangle` doesn't own any heap data. We can get Rust to compile `set_to_max` by adding `#[derive(Copy, Clone)]` to the definition of `Rectangle`, like so
```rust
#[derive(Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {    
  fn max(self, other: Self) -> Self {
    let w = self.width.max(other.width);
    let h = self.height.max(other.height);
    Rectangle { 
      width: w,
      height: h
    }
  }
    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}
```

Now consider the following:
```rust
struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {    
  fn max(self, other: Self) -> Self {
    let w = self.width.max(other.width);
    let h = self.height.max(other.height);
    Rectangle { 
      width: w,
      height: h,
      name: String::from("max")
    }
  }
    fn set_to_max(&mut self, other: Rectangle) {
        let max = self.max(other);
        drop(*self); // This is usually implicitly. double free occurs here b/c of name field
                     // added here for clarity.
        *self = max;
    }
}

fn main() {
    let mut r1 = Rectangle { 
        width: 9, 
        height: 9, 
        name: String::from("r1") 
    };
    let r2 = Rectangle {
        width: 16,
        height: 16,
        name: String::from("r2")
    };
    r1.set_to_max(r2);
}
```
the `name` field of any instance of `Rectangle` is an owned heap-allocated string, so when `set_to_max` is called, `self` and `other` are deallocated due to the `max` method consuming their ownership once it returns. Then in the following line `*self` has already been deallocated, resulting in a double-free.

When you see an error like "cannot move out of `*self`", that's usually because you're trying to call a `self` method on a reference like `&self` or `&mut self`. Rust is protecting you from a double-free.