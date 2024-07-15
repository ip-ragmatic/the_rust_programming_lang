Compound types are able to group multiple values into one compound type.

***Tuples*** are compound types with a fixed-length defined at inception, and are able to store different types of values
- like variables, you can add `mut` to make it mutable. but only the values of the tuple are mutable
- can access a tuple's value by destructuring it or by using `.` followed by index of the value in tuple:
    1. Destructuring:
        ```Rust
        fn main() {
            let tup = (500, 6.4, 1);
            let (x, y, z) = tup;
            println!("The value of y is: {y}");
        }
        ```
- A Unit in Rust is an empty tuple. A unit's value and type are both written as `()`, which represents an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.
      
***Arrays*** group a fixed number of values that must all have the same type.
- useful when you want your data allocated on the stack rather than the heap
- type annotate array using square brackets with the type of each element, a semicolon, and then number of elements in array
    ```rust
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0] // similar to python list access, yields the value 1
    let a = [3; 5]; // a = [3, 3, 3, 3, 3]
    ```

***Vectors*** are similar collection type to an array except they can grow or shrink in size.