use std::{fs::read_to_string, io::Error};
use itertools::Itertools;

#[derive(Debug)]
struct ErrInfo {
    all_increasing: bool,
    all_decreasing: bool,
    all_diffs_safe: bool,
}

fn read_lines(filename: &str) -> Vec<Vec<i32>> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|s| {
            let split = s.split(" ");

            split.map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn safe_levels(levels: &Vec<i32>) -> Result<(), ErrInfo> {
    let deltas : Vec<i32> = levels.windows(2).map(|w| (w[0] - w[1]) )
        .collect();

    let iter = deltas.into_iter();

    let all_increasing = iter.clone().all(|x| x > 0);
    let all_decreasing = iter.clone().all(|x| x < 0);
    let all_diffs_safe = iter.clone().all(|x| (x.abs() >= 1) && (x.abs() <= 3));

    if (all_increasing || all_decreasing) && all_diffs_safe {
        Ok(())
    } else {
        Result::Err(ErrInfo { all_increasing: all_increasing, all_decreasing: all_decreasing, all_diffs_safe: all_diffs_safe })
    }
}

fn safe_levels_problem_dampner(levels: &Vec<i32>) -> Result<(), ErrInfo> {
    let res = safe_levels(levels);

    if res.is_err() {
        for idx in 0..levels.len() {
            let mut mod_levels = levels.clone();
            mod_levels.remove(idx);

            if safe_levels(&mod_levels).is_ok() {
                println!("Levels saved by removing {idx}: {}", levels.into_iter().join(":"));
                return Ok(())
            }
        }

        println!("Levels could not be saved: {:?} - {}", res, levels.into_iter().join(":"));
    }

    return res
}

pub fn advent2a() {
    let input = read_lines("advent2a-input.txt");

    let safe_levels = input.into_iter()
                        .map(|x| safe_levels(&x))
                        .filter(|x| x.is_ok())
                        .count();

    println!("Safe levels: {}", safe_levels);
}

pub fn advent2b() {
    let input = read_lines("advent2a-input.txt");

    let safe_levels = input.into_iter()
                        .map(|x| safe_levels_problem_dampner(&x))
                        .filter(|x| x.is_ok())
                        .count();

    println!("Safe levels: {}", safe_levels);
}