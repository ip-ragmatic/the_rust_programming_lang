mod front_of_house;
    
mod back_of_house;

use crate::front_of_house::hosting;

#[allow(dead_code)]
fn deliver_order() {}

pub fn eat_at_restaurant() {
    hosting::add_to_wait();
}
