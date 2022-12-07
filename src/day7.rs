use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use anyhow::Result;

use crate::common;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Directory {
    name: String,
    children: Vec<Rc<RefCell<Directory>>>,
    files: HashMap<String, usize>,
    size: usize,
}

impl Directory {
    fn calculate_size(&mut self) {
        for d in &mut self.children {
            d.borrow_mut().calculate_size();
        }
        self.size = self.files.values().sum::<usize>() + self.children.iter().map(|d|d.borrow().size).sum::<usize>();
    }
    
    fn get_directory_sizes(&self, directories: &mut Vec<usize>) {
        directories.push(self.size);
        for d in &self.children {
            d.borrow().get_directory_sizes(directories);
        }
    } 
}

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/7.txt")?;

    let mut current_directory = Rc::new(RefCell::new(Directory {name: "/".to_owned(), children: Vec::new(), files: HashMap::new(), size: 0}));
    let mut directory_stack = vec![];

    for line in lines {
        let line = line?;
        if line.starts_with('$') {
            if let Some(("cd", argument)) = line[2..].split_once(' ') {
                match argument {
                    "/" => {
                        while !directory_stack.is_empty() {
                            current_directory = directory_stack.pop().unwrap();
                        }
                    }
                    ".." => {
                        current_directory = directory_stack.pop().unwrap();
                    },
                    _ => {
                        let tmp = current_directory.borrow().children.iter().find(|d|d.borrow().name == argument).map_or_else(|| {
                            let mut borrowed = current_directory.borrow_mut();
                            borrowed.children.push(Rc::new(RefCell::new(Directory {
                                name: argument.to_owned(),
                                children: Vec::new(),
                                files: HashMap::new(),
                                size: 0,
                            })));
                            let size = borrowed.children.len() - 1;
                            borrowed.children[size].clone()
                        }, Rc::clone);
                        directory_stack.push(current_directory.clone());
                        current_directory = tmp;
                    }
                }
            }
        } else {
            // Is ls output
            let (size, name) = line.split_once(' ').expect("ls output lines need to consist of two parts");
            if size == "dir" {
                let borrowed = &mut current_directory.borrow_mut().children;
                if !borrowed.iter().any(|d|d.borrow().name == name) {
                    borrowed.push(Rc::new(RefCell::new(Directory {
                        name: name.to_owned(),
                        children: Vec::new(),
                        files: HashMap::new(),
                        size: 0
                    })));
                }
            } else {
                current_directory.borrow_mut().files.insert(name.to_owned(), size.parse()?);
            }
        }
    }

    while !directory_stack.is_empty() {
        current_directory = directory_stack.pop().unwrap();
    }
    current_directory.borrow_mut().calculate_size();
    let mut directories = Vec::new();
    current_directory.borrow().get_directory_sizes(&mut directories);
    let solution_a = directories.iter().filter(|n|**n <= 100000).sum(); 
    
    let remaining_space = 70000000 - current_directory.borrow().size;
    let to_delete = 30000000 - remaining_space;
    let solution_b = *directories.iter().filter(|n|**n >= to_delete).min().unwrap();

    Ok((solution_a, solution_b))
}
