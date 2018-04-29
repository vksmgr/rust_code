mod g;
extern crate my_lib;

use g::he::algo::strings;
use my_lib::*;
use g::he::ds::array_1d;
use std::io;

fn main() {
   /* echo();
    let a = readInt();
    println!(" out put : {} ", a);*/

    let T = readInt();

    for _ in 0..T {
       strings::strsumit();
    }

   /* for arg in std::env::args(){
        print!("{}", arg);
    }*/
   // strings::permutation();

}

