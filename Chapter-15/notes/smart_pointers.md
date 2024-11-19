## Smart Pointers

A *pointer* describes a variable containing an address in memory. This address
refers to ("points at") some other data. The most common kind of pointer in
Rust is a reference. They're only capable of referring to data and have no
overhead.

On the other hand, a ***smart pointer*** is a data structure that acts like a
pointer but also has additional metadata and capabilities. One difference
between references and smart pointers is that smart pointers ***own*** the data
they point to, whereas references only borrow the data. There's a bunch of
different types of smart pointers in Rust's standard library.
- one kind is the *reference counting smart pointer* which enables you to allow
  data to have multiple owners by tracking the number of owners. When no owners
  remain, it automatically cleans up the data.
- `String` and `Vec<T>` are actually smart pointers too, as they own memory and
  allow manipulation of it. Additionally, they have metadata and extra
  capabilities. For example, `String` guarantees that it's data will always be
  valid UTF-8.

Smart pointers are usually implemented using structs that implement the `Deref`
and `Drop` traits.
- `Deref` trait allows an instance of the smart pointer struct to behave like a
  reference so you can write your code to work with either references or smart
  pointers.
- `Drop` trait allows you to customize the code that's run when an instance of
  the smart pointer goes out of scope.
  
The most common smart pointers in the standard library:
- `Box<T>` for allocating values on the heap.
- `Rc<T>` is a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>` are accessed through `RefCell<T>`, a type that
  enforces the borrowing rules at runtime instead of compile time
