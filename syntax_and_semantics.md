## Keywords and stuff
`fn` is used to create/define a function

<br/>

`const` used to define a constant variable

<br/>

`String` is a heap allocated string that is mutable. It owns its data.
- `String::from("some kind of string")`
- `"some string".to_string()`

<br/>

`&str` is a string slice that can be stored in the heap, stack, or binary.
- string literal is an &str stored in program's binary

<br/>


## Methods, functions, and macros

`.to_string()` converts a string slice `&str` to a `String`

<br/>

`.to_vec()` converts an immutable reference type `T` to `Vec<T>` by cloning each element.
- typically used on collection types and slice types

<br/>

`.contains()` used to check if a particular element or pattern exists within a collection or string

<br/>

`.into()` converts a value from one type to another compatible type.
- Part of the `Into` trait, which is the reciprocal of the From trait

<br/>

`.push_str(&str)` used to append a string slice `&str` to the end of a `String`

<br/>

`.push(data)` used to add an element to the end of a vector or other growable collection

<br/>

`assert!()` checks if a boolean condition is `true`. If the condition is `false`, it will panic and terminate the program.

<br/>

`.clone()` creates a duplicate of an object.

<br/>

`vec!` allows Vecs to be defined with the same syntax as array expressions
- `vec![1, 2, 3]`
- `vec![1; 5]`

<br/>

`.remove(idx)` used to remove and return an element from a collection at a specified index (idx).

<br/>

`.replace_range()` replaces a specified range of characters in a String with new content.

<br/>

`.join("<seperator>")` used to concatenate elements of an iterator into a single `String`, with a specified separator between each element.

<br/>

`.iter()` creates an iterator over the elements of a collection, such as a vector or an array.

<br/>

`.map()` takes a closure an creates an iterator which calls that closure on each element.
- `map()` transforms one iterator into another by way of its argument, which implements the FnMut
- syntax: `map(|<closure>| {some action to perform on <iterator>})`

<br/>

`.unwrap()` extracts the value inside an `Option` or `Result`, panicking if it is `None` or `Err`. 

<br/>

`.max_by_key()` finds the maximum element in an iterator based on a specified key function. Returns an `Option` containing a reference to the maximum element, or `None` if the iterator is empty.
- Takes a closure that extracts a key from each element.
- The key is used to determine the ordering.

<br/>

`.filter()` Creates an iterator that yields only the elements that satisfy a given predicate.

<br/>

`.collect()` Transforms an iterator into a collection.

<br/>

`.as_bytes()` converts a `String` type to an array of bytes

<br/>

`.enumerate()` wraps the result of `.iter()` and returns each element as part of a tuple instead.
- first element of tuple returned is the index
- second element is a reference to the element

<br/>

`split_at_mut(&mut self, mid: usize)` divides one mutable slice into two at an index. The first will contain all indices from [0, mid) and the second will contain all indices from [mid, len)
- thing of math notation for ranges and domains when reading description

<br/>

`max(iterable: I)` returns the maximum value of the iterable `I`.
- spits out `Option<I::Item>`
- `Item` refers to the type of elements being iterated over

<br/>

`format!()` creates a `String` type using interpolation of runtime expressions.
- The first argument is a formatting string that's a string literal (`&str`).
- Additional parameters passed to `format!` replace any `{}`s within the formatting string in the order.
- Examples:
	```rust
	format!("test");
	format!("hello {}", "world!");
	```

<br/>



## Standard Library stuff
