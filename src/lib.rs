#[macro_use] extern crate lazy_static;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod expense_report;
pub mod password_validator;
pub mod toboggan_trajectory;
pub mod passport_processing;
pub mod binary_boarding;
pub mod customs;
pub mod handy_haversack;
pub mod handheld;
pub mod encoding_errors;
pub mod adapter_array;
pub mod seating_system;
pub mod rain_risk;
pub mod shuttle_schedule;
pub mod docking_data;




pub fn string_to_intvec(inp:&str, delimeter:&str) -> Vec<i64> {
    let mut result = Vec::new();
    for token in inp.split(delimeter) {
        if token.len()>0 {
            result.push(token.trim().parse::<i64>().unwrap())
        }
    }
    return result;
}
pub fn string_to_uintvec(inp:&str, delimeter:&str) -> Vec<u64> {
    let mut result = Vec::new();
    for token in inp.split(delimeter) {
        if token.len()>0 {
            result.push(token.trim().parse::<u64>().unwrap())
        }
    }
    return result;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}