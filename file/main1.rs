use std::fs::File;
use std::io::{self, Read, Write, ErrorKind};

type FileResult = Result<String, io::Error>;

fn open_read_file() -> FileResult {
    let mut file_open = File::open("test1.txt")?;

    let mut s = String::new();

    file_open.read_to_string(&mut s)?;
    Ok(s)
}

fn create_write_file() -> FileResult {
    let mut file_write = File::create("test1.txt")?;

    file_write.write_all(b"Hello World")?;
    
    Ok(String::from("File created and written to successfully."))
}

fn main() {
    match open_read_file() {
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    match create_write_file() {
                        Err(error) => {
                            match error.kind() {
                                ErrorKind::PermissionDenied => {
                                    panic!("Permission Denied: {:#?}", error);
                                }
                                _ => {
                                    panic!("Error occurred {:#?}", error);
                                }    
                            }
                            // return; // Added return here
                        },
                        Ok(s) => {
                            println!("{}", s);
                            // return; // Added return here
                        }
                    }
                },
                _ => {
                    panic!("Error occurred {:#?}", error);
                    // return; // Added return here
                }
            }
        },
        Ok(s) => println!("{}", s),
    }
}
