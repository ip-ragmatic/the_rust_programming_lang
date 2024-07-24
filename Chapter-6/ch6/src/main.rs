// 6.4.6
fn get_or_default(arg: &Option<String>) -> String {
    match arg {
        None => String::new(),
        Some(i) => i.clone()
    }
}

fn main() {
    let opt = Some(String::from("Rust"));
    let s = get_or_default(&opt);
    println!("{}", s);
}

// // 6.2.6
// fn main() {
//     let opt: Option<String> = 
//         Some(String::from("Hello world"));
    
//     match &opt {
//         // _ became s
//         Some(s) => println!("Some: {}", s),
//         None => println!("None!")
//     };
    
//     println!("{:?}", opt);
// }


// // 6.2.3
// fn plus_one(int: Option<i32>) -> Option<i32> {
//     match int {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn main() {
// let x = Some(41);
//     let y = plus_one(x);
//     let z = plus_one(None);
    
//     println!("x is {}\ny is {}\nz is {:?}", x.unwrap(), y.unwrap(), z);
// }