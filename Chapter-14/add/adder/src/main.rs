use add_one::add_one;
use times_two::times_two;


fn main() {
    println!("Hello, world!");

    let num1 = 41;
    println!("\n{num1} + 1 = {}. The answer to the universe and everything.", add_one(num1));
    
    let num2 = 21;
    println!("\nWhat is 21 * 2? It is also ... {}!!!", times_two(num2));
}
