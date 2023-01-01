pub mod functions {
    pub fn run() {
        let y = {
            let x = 3;
            x + 1
        };

        let x = five();
        println!("The value of x is {x}")
    }

    fn five() -> i32 {
        5
    }
}

pub mod datatypes {
    pub fn run() {
        // explicit type
        let guess: u8 = "32".parse().expect("Not a Number");
        // Tuples
        let tup = (500, 6.4, 1);

        // Char
        let c = 'z';
        let z: char = 'ℤ';
        let heart_eyed_cat = '😻';
    }
}

pub mod variables {

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
}
