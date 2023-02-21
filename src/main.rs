#![allow(unused)]

use std::fmt::LowerExp;
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use stopwatch::{self, Stopwatch};


fn main() 
{
    let day:i32 = 1;
    let part:i32 = 1;
    let test:bool = false;

    let mut input: String = format!("./Input/day{:02}", day);
    let mut ending: String = "".to_string();

    ending = (test.then_some("_test.txt").unwrap_or(".txt")).to_string();
    input = input + &ending;

    println!("AoC 2018 - Day {} , Part {} - Test Data {}", day, part, test);
    
    let mut st = Stopwatch::new();
    st.start();

    let result:String = match day
    {
        1 => day1(input, part).to_string(),
        _ => "Unimplemented day value".to_string(),
    };
    st.stop();
    println!("Result : {}", result);
    println!("Ellapsed : {:4.3}", st.elapsed().as_millis()*1000);
    
}

fn day1(input:String, part:i32) -> i32
{
    return 0;
}