## Fearless Concurrency

Handling concurrent programming safely and efficiently is another of Rust's major goals.
***Concurrent programming***, where different parts of a program execute independently, and
***parallel programming***, where different parts of a program execute at the same time, are
becoming increasingly important as more computers take advantage of their multiple processors.

In Rust, many concurrency errors are compile-time errors by leveraging ownership and type checking.
So rather than spending lots of time trying to reproduce the circumstances causing the runtime
concurrency bug, the compiler will reject the code and present an explanation of the error. As a
result, the code can be fixed before shipping. This aspect of Rust is nicknamed ***fearless
concurrency***; concurrent code can be written free of subtle bugs and easy to refactor w/o
introducing new ones.

> For simplicity, mentally substitute *concurrent and/or parallel* with just *concurrent*.

Lower-level languages are expected to provide the solution with the best performance in any given
situation and have fewer abstractions over the hardware. For that reason, Rust offers a variety of
tools for modeling problems in whatever way is appropriate for different situations and
requirements.