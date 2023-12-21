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
                Ok(7) => {
                    println!("Day7");
                    days::day07::a();
                    days::day07::b();
                },
                Ok(8) => {
                    println!("Day8");
                    days::day08::a();
                    days::day08::b();
                },
                Ok(9) => {
                    println!("Day9");
                    days::day09::a();
                    days::day09::b();
                },
                Ok(10) => {
                    println!("Day10");
                    days::day10::a();
                    days::day10::b();
                },
                Ok(11) => {
                    println!("Day11");
                    days::day11::a();
                    days::day11::b();
                },
                Ok(12) => {
                    println!("Day12");
                    days::day12::a();
                    days::day12::b();
                },
                Ok(13) => {
                    println!("Day13");
                    days::day13::a();
                    days::day13::b();
                },
                Ok(14) => {
                    println!("Day14");
                    days::day14::a();
                    days::day14::b();
                },
                Ok(15) => {
                    println!("Day15");
                    days::day15::a();
                    days::day15::b();
                },
                _ => println!("Day must be between 1-15")
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
