#![allow(unused)]
use std::fs::File;
use std::io::ErrorKind;
use std::io::prelude::*;
fn main() {
let mut file = File::open("hello1.txt");


match file {
    Ok(mut file) => {
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        println!("The contents is {:#?}", contents);
    }
    Err(error)=> match error.kind() {
        ErrorKind::NotFound => {
            match File::create("hello1.txt") {
                Ok(mut fc) => {
                    let mut content = String::new();
                    fc.write_all(b"Hello, World!");
                    fc.read_to_string(&mut content);
                    println!("The content is {:#?}", content);
                }
                Err(error) => {
                    panic!("Error {} creating file)",error);
                }    
            }
        },
        other_error => {
            panic!("Problem opening the file {}", other_error);
        }

    }
}
}
#![allow(unused)]
use std::fs::File;
use std::io::ErrorKind;
use std::io::prelude::*;
fn main() {
let mut file = File::open("hello1.txt");


match file {
    Ok(mut file) => {
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        println!("The contents is {:#?}", contents);
    }
    Err(error)=> match error.kind() {
        ErrorKind::NotFound => {
            match File::create("hello1.txt") {
                Ok(mut fc) => {
                    let mut content = String::new();
                    fc.write_all(b"Hello, World!");
                    fc.read_to_string(&mut content);
                    println!("The content is {:#?}", content);
                }
                Err(error) => {
                    panic!("Error {} creating file)",error);
                }    
            }
        },
        other_error => {
            panic!("Problem opening the file {}", other_error);
        }

    }
}
}
