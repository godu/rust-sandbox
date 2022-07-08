mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;
pub use crate::front_of_house::hosting::add_to_waitlist as add_to_waitlist_2;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    add_to_waitlist();
    add_to_waitlist_2();
}

#[allow(dead_code)]
fn deliver_order() {}

mod back_of_house {
    #[allow(dead_code)]
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

use rand::Rng;

pub fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("my secret number : {}", secret_number)
}
