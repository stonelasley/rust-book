use std::fs::File;
use std::io::ErrorKind;

pub fn panic() {
    panic!();
}

pub fn try_open_file() {
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem Creating File: {:?}", e),
            },
            other_error => {
                panic!("Problem Opening the file: {:?}", other_error)
            }
        },
    };
}
