use anyhow::Result;
use core::convert::AsRef;
use core::result::Result::Ok;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

pub fn read_lines<P>(filename: P) -> Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub enum Day {
    Combined(fn() -> Result<(i32, i32)>),
    CombinedUsize(fn() -> Result<(usize, usize)>),
    CombinedLong(fn() -> Result<(i64, i64)>),
    CombinedString(fn() -> Result<(String, String)>),
    Separated(fn(bool) -> Result<i32>),
    SeparatedLong(fn(bool) -> Result<i64>),
    SeparatedUsize(fn(bool) -> Result<usize>),
    SeparatedULong(fn(bool) -> Result<u64>),
}

impl Day {
    fn run_with_result(&self, name: &str) -> Result<()> {
        match self {
            Day::Combined(func) => {
                let now = Instant::now();
                let (solution_a, solution_b) = func()?;
                info!("Combined parts took {:#?}", now.elapsed());
                info!("Solution {}a: {}", name, solution_a);
                info!("Solution {}b: {}", name, solution_b);
            }
            Day::CombinedUsize(func) => {
                let now = Instant::now();
                let (solution_a, solution_b) = func()?;
                info!("Combined parts took {:#?}", now.elapsed());
                info!("Solution {}a: {}", name, solution_a);
                info!("Solution {}b: {}", name, solution_b);
            }
            Day::CombinedLong(func) => {
                let now = Instant::now();
                let (solution_a, solution_b) = func()?;
                info!("Combined parts took {:#?}", now.elapsed());
                info!("Solution {}a: {}", name, solution_a);
                info!("Solution {}b: {}", name, solution_b);
            }
            Day::CombinedString(func) => {
                let now = Instant::now();
                let (solution_a, solution_b) = func()?;
                info!("Combined parts took {:#?}", now.elapsed());
                info!("Solution {}a: {}", name, solution_a);
                info!("Solution {}b: {}", name, solution_b);
            }
            Day::Separated(func) => {
                let now = Instant::now();
                let result_a = func(false)?;
                info!("Part a took {:#?}", now.elapsed());
                let now = Instant::now();
                let result_b = func(true)?;
                info!("Part b took {:#?}", now.elapsed());
                info!("Solution {}a: {}", name, result_a);
                info!("Solution {}b: {}", name, result_b);
            }
            Day::SeparatedLong(func) => {
                let now = Instant::now();
                let result_a = func(false)?;
                info!("Part a took {:#?}", now.elapsed());
                let now = Instant::now();
                let result_b = func(true)?;
                info!("Part b took {:#?}", now.elapsed());
                info!("Solution {}a: {}", name, result_a);
                info!("Solution {}b: {}", name, result_b);
            }
            Day::SeparatedUsize(func) => {
                let now = Instant::now();
                let result_a = func(false)?;
                info!("Part a took {:#?}", now.elapsed());
                let now = Instant::now();
                let result_b = func(true)?;
                info!("Part b took {:#?}", now.elapsed());
                info!("Solution {}a: {}", name, result_a);
                info!("Solution {}b: {}", name, result_b);
            }
            Day::SeparatedULong(func) => {
                let now = Instant::now();
                let result_a = func(false)?;
                info!("Part a took {:#?}", now.elapsed());
                let now = Instant::now();
                let result_b = func(true)?;
                info!("Part b took {:#?}", now.elapsed());
                info!("Solution {}a: {}", name, result_a);
                info!("Solution {}b: {}", name, result_b);
            }
        }
        Ok(())
    }
}

pub(crate) trait Runnable {
    fn run(&self, name: &str);
}

impl Runnable for Day {
    fn run(&self, name: &str) {
        if let Err(e) = self.run_with_result(name) {
            error!("Error occurred running {}: {}", name, e);
        }
    }
}
