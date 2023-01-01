mod datatypes;
mod functions;
mod guessing_game;
mod hello_cargo;
mod hello_world;
mod ownership;
mod structs;
mod variables;
mod enums;

fn main() {
    println!("Chapter One");
    hello_world::run();
    hello_cargo::run();
    println!("Chapter Two");
    println!("Skipping Guessing Game!");
    //guessing_game::run();
    println!("Chapter Three");
    functions::run();
    datatypes::run();
    variables::run();
    println!("Chapter Four");
    ownership::run();
    println!("Chapter Five");
    structs::run();
    println!("Chapter Six");
    enums::run();
}
