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

fn main() {
    pretty_env_logger::init();
    let now = Instant::now();
    Day::CombinedLong(day1::main).run("day 1");
    info!("All days together took {:#?}", now.elapsed());
}
