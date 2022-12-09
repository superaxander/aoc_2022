#![feature(array_windows)]
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::time::Instant;

use mimalloc::MiMalloc;

use common::{Day, Runnable};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    pretty_env_logger::init();
    let now = Instant::now();
    Day::CombinedLong(day1::main).run("day 1");
    Day::CombinedLong(day2::main).run("day 2");
    Day::CombinedLong(day3::main).run("day 3");
    Day::CombinedLong(day4::main).run("day 4");
    Day::CombinedString(day5::main).run("day 5");
    Day::CombinedUsize(day6::main).run("day 6");
    Day::CombinedUsize(day7::main).run("day 7");
    Day::CombinedUsize(day8::main).run("day 8");
    Day::CombinedUsize(day9::main).run("day 9");
    info!("All days together took {:#?}", now.elapsed());
}
