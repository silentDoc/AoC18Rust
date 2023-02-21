use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use stopwatch::{self, Stopwatch};

mod day_01;

fn read_input_file(filename: impl AsRef<Path>) -> Vec<String> 
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() 
{
    let day:i32 = 2;
    let part:i32 = 1;
    let test:bool = !false;

    let mut input: String = format!("./src/Input/day{:02}", day);
    let ending: String;

    ending = (test.then_some("_test.txt").unwrap_or(".txt")).to_string();
    input = input + &ending;

    println!("AoC 2018 - Day {} , Part {} - Test Data {}", day, part, test);
    
    let mut st = Stopwatch::new();
    st.start();

    let result:String = match day
    {
        1 => day1(input, part).to_string(),
        2 => day2(input, part).to_string(),
        _ => "Unimplemented day value".to_string(),
    };
    st.stop();
    println!("Result : {}", result);
    let mut ellapsed_millis =  st.elapsed().as_millis() as f32;
    ellapsed_millis = ellapsed_millis / 1000_f32;

    println!("Ellapsed : {:4.3}",ellapsed_millis);
    
}

fn day1(input:String, part:i32) -> i32
{
    let lines = read_input_file(input);
    return day_01::solve(lines, part);
}

fn day2(input:String, part:i32) -> i32
{
    let lines = read_input_file(input);
    return 0;
}