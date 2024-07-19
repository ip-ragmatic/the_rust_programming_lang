fn main() {
    test3();
}

fn test1() {
    let arr = [2, 22, 42, 27, 69];
    let mut idx = arr.len();

    while idx > 0 {
        idx -= 1;
        println!("{}", arr[idx]);
    }
}

fn test2() {
    let arr = [2, 22, 42, 27, 69];
    for &item in arr.iter().rev() {
        println!("{}", item)
    }
}

fn test3() {
    for num in (1..4).rev() {
        println!("{num}")
    }
    println!("LIFTOFF!!!")
}