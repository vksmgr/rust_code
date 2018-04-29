use std::io;
extern crate my_lib;
use my_lib::*;
pub fn call(){

    let mut string = String::new();

    let mut count = 0;
    io::stdin().read_line(&mut string).expect("error");

    for ch in string.chars(){
        match ch {
            'a' => count +=1,
            'e' => count +=1,
            'i' => count +=1,
            'o' => count +=1,
            'u' => count +=1,
            _ => (),
        }
    }
    println!("{}", count);
}

//second problem
pub fn pallindrome(){
    let mut strg = String::new();
    let mut flag: bool = true;
    io::stdin().read_line(&mut strg).expect("Error while reading string");
    let mut rev_str  = String::new();
    let mut rev: String = strg.chars().rev().collect();
    let rev = rev.trim();
    let strg = strg.trim();
    if strg == rev {
        println!("YES");
    }else {
        println!("NO");
    }
}


//third problem

pub fn  permutation(){
    let size: usize = readInt() as usize;
    let mut array: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    let mut swaps = 0;
    readIntArray(&mut array);
    let mut tmp = 0;
    for i in 0..size-1 {
        if array[i] > array[i+1]{
            tmp = array[i];
            array[i] = array[i+1];
            array[i+1] = tmp;
            swaps +=1;
        }
    }

    println!("{}", swaps);
}


//sumit Strings

pub fn strsumit(){
    let mut ip_txt = String::new();
    let mut flag = false;
    io::stdin().read_line(&mut ip_txt).expect("error while reading");
    let mut temp_vec: Vec<u8> = Vec::new();
    let ip_txt = ip_txt.trim();
    let mut vec = ip_txt.to_string().into_bytes();
    for val in vec.iter() {
        temp_vec.push(*val);
    }
    temp_vec.sort();
    //debug
    let mut inc = temp_vec[0];
    for va in temp_vec.iter(){

       if *va ==  inc || *va == (inc-1){  flag = true; inc +=1; }
        else { flag = false; break; }
    }
    if flag {println!("YES");} else { println!("NO"); }
}