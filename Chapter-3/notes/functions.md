***functions*** can be defined outside the main() function, that is in the global scope (kind of like constants).
- functions defined using `fn` keyword
- functions defined outside of `main()` can still be called from within `main()`.
- when defining functions that utilize parameters, you must type annotate each one.
    ```rust
    fn f(x: i32, y: i32, z: i32) {
       let sum = x + y + z;
       println!("x + y + z = {sum}");
    }

    fn main() {
       f(1, 3, 5);    // prints 'x + y + z = 9'
    }
    ```

***Statements*** are instructions that perform some action and do not return a value.
- the key characteristic of a statement is that it doesn't produce a value that can be used in another expression.
    ```rust
    fn main() {
        let y = 27; // line is a statement; doesn't return a value, just states y is equal to 27
    } // function defintions are statements themselves
    ```

***Expressions*** evaluate to a resultant value. That is, they return a value.
- expressions can be part of statements
- Expressions don't include ending semicolons.
    - Adding a semicolon to the end of an expression turns it into a statement and it won't return a value; instead, it becomes a `()` (unit type).
- examples of expressions include calling a function, calling a macro, or a new scope block between curly braces.
    ```rust
    fn main() {
        let y = {
            let x = 3;
            x + 1
        }; // inner scope of y an expression b/c it returns a value of 4

        println!("The value of y is: {y}");
    }
    ``` 

***Functions with return values***
- most functions return the last expression implicitly (w/out ;).
- to have a function w/ a return value, we must declare the returned value's type using `-> {type}`
    ```rust
    fn five() -> i32 { // notice "-> i32", this line declares the output type. 
    5 // this line implicitly reutrns 5. in python this would be "return 5". 
    }

    fn main() {
    let x = five(); // assigns 5 to x because five() yields 5.

    println!("The value of x is: {x}");
    }    

    fn main() {
    let x = plus_one(5); // calls plus_one with 5 as an argument

    println!("The value of x is: {x}");
    }

    fn plus_one(x: i32) -> i32 { // defines parameter and it's type, and what the output's type will be
    x + 1 // if ; added to line end, then this code would produce a "mismatched types" error
    }
    ```