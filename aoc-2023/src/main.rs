mod utils;
mod days;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            match args[1].parse() {
                Ok(1) => {
                    println!("Day1");
                    days::day01::a();
                    days::day01::b();
                },
                Ok(2) => {
                    println!("Day2");
                    days::day02::a();
                    days::day02::b();
                },
                _ => println!("Day must be between 1-2")
            }
        }
        _ => help()
    }
}

fn help() {
    println!("usage:
cargo run -- <number>
    Runs the prgram for the given day.");
}
