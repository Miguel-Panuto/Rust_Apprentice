use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // The '?' it is a shortcut for the simplest error handle, for the case just print
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    // With the unwrap_or_else we can handle the errors with if or else states
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        // This verifies if the error the kind "NotFound", the code talks for himself
        if error.kind() == ErrorKind::NotFound {
            // If it is NotFound, its then the file is created
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem when create the file: {:?}", error);
            })
        } else {
            panic!("Error on opening the file: {:?}", error);
        }
    });

    read_username_from_file();
}
