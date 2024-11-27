pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        self.components.iter().for_each(|item| item.draw());
    }
}

pub struct Button {
    pub width: usize,
    pub height: usize,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button has been drawn.");
        // button drawing code
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
