A common theme will be understanding whether a function is actually safe or unsafe. Rust will always reject an unsafe program. But sometimes, Rust will also reject a safe program. These case studies will show how to respond to errors in both situations.

#### Fixing an Unsafe Program: Returning a Reference to the Stack
```rust
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}
```
This program is unsafe because of the lifetime of the referred data. To pass around a reference to a string, you have to make sure that the underlying string lives long enough. 

Four ways to extend string lifetime:
1. move ownership of the string out of the function, changing `&String` to `String`
    ```rust
    fn return_a_string() -> String {
        let s = String::from("Hello world");
        s
    }
    ```

2. return a string literal, which lives forever (indicated by 'static). This solution applies if we never intend to change the string, and then a heap allocation is unnecessary
    ```rust
    fn return_a_string() -> &'static str {
        "Hello world"
    }
    ```

3. defer borrow-checking to runtime by using garbage collection. For example, by using a reference-counted pointer
    ```rust
    use std::rc::Rc;
    fn return_a_string() -> Rc<String> {
        let s = Rc::new(String::from("Hello world"));
        Rc::clone(&s)
    }
    ```
    `Rc::clone` only clones a pointer to `s` and not the data itself. At runtime, the `Rc` checks when the last `Rc` pointing to data has been dropped, and then deallocates the data

4. have the caller provide a "slot" to put the string using a mutable reference
    ```rust
    fn return_a_string(output: &mut String) {
        output.replace_range(.., "Hello world");
    }
    ```
    Here the caller is responsible for creating space for the string. This style can be verbose, but can also be more memory-efficient if the caller needs to carefully control when allocations occur

#### Fixing an Unsafe Program: Not Enough Permissions
A common issue is trying to mutate read-only data, or trying to drop data behind a reference.
```rust
fn stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
} // ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."
```
This function is supposed to create a person's full name from a vector of name parts, including an extra title. The program is rejected by the borrow checker because `name` is an immutable reference, but `name.push(..)` requires the W permission. It's unsafe because `push` could invalidate other references to `name` outside of `stringify_name_with_title`
1. change the type of name from `&Vec<String>'`to `&mut Vec<String>`
    ```rust
    fn stringify_name_with_title(name: &mut Vec<String>) -> String {
        name.push(String::from("Esq."));
        let full = name.join(" ");
        full
    }
    ```
    This ins't a good solution. Functions should not mutate their inputs if the caller would not expect it.


2. take ownership of the name, by changing `&Vec<String>` to `Vec<String>`.
    ```rust
    fn stringify_name_with_title(name: Vec<String>) -> String {
        name.push(String::from("Esq."));
        let full = name.join(" ");
        full
    }
    ```
    But this is also not a good solution! It is very rare for Rust functions to take ownership of heap-owning data structures like Vec and String. This version of `stringify_name_with_title` would make the input name unusable

3. clone the input `name`.
    ```rust
    fn stringify_name_with_title(name: &Vec<String>) -> String {
        let mut name_clone = name.clone();
        name_clone.push(String::from("Esq."));
        let full = name_clone.join(" ");
        full
    } 
    ```
    or
    ```rust
    fn stringify_name_with_title(name: &Vec<String>) -> String {
        let mut full = name.join(" ");
        full.push_str(" Esq.");
        full
    }
    ```

Quiz 1 Notes:
- A stack frame cannot have its lifetime extended, so it's not a valid solution to the issue of returning a stack reference from a function
- if a function is supposed to modify specific data in place, it wouldn't be idiomatic to return a new copy of the same data (`-> T`) or to consume ownership of the data (`mut var: T`). The best solution is to change the signature from `&T` to `&mut T`.

#### Fixing an Unsafe Program: Aliasing and Mutating a Data Structure
Another unsafe operation is using a reference to heap data that gets deallocated by another alias
```rust
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String =                               
      dst.iter().max_by_key(|s| s.len()).unwrap();      // not let mut largest, W permissions removed from dst
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}
```
This program is rejected by the borrow checker because `let largest = ..` removes the W permissions on `dst`. However, `dst.push(..)` requires the W permission. So it's unsafe b/c `dst.push(..)` could deallocate the contents of `dst`, invalidating the reference largest.

To fix the program, the key insight is that we need to shorten the lifetime of 'largest' to not overlap with 'dst.push(..)'
1. clone `largest`
   ```rust
   fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
       let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
       for s in src {
           if s.len() > largest.len() {
               dst.push(s.clone());
           }
       }
   }
   ```

2. perform all the length comparisons first, and then mutate dst afterwards
    ```rust
    fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
        let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
        let to_add: Vec<String> = 
            src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
        dst.extend(to_add);
    }
    ```

3. copy out the length of largest, since we don't actually need the contents of largest, just its length.
    ```rust
    fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
        let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
        for s in src {
            if s.len() > largest_len {
                dst.push(s.clone());
            }
        }
    }
    ```
> If a value does not own heap data, then it can be copied without a move. For example:
> - An `i32` does not own heap data, so it can be copied without a move.
> - A `String` does own heap data, so it can not be copied without a move.
> - An `&String` does not own heap data, so it can be copied without a move.

#### Fixing an Unsafe Program: Copying vs. Moving Out of a Collection
copying data out of a collection, like a vector.
```rust
fn main() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    let s: String = *s_ref; // error[E0507]: cannot move out of `*s_ref` which is behind a shared reference
}
```
The issue is that `v` own's the string "Hello world". Dereferencing `s_ref` tries to take ownership of the string through the reference. Recall references are non-owning pointers, so we can't take ownership through a reference (something that doesn't own).

So if we have a vector of non-'Copy' types like 'String', then how do we safely get access to an element of the vector? 
1. avoid taking ownership of the string and just use an immutable reference:
    ```rust
    fn main() {
        let v: Vec<String> = vec![String::from("Hello world")];
        let s_ref: &String = &v[0];
        println!("{s_ref}!");
    }
    ```

2. clone the data if you want to get ownership of the string while leaving the vector alone
    ```rust
    fn main() {
        let v: Vec<String> = vec![String::from("Hello world")];
        let mut s: String = v[0].clone();
        s.push('!');
        println!("{s}");
    }
    ```

3. use a method like 'Vec::remove' to move the string out of the vector
    ```rust
    fn main {
        let mut v: Vec<String> = vec![String::from("Hello world")];
        let mut s: String = v.remove(0);
        s.push('!');
        println!("{s}");
        assert!(v.len() == 0);
    }
    ```

#### Fixing a Safe Program: Mutating Different Tuple Fields
A common issue is that Rust tries to track permissions at a fine-grained level. However, Rust may conflate two different paths as the same path. So Rust can lose track of exactly which paths are borrowed.

The problem is that Rust only looks at the type signature. In the case of a function that expects a tuple reference type `&(T, T)`, Rust conservatively decides that both fields of the input tuple are borrowed and thus removes both W and O permissions on them.

#### Fixing a Safe Program: Mutating Different Array Elements
A similar kind of problem to the above arises when we borrow elements of an array. Rust's borrow checker doesn't have different paths for each index in an array. So the borrow checker just uses a single path 'a[_]' to represent all indexes of an array. Rust does this because it cannot always determine the value of an index (e.g. index value is returned from a complex function).
```rust
fn main() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];
    let y = &a[2];
    *x += *y;
}
```
Although this program is safe, Rust rejects this program because at line 3 `a[_]` gave its R permission to `x`, but then `y` is trying to assume R permissions over `[_]` in line 4. This violates the Pointer Safety Principle because even though lines 3 and 4 are trying to handle different indexes, Rust treats them both as one path, and a piece of data cannot be aliased and mutated at the same time.

For cases like these, Rust often provides a function in the standard library that can work around the borrow checker.
1. use `slice::split_at_mut` to split `a` into two different arrays and operate on each one.
    ```rust
    fn main() {
        let mut a = [0, 1, 2, 3];
        let (a_l, a_r) = a.split_at_mut(2); // a_l = [0,1] and a_r = [2,3] 
        let x = &mut a_l[1];
        let y = &a_r[0];
        *x += *y;
    }
    ```

2. use an unsafe block. unsafe blocks allow the use of "raw" pointers, which are not checked for safety by the borrow checker.
    ```rust
    fn main() {
        let mut a = [0, 1, 2, 3];
        let x = &mut a[1] as *mut i32;
        let y = &a[2] as *const i32;
        unsafe { *x += *y; } // DO NOT DO THIS unless you know what you're doing!
    }
    ```

#### Quiz 2 Notes:
1. an `i32` can be copied without a move while a `String` can't because a it owns data on the heap, while an `i32` doesn't. If a `String` could be copied without a move, then two variables could think they own the same string, leading to a double-free.
2. The following code snippet does not compile:
    ```rust
    let s = String::from("Hello world");
    let s_ref = &s;
    let s2 = *s_ref;
    println!("{s2}");
    ```
    The undefined behavior occurs b/c the string is freed twice at the end of the program. The `println` is technically safe since the string won't be deallocated until the end of the current scope. But the undefined behavior occurs when the string is freed. It's freed twice on behalf of `s` and `s2`; `*s_ref` points to the same thing `s` does.

#### Summary:
When fixing an ownership error, you should ask yourself, "is my program actually unsafe?" If yes, then you need to understand the root cause of the unsafety. If no, then you need to understand the limitations of the borrow checker to work around them.