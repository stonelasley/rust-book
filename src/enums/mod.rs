pub fn run() {
    let _home = IpAddrKind::Ipv4(127, 0, 0, 1);
    let _loopback = IpAddrKind::Ipv6(String::from("::1"));

    let m = Message::Write(String::from("hello!"));
    m.call();

    let _some_number: Option<i32> = Some(5);
    let _some_char: Option<char> = Some('a');
    let _missing_number: Option<i32> = None;
    let _cents = value_in_cents(Coin::Dime);
    let _quarter = value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let _six = plus_one(five);
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            print!("State Quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
enum IpAddrKind {
    Ipv4(u8, u8, u8, u8),
    Ipv6(String),
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Texas,
    Utah,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call");
    }
}
