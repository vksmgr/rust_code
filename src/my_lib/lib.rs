use std::io;

//This is my library

//first just test it

pub fn echo(){
    println!("Hello World!");
}

// function to read int

pub fn readInt() -> i32{
    let mut ip_txt = String::new();
    io::stdin().read_line(&mut ip_txt).expect("error while reading");
    let trimed = ip_txt.trim();

 /*   match trimed.parse::<i32>() {
        Ok(value) => value,
        Err(_) => 0000,
    }*/

    trimed.parse::<i32>().unwrap()
}

pub fn readIntArray(array: &mut Vec<i32>){
    let mut ip_txt = String::new();
    io::stdin().read_line(&mut ip_txt).expect("error while reading");
    let trimed = ip_txt.trim();
    let ip_vec :Vec<&str> = ip_txt.split_whitespace().collect();

    for txt in ip_vec.iter() {
        array.push(txt.parse::<i32>().unwrap())
    }
}

//STACK :

pub fn stack(){

}