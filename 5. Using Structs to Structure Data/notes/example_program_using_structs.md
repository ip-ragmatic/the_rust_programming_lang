> project 'rectangles' correspnds to these notes

To know when to use structs, let's write a program that calculates the area of a rectangle using it's length and width. We’ll start by using single variables, and then refactor the program until we’re using structs instead.

- will take the width and height of a rectangle in units pixels and then calculates the area of the rectangle

### Rectangle Area w/ Single Variables

```rust
fn main() {
    let l: u32 = 30;
    let w: u32 = 50;

    println!(
        "The area of a {}px × {}px rectangle is {}px²",
        l,
        w,
        area(&l, &w)
    )
}

fn area(length: &u32, width: &u32) -> u32 {
    length * width
}
```
The `area` function calculates the area of *one rectangle*, but our function has *two parameters*. It’s not necessarily clear anywhere in our program that the two parameters are related. It would make more sense to group length and width together so as to make it more obvious that we are using them to describe one object.

### Refactoring w/ Tuples
```rust
fn main() {
    let rect = (30, 50);

    println!("The area of a {}px × {}px rectangle is {}px²", rect.0, rect.1, area(&rect));
}

fn area(dim: &(u32, u32)) -> u32 {
    dim.0 * dim.1
}
```
This is better in way. There is a bit more structure due to `rect` and the `area` function only has *one parameter*, which is more intuitive for the `area` function.

But this refactor is also worse because now it becomes less clear which number describes what in the tuple; there is no label for length and width. In the case of our program it doesn't exactly matter because the result of the `area` will be the same regardless of what position the numbers are in (l × w = w × l). But what if we were drawing a rectangle? The order of the tuple's data would certainly matter b/c it could cause the rectangle to have the wrong side lengths when looking at it.

### Refactoring w/ Structs: Adding More Meaning
In comes structs. Using structs allows us to continue only putting one parameter for the `area` function without compromising the clarity of what each piece of data represents.
```rust
struct Rectangle {
    length: u32,
    width: u32
}

fn main() {
    let r = Rectangle {
        length: 30,
        width: 50
    };

    println!("The area of an {}px × {}px rectangle is {}px²", r.length, r.width, area(&r));
}

fn area(rect: &Rectangle) -> u32 {
    rect.length * rect.width
}
```
Now the program does exactly what we want it to do: calculate the area of a rectangle using its length and width. Using structs allowed us to accomplish this task in an organized and clear manner (in the sense that what's happening is clear).
- Notice in each refactorization the paramter(s) of the `area` function. They all utilize immutable borrows. This is so `main` retains ownership of rectangle related data.

### Adding Useful Functionality with Derived Traits
With structs, the way `println!` should format the output isn't clear because there are many display possibilities. To be able to print an instance of a struct, use `{:?}` or `{:#?}` (pretty print) when formatting inside `println!`, and then add `#[derive(Debug)]` above the `struct` definition.
```rust
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}

fn main() {
    let r = Rectangle {
        length: 30,
        width: 50
    };

    println!("rectangle is {:#?}", r);
}
```
```
Output:
rectangle is Rectangle {
    length: 30,
    width: 50,
}
```


We can also use the `dbg!` macro, which takes ownership of an expression (`println!` takes a reference), prints the file and line number of where that `dbg!` macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value. The output uses the pretty `Debug` formatting of the `Rectangle` type
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```
```
Output:
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```