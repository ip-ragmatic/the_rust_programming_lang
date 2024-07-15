#### Ownership is a discipline for ensuring the safety of Rust programs.
- Safety is the Absence of Undefined Behavior (aka, there not being undefined behavior)
- Rust's goal is to compile programs into efficient binaries that require as few runtime checks as possible. Therefore Rust does not check at runtime whether a variable is defined before being used.

### Ownership as a Discipline for Memory Safety

#### Variables live in the Stack:
- A stack serves as a collection of elements that can “push” or “pop” elements on and off the stack.
- variables live in frames, which are mappings to variables to values that're within a single scope (i.e functions)
- frames are organized into currently-called-functions.
    ```rust
    fn main() {                                 
        let n = 5;                          // stack started by calling main(). main() @ top of stack. L1
        let y = plus_one(n);                // after L2, new local var y=6 initialized. so we have n=5 & y=6. L3
        println!("The value of y is: {y}"); // this line executes and main() returns (ends) and is popped from call stack.
    }
    
    fn plus_one(x: i32) -> i32 {
        x + 1                               // plus_one(n) , plus_one @ top of stack with argument x=5, 
    }                                       // and returns 6. plus_one dropped from stack, main back on top. L2
    ```
- When an expression reads a variable, the variable's value is copied from its slot in the stack frame.
    ```rust
    fn main() {
        let a = 5;     // L1. main() top of stack w/ var a=5
        let mut b = a; // L2. mutable var b initialized w/ value of "a" copied into it. now a=5 and b=5
        b += 1;        // L3. 1 is added to "b". a=5 still but b=6 now. main() returns and dropped from stack
    }
    ```
    this copying of data (copying value of `a` into `b` in this case) can be cumbersome in terms of memory.

#### Boxes live in the heap
- the Heap is an independent region of memory where data can live indefinitely. Heap data is not tied to a specific stack frame
    - the Stack and Heap are two of the segments that make up a program's address space
    - address space is the total memory associated with a running program
- To transfer access to data without copying it, Rust uses pointers
    - A pointer is a value that describes a location in memory (points to an address). 
    - The value that a pointer points-to is called its pointee.
    - One common way to make a pointer is to allocate memory in the heap
- Rust provides a construct called `Box` to put data on the Heap
    - For example, we can wrap the million-element array in `Box::new` like this:
        ```rust
        fn main() {
            let a = Box::new([0; 1_000_000]); // L1. value of "a" is a pointer to the array [0; 1000000] inside the heap
            let b = a;                        // L2. copies the pointer from "a" into "b", but pointed to data not copied
        } 
        ```
- Frames in the stack are associated with a specific function, and are deallocated when the function returns. Data on the heap can live indefinitely. Note that both stack and heap data can be mutable and can be copyable. The heap is allowed to contain pointers
        
#### Rust doesn't permit manual memory management:
- Memory management is the process of allocating memory and deallocating memory.

#### A Box's owner manages deallocation:
- Box deallocation principle (**almost correct**): If a variable is bound to a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory. (Rust automatically frees a box's heap memory)
    - if there's a variable inside a call stack frame bound to a `Box` pointing to something on the heap, and that call stack frame is dropped, then the variable is dropped and so is it's corresponding pointer and heap data.
        ```rust
        fn main() {
            let a_num = 4;           // L1. [main: a_num = 4]
            make_and_drop();         // L3. [main| a_num = 4]
        }
         
        fn make_and_drop() {
            let a_box = Box::new(5); // L2. [make_and_drop| a_box -> 5 (in heap)]   [main| a_num = 4]
        }
        ```
- Box deallocation principle (**fully correct**): If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.
    - when a variable is bound to `Box`, we say that variable owns the `Box`. when a pointer is copied from one variable to another, the copied to variable now owns the `Box`. 

#### Variables cannot be used after being moved:
- Moved heap data principle: if a variable `x` moves ownership of heap data to another variable `y`, 
      then `x` cannot be used after the move.

#### Cloning avoids moves:
- One way to avoid moving data is to clone it using the `.clone()` method
- `.clone()` doesn't "shallow" copy a pointer, but instead "deep" copies the data into a new heap allocation.

#### Summary:
- Ownership is primarily a discipline of heap management (in another sense, a discipline of pointer management):
    - All heap data must be owned by exactly one variable.
    - Rust deallocates heap data once its owner goes out of scope.
    - Ownership can be transferred by moves, which happen on assignments and function calls.
    - Heap data can only be accessed through its current owner, not a previous owner.
- `String::from(Box<str>)` directly places `str` on the heap, and unless ownership is transferred Rust will automatically offload the heap data.
- `Box::new(T)` does not place `T` on the heap, it instead puts a `Box` on the heap w/ a pointer to `T` which is located in the binary.
    - `T` in this case is static (and refers to any given type being stored)
    - binary in this case means 
        - Read-Only Data Section: String literals are stored in the read-only data section of the binary. This section is loaded into memory when the program starts and remains constant throughout the program's execution.
        - Lifetime: String literals have a static lifetime, meaning they are available for the entire duration of the program.
