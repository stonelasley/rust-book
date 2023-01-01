pub fn run() {
    println!("{}", hello_cargo());
}

fn hello_cargo() -> &'static str {
    return "Hello, Cargo!";
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_hello_cargo() {
        assert_eq!(hello_cargo(), "Hello, Cargo!");
    }
}

