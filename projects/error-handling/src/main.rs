use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
// fn main() {
// panic!("crash and burn");

// let v = vec![1, 2, 3];

// v[99];

// let greeting_file_result = fs::File::open("hello.txt");

// let greeting_file = match greeting_file_result {
//     Ok(file) => file,
//     Err(error) => match error.kind() {
//         ErrorKind::NotFound => match File::create("hello.txt") {
//             Ok(fc) => fc,
//             Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
//         },
//         other_error => panic!("There was a problem opening the file: {:?}", other_error),
//     },
// };

// come back after reading chapter 13
// let greeting_file = fs::File::open("test.txt").unwrap_or_else(|error| {
//     if error.kind() == io::ErrorKind::NotFound {
//         fs::File::create("test.txt").unwrap_or_else(|error| {
//             panic!("Problem creating the file: {:?}", error);
//         })
//     } else {
//         panic!("Problem opening the file: {:?}", error);
//     }
// });

// let greeting_file = File::open("hellodoesntexist.txt").unwrap();

// let greeting_file = File::open("hellodoesntest.txt")
//     .expect("hellodoesntest.txt should be included in this project");

// In production-quality code, most Rustaceans choose expect rather than unwrap and give more context about why
// the operation is expected to always succeed.
// That way, if your assumptions are ever proven wrong, you have more information to use in debugging.

// read_username_from_file();

// let greeting_file = File::open("hello.txt")?;
// }

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
