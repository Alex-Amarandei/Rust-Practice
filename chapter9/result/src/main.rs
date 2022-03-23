use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let _f = File::open("hello.txt");

    let _f = match _f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // let f = File::open("hello.txt").unwrap(); will return the value inside Ok or will panic
    // let f = File::open("hello.txt").expect("Error message goes here"); will act similarly but you can configure the panic message

    println!("{:?}", example2());
}

// fn example2() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn example2() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

fn example2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn _shortest_example2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
