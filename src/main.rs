use std::io::{self, BufRead};

mod env;

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    println!("The Rain Challenge");
    println!("Please enter the relief as a space delimited list of integers:");
    let relief_input = iterator.next().unwrap().unwrap();

    let relief: Vec<u32> = relief_input
        .split_whitespace()
        .map(|col| col.parse::<u32>().unwrap())
        .collect();

    let mut env = env::Environment::new(relief);

    println!("Thank You!");
    println!("How many hours of rain will occour?");
    let rain_input = iterator.next().unwrap().unwrap();
    let rain: f32 = rain_input.parse::<f32>().unwrap();

    env.rain(rain);

    println!("Result is :");
    println!("{:?}", env);
}
