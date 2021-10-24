#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate num;
extern crate dynparser;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod utils;

fn run() -> Result<(), utils::ParseError> {
  day25::problem1()?;
  day25::problem2()?;

  Ok(())
}

fn main() {
  match run() {
    Err(err) => println!("Error occurred: {}", err),
    _ => {}
  }
}
