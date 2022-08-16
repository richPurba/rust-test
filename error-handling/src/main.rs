extern crate core;
extern crate core;

use core::panicking::panic;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    let v = vec![1,2,3];
    // v[999];

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => {
                panic!("Problem opening the file. There is unknown error" )
            }
            // not sure why the following lines also ok with pattern matching:
            // other_err => {
            //     panic!("Problem opening the file {:?}", other_err )
            // }
        }
    };

    let ff = File::open("hello_fresh.txt").unwrap();
    let fff = File::open("hello_fresh.txt").expect("Failed to open hello_fresh.txt");

    read_username_from_file();

}

fn read_username_from_file() -> Result<String, io::Error>{
    let mut ffff = File::open("hello.txt")?;
    let mut s = String::new();
    ffff.read_to_string(&mut s)?;
    Ok(s)
}


fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


pub struct Guess {
   value: i32, // can be public
}

impl Guess {
    pub fn new(value_some: i32) -> Guess {
        if value_some < 1 || value_some > 100 {
            panic!("Guess value must be between 1 and 100, got {}. ", value_some);
        }
        Guess {
              // the filed is assigned
            value: value_some
        }
    }

    // accessor or getter!
    pub fn value(&self ) -> i32 {
        self.value
    }
}