*Scalar types* are those that represent a single value. The 4 main scalar types are:
1. integer
2. floats
3. booleans
4. characters.

***Integers***
- can be signed (domain is [ -(2^(n-1), 2^(n-1)-1) ]) or unsigned (domain [ 0, 2^(n)-1 ]) n-bit integers.
    - prefix `i` indicates signed integer and `u` indicates unsigned integer, the prefix is followed by bit size. They are 8-, 16-, 32-, 64-, and 128-bit lengths and also "arch" which just elects the bit size equal to that of the processors architecture.
        - signed arch is `isize` and unsigned is `usize`.
    - Signed numbers follow Two's Complement.
- Number literals (ways of expressing them) include: decimals (3.14), hex (0xff), octals (0o77), binary (0b1111_0000), and bytes (b'A').
    - can also express integers by using the number followed by the type (e.g. 42u32).
- *IMPORTANT*: integer division rounds the result down so that the result isn't a float. So `5 / 3 = 1` (kind of like a floor function).
    ```rust
    let val: i32 = -42;
    let hex = 0xff; (0xff = 255)
    ```

***Floats***
- floating-point numbers are decimals.
- 2 type annotations for floats are `f32` and `f64`.
- floats are signed.
    ```rust
    let x: f64 = -3.1415926536;
    ```

***Characters***
- literally a single character or symbol, and are defined using single quotes (`''`)
- type annotation for characters is `char`
- strings are a collection of characters. type annotation for strings is &str
    ```rust
    let character = '∫';    // note the use of single quotes
    let name: &str = "Ian"; // use of double quotes and no heap allocated string declaration
    ```