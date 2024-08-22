fn main() {
    let a = String::from("Hello");
    let res;
    {
        let b = String::from("Hell");
        res = longest(&a, &b);
    }
    // println!("{res}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
