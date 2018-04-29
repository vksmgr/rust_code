
use std::io;
pub fn monk(){
    // This problem is split into two part's
    //First find the min element.
    // Then calculate it's frequency

    let  N: usize;
    let mut array:Vec<i32> = vec![];

    N = readInt() as usize;


    for _ in 0..N as usize{
        array.push(readInt());
    }

    //first part: Find Min:

    let mut min = 999999999;
    for index in 0..N {
        if min > array[index]{
            min = array[index];
        }
    }

    // Now second part is to find frequency of the min element.
    let mut freq: usize = 0;
    for index in 0..N {
        if min == array[index]{
            freq += 1;
        }
    }

    match freq%2 {
        0 => println!("Unlucky"),
        1 => println!("Lucky"),
        _ => print!("Error"),
    }
}

fn readInt() -> i32{
    let mut ip_txt = String::new();
    io::stdin().read_line(&mut ip_txt).expect("error while reading");
    let trimed = ip_txt.trim();

    match trimed.parse::<i32>() {
        Ok(value) => value,
        Err(_) => 00000,
    }
}