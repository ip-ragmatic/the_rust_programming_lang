fn main() {
    let x: (i32, f64, u8, &str) = (500, 6.4, 1, "Lagrange multiplier");
    let a = x.0;
    let b = x.1;
    let c = x.2;
    let d = x.3;
    
    
    println!("Tuple x's values: {a}, {b}, {c}, {d}");
}