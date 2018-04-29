use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
pub fn read_file_input(){

    // 1: Create path variable;
    let path = Path::new("hello.txt");

    //2: Open file using path

    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open file: {}",why.description()),
        Ok(file) => file,
    };

    //Create Buffer to read content

    let mut string = String::new();

    match file.read_to_string(&mut string) {
        Err(why) => panic!("There is and Error while reading : {}",why.description()),
        Ok(string) => string,
    };

    for line in string.lines() {
       // println!("printing line by line:  {}",line);
        print!("Number : {} ",line.parse::<i32>().unwrap());
    }
}

pub fn create_file(path: &str) -> File{
    let path = Path::new(path);

    let file = match File::create(&path) {
        Err(why) => panic!("There is error! {}",why.description()),
        Ok(file) => file,
    };
    file
}