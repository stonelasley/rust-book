pub mod hosting {
    pub fn add_to_waitlist() {
        println!("Added to waitlist");
    }
    pub fn seat_at_table() {
        println!("Seated at table");
    }
}
mod serving {
    fn take_order() {}
    fn server_order() {}
    fn take_payment() {}
}
