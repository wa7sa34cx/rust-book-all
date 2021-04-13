use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::fs;
use std::io::Read;

fn main() {
    // panic!("crash and burn");

    // ----

    // let v = vec![1, 2, 3];
    // v[99];

    // ----

    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file {}", error),
    // };

    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // ---- Open or panic
    let f = File::open("hello.txt").unwrap();

    // ---- Open or print error
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// the ? Operator
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// We could even shorten this code
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// even shorter!!!
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}