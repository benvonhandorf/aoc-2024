use std::{fs::read_to_string, io::Error, ops::IndexMut, slice::RChunks};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct ErrInfo {
    all_increasing: bool,
    all_decreasing: bool,
    all_diffs_safe: bool,
}

fn read_lines(filename: &str) -> String {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
}

pub fn advent3a() {
    let data = read_lines("advent3a-input.txt");

    let pattern = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();

    let captures = pattern.captures_iter(data.as_str()).map(|c| c.extract());

    let result : i32 = captures.map( |(_, [lhs, rhs])| lhs.parse::<i32>().unwrap() * rhs.parse::<i32>().unwrap())
        .sum();
    
    println!("{}", result);
}

pub fn advent3b() {
    let data = read_lines("advent3a-input.txt");

    let pattern = Regex::new(r#"((?P<mul>mul)\((\d+),(\d+)\)|(?P<enable>do|don't)\(\))"#).unwrap();

    let captures = pattern.captures_iter(data.as_str());

    let mut result : i32 = 0;
    let mut enabled: bool = true;

    captures.for_each( |c| {
        if let Some(enable_instr) = c.name("enable") {
            match enable_instr.as_str() {
                "do" => {
                    enabled = true;
                },
                "don't" => {
                    enabled = false ;
                },
                _ => panic!("Unknown enable instruction")
            }
        } else {
            if enabled {
                let lhs = c.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let rhs = c.get(4).unwrap().as_str().parse::<i32>().unwrap();

                result += lhs * rhs;
            }
            
        }
    });

    println!("{}", result);
}