pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(val: i32) -> Guess {
        if val < 1 || val > 100 {
            panic!("Your guess must be between 1 and 100, got {}", val);
        }
        Guess { value: val }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {}
