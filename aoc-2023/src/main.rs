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
                Ok(3) => {
                    println!("Day3");
                    days::day03::a();
                    days::day03::b();
                },
                Ok(4) => {
                    println!("Day4");
                    days::day04::a();
                    days::day04::b();
                },
                Ok(5) => {
                    println!("Day5");
                    days::day05::a();
                    days::day05::b();
                },
                Ok(6) => {
                    println!("Day6");
                    days::day06::a();
                    days::day06::b();
                },
                _ => println!("Day must be between 1-6")
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
