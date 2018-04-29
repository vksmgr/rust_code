---
title: First Problem
date: 2018-03-16 11:51:14
tags:
    -   Rust
    -   Algorithms
    -   HackerEarth
    -   Problems
---

# First Problem

Let's start our first problem to solve. [Problem](https://www.hackerearth.com/practice/data-structures/arrays/1-d/practice-problems/algorithm/monk-and-lucky-minimum-3/)

>Problem Statement: Monk just purchased an array A having N integers. Monk is very superstitious. He calls the array A Lucky if the frequency of the minimum element is odd, otherwise he considers it Unlucky. Help Monk in finding out if the array is Lucky or not.

it's very simple and straight forward problem.

This problem we will solve in two step's
* First : Finding min.
* second: Calculating Frequency.

### Getting User Input:
Before that we need to get user input.

Input:
First line consists of a single integer T denoting the number of test cases.
First line of each test case consists of a single integer N denoting the size of array A.
Second line of each test case consists of N space separated integers denoting the array A.

```$xslt
//getting input for problem.
    let mut T: i32;
    let mut N: i32;
    let mut array:Vec<i32> = vec![];

    T = readInt();
    println!("T :{}",T);
    N = readInt();
    println!("N : {}",N);
```
code to get user input. Here is the readInt method code which 
will get input as string from user and parse it into <i32>

```$xslt
fn readInt() -> i32{
    let mut ip_txt = String::new();
    io::stdin().read_line(&mut ip_txt).expect("error while reading");
    let trimed = ip_txt.trim();

    match trimed.parse::<i32>() {
        Ok(value) => value,
        Err(E) => 00000,
    }
}
```
### First Part:
Now first part of the program. Finding min. this we will do by using linear search.

```$xslt
  //first part: Find Min:

    let mut min = 999999999;
    for index in 0..N {
        if min > array[index]{
            min = array[index];
        }
    }
```
### Second Part: 
Now the second part of the problem.

```
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
```

Source [code](https://github.com/vksmgr); 