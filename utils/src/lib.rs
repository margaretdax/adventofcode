pub trait Solution {
    fn get_year(self: &Self) -> i32 { 2022 }
    fn get_number(self: &Self) -> i32;
    fn solve(self: &Self, lines: Vec<String>) -> Option<String>;
}