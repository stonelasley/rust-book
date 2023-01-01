pub mod ownership {
    pub fn run() {
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
        let s2_edited = s;
        // println!("{}", s); Invalid "value borrowed here after move"
        print!("{}", s2_edited);

        let my_str = String::from("Ownership");
        takes_ownership(my_str);
        // my_str.push_str("something"); Invalid here

        let mut my_int = 5;
        makes_copy(my_int);
        my_int = 1;
        println!("my_int: {}", my_int);

        let x = 5;
        let y = x;
        let z = x.clone();
        println!("x = {}, y = {}, z = {}", x, y, z);

        let a1 = gives_ownership();
        let a2 = String::from("hello world");
        let a3 = takes_and_gives_back(a2);
        println!("a1 = {}, a3 = {}", a1, a3); // cannot print a2 here as it was moved

        let borrowed_str = String::from("hello");
        let len = calculate_length(&borrowed_str);
        println!("The size of {} is {}", borrowed_str, len);
        let mut borrowed_changed = String::from("my val");
        change(&mut borrowed_changed);
        println!("{}", borrowed_changed);

        let mut words = String::from("hello world");
        // let word = first_word(&words);
        // words.clear();
        // this can cause a bug as the word index no longer applies.

        //let hello = &words[..5];
        //let world= &words[6..];
        //println!("slice a: {}", hello);
        //println!("slice b: {}", world);
        let first_word = first_word_slice(&words);
        // words.clear(); this is prevented with "immutable borrow" error
        println!("first slice: {}", first_word);
    }

    fn takes_ownership(some_string: String) {
        println!("{}", some_string)
    }

    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer)
    }

    fn takes_and_gives_back(a_str: String) -> String {
        a_str
    }

    fn gives_ownership() -> String {
        // gives_ownership will move its
        // return value into the function
        // that calls it

        let some_string = String::from("yours"); // some_string comes into scope

        some_string // some_string is returned and
                    // moves out to the calling
                    // function
    }

    // Borrows the value
    fn calculate_length(s: &String) -> usize {
        // s with the '&' signifies a reference which is 'borrowing'
        s.len()
    } // s has gone out of scope at this point but because of the '&' the item being referenced will
      // not be dropped;

    fn change(s: &mut String) {
        s.push_str(", changed");
    }

    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        s.len()
    }

    fn first_word_slice(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }
}
