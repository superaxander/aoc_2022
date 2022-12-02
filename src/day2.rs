use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<i64> {
    let lines = common::read_lines("inputs/2.txt")?;
    let mut solution = 0;
    
    
    for line in lines {
        let line = line?;
        let mut split = line.trim().split(' ').collect::<Vec<&str>>();
        if do_b {
            split[1] = match (split[0], split[1]) {
                ("A", "X") => "Z",
                ("A", "Y") => "X",
                ("A", "Z") => "Y",
                ("B", "X") => "X",
                ("B", "Y") => "Y",
                ("B", "Z") => "Z",
                ("C", "X") => "Y",
                ("C", "Y") => "Z",
                ("C", "Z") => "X",
                _ => panic!()
            };
        }
        let shape_score = match split[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!()
        };
        let winner_score = match (split[0], split[1]) {
            ("A", "X") => 3,
            ("A", "Y") => 6,
            ("A", "Z") => 0,
            ("B", "X") => 0,
            ("B", "Y") => 3,
            ("B", "Z") => 6,
            ("C", "X") => 6,
            ("C", "Y") => 0,
            ("C", "Z") => 3,
            _ => panic!()
        };
        solution += winner_score + shape_score;
    }
    
    Ok(solution)
}
