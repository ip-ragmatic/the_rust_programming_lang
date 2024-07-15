### condtional expressions
- "if" is an expression allowing you to branch your code depending on conditions.
- "else if" is between "if" and "else"; extra branch of control for program beside "if" and "else"
- "else" is an alternative block of code to execute should the previous condition evaluate to "false"
    - Blocks of code associated with the conditions in "if" expressions are sometimes called arms
    - if the "if" condition is "false" and there's no "else" arm, then program will skip the "if" block and continue on in the code.
    - the condition provided must be a boolean
        - unlike other languages, we need to explicity provide "if" with a boolean, Rust doesn't try to convert non-booleans to booleans.
    - Rust only executes block for first true condition. once it finds one, it doesn’t check the rest.
    - potential results from each "if" arm need to be the same type; otherwise incompatible type error
    - "if"s can be used to assign a value to a variable like so: `let x = if cond {5} else {6}`
    ```rust
    fn main() {
        let number = 5;

        if number < 5 {
            println!("the number is less than 5");
        } else if number == 5 {
            println!("the number equals 5!")
        } else {
            println!("the number is greater than 5");
        }
    }
    ```
            
### loops (like python "for" loop)
The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop. Note that `break` and "continue" work the same in Rust as it does in python. `break` stops the loop, `continue` skips over remaining code. 
- `break` can be combined with a condition if it comes after an `if` expression inside `loop`
- `break` and `return` both cause code after them to not be executed. Rust treats them as units, or `()`. loop labels can be used to specify which loop to perform an action on. they're expressed as: `'labelName : loop { some action here}`
    ```rust
    fn main() {
        let mut count = 0;                             // iteration counter, increases
        'counting_up: loop {                           // counting_up is name of this loop
            println!("count = {count}");
            let mut remaining = 10;                    // count down counter
            
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {                    // if "remaining" counts down to 9, break inner loop
                    break;                             
                }
                if count == 2 {
                    break 'counting_up;                // if "count" counts up to 2, break counting_up loop
                }
                remaining -= 1;                        // makes "remaining" counter decrement
            }
        
            count += 1;                                // makes "count" counter increment
        }
        println!("End count = {count}");
    }
    ```
      
### while
a loop that runs while it's condition is `true`. once condition is `false`, `break` is called.
    
### Range

provided by the standard library, generates all numbers in sequence starting from one number and ending before another number.
- if we wanted to count down from a number, we could combine a range of numbers and the method `rev()`, which reverses the range.
    ```rust
    fn main() {
        for number in (1..4).rev() {    // (1..4) is range of numbers from 1 to 3, .rev() reverses this range
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
    ```