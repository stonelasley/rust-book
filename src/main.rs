mod chapter_01;
mod chapter_02;
mod chapter_03;
mod chapter_04;
mod chapter_05;
mod chapter_06;
mod chapter_07;
mod chapter_08;
mod chapter_09;

use std::collections::vec_deque;

use chapter_07::front_of_house::hosting;

fn main() {
    println!("---Chapter One---");
    chapter_01::hello_world::run();
    chapter_01::hello_cargo::run();
    println!("---Chapter Two---");
    println!("Skipping Guessing Game!");
    // chapter_02::guessing_game();
    println!("---Chapter Three---");
    chapter_03::functions::run();
    chapter_03::datatypes::run();
    chapter_03::variables::run();
    println!("---Chapter Four---");
    chapter_04::ownership::run();
    println!("---Chapter Five---");
    chapter_05::structs::run();
    println!("---Chapter Six---");
    chapter_06::enums::run();
    println!("---Chapter Seven---");
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    chapter_07::eat_at_restaurant();
    println!("---Chapter Eight---");
    chapter_08::run();
    println!("---Chapter Nine ---");
    println!("Skipping panic!()");
    // chapter_09::panic();
    chapter_09::try_open_file();
}
