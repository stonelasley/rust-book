pub mod back_of_house;
pub mod front_of_house;

use crate::chapter_07::front_of_house::hosting;

fn deliver_order() {}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // absolute
    front_of_house::hosting::seat_at_table(); // relative
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    println!("I'd like {} toast, please.", meal.toast);

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}
