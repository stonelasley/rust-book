const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

pub fn run() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    let y = 10;
    {
        let mut y = y + 5;
        y = y + 1;
        println!("The value of y inside scope is {y}");
    }
    println!("The value of y outside scope is {y}");



}
