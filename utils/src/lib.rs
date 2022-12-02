use std::io::{Lines};

pub trait Solution {
    fn solve(lines: Lines<String>) -> String;
}