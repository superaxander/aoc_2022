use core::convert::AsRef;
use core::result::Result::Ok;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Result};
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
    Combined(fn() -> io::Result<(i32, i32)>),
    CombinedUsize(fn() -> io::Result<(usize, usize)>),
    CombinedLong(fn() -> io::Result<(i64, i64)>),
    Separated(fn(bool) -> io::Result<i32>),
    SeparatedLong(fn(bool) -> io::Result<i64>),
    SeparatedUsize(fn(bool) -> io::Result<usize>),
    SeparatedULong(fn(bool) -> io::Result<u64>),
}

pub(crate) trait Runnable {
    fn run(&self, name: &str);
}

impl Runnable for Day {
    fn run(&self, name: &str) {
        match self {
            Day::Combined(func) => {
                let now = Instant::now();
                let result = func();
                info!("Combined parts took {:#?}", now.elapsed());
                match result {
                    Ok((solution_a, solution_b)) => {
                        info!("Solution {}a: {}", name, solution_a);
                        info!("Solution {}b: {}", name, solution_b);
                    }
                    Err(err) => error!("Error occurred running {}: {}", name, err),
                }
            }
            Day::CombinedUsize(func) => {
                let now = Instant::now();
                let result = func();
                info!("Combined parts took {:#?}", now.elapsed());
                match result {
                    Ok((solution_a, solution_b)) => {
                        info!("Solution {}a: {}", name, solution_a);
                        info!("Solution {}b: {}", name, solution_b);
                    }
                    Err(err) => error!("Error occurred running {}: {}", name, err),
                }
            }
            Day::CombinedLong(func) => {
                let now = Instant::now();
                let result = func();
                info!("Combined parts took {:#?}", now.elapsed());
                match result {
                    Ok((solution_a, solution_b)) => {
                        info!("Solution {}a: {}", name, solution_a);
                        info!("Solution {}b: {}", name, solution_b);
                    }
                    Err(err) => error!("Error occurred running {}: {}", name, err),
                }
            }
            Day::Separated(func) => {
                let now = Instant::now();
                let result_a = func(false);
                info!("Part a took {:#?}", now.elapsed());
                let now = Instant::now();
                let result_b = func(true);
                info!("Part b took {:#?}", now.elapsed());
                match result_a {
                    Ok(solution_a) => {
                        info!("Solution {}a: {}", name, solution_a);
                    }
                    Err(err) => error!("Error occurred running {}: {}", name, err),
                }

                match result_b {
                    Ok(solution_b) => {
                        info!("Solution {}b: {}", name, solution_b);
                    }
                    Err(err) => error!("Error occurred running {}: {}", name, err),
                }
            }
            Day::SeparatedLong(func) => {
                let now = Instant::now();
                let result_a = func(false);
                info!("Part a took {:#?}", now.elapsed());
                let now = Instant::now();
                let result_b = func(true);
                info!("Part b took {:#?}", now.elapsed());
                match result_a {
                    Ok(solution_a) => {
                        info!("Solution {}a: {}", name, solution_a);
                    }
                    Err(err) => error!("Error occurred running {}: {}", name, err),
                }

                match result_b {
                    Ok(solution_b) => {
                        info!("Solution {}b: {}", name, solution_b);
                    }
                    Err(err) => error!("Error occurred running {}: {}", name, err),
                }
            }
            Day::SeparatedUsize(func) => {
                let now = Instant::now();
                let result_a = func(false);
                info!("Part a took {:#?}", now.elapsed());
                let now = Instant::now();
                let result_b = func(true);
                info!("Part b took {:#?}", now.elapsed());
                match result_a {
                    Ok(solution_a) => {
                        info!("Solution {}a: {}", name, solution_a);
                    }
                    Err(err) => error!("Error occurred running {}: {}", name, err),
                }

                match result_b {
                    Ok(solution_b) => {
                        info!("Solution {}b: {}", name, solution_b);
                    }
                    Err(err) => error!("Error occurred running {}: {}", name, err),
                }
            }
            Day::SeparatedULong(func) => {
                let now = Instant::now();
                let result_a = func(false);
                info!("Part a took {:#?}", now.elapsed());
                let now = Instant::now();
                let result_b = func(true);
                info!("Part b took {:#?}", now.elapsed());
                match result_a {
                    Ok(solution_a) => {
                        info!("Solution {}a: {}", name, solution_a);
                    }
                    Err(err) => error!("Error occurred running {}: {}", name, err),
                }

                match result_b {
                    Ok(solution_b) => {
                        info!("Solution {}b: {}", name, solution_b);
                    }
                    Err(err) => error!("Error occurred running {}: {}", name, err),
                }
            }
        }
    }
}
