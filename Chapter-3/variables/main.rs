const TEST: &str = "Ian";

fn main() {
    println!("Hello, my name is {TEST}");

    let x = 40;
    println!("x = {x}");

    let x = x + 2;
    println!("x + 2 = {x}");
    
    {
        let x = x / 3;
        println!("x in the nested scope is x / 3 = {x}");
        println!("\t** Changes made by a shadowing variable in a nested scope\n\t   doesn't effect the original variable in the outer scopes **")
    }

    println!("x in base main() scope is ...... still {x}")
}