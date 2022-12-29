fn main() {
    let mut user1 = build_user(String::from("someone@example.co"), String::from("myuser"));
    let _user2 = User {
        email: String::from("anotheremail@example.com"),
        ..user1 // set all fields that aren't explicitly set but this moves the data in user1 so
                // user1 is now immutable
    };

    user1.active = false;

    println!("User 2 state: {}", _user2.active);
    //let username = user1.username; // Invalid now becuase of move

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;

    let scale = 10;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 10,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    println!("rectangle {:?} square pixels", rect1);

    println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1 {}", rect2.can_hold(&rect1));

    dbg!(rect1);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual; // Unit struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}
