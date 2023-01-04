pub trait Solution {
    fn get_year(&self) -> i32 { 2022 }
    fn get_number(&self) -> i32;
    fn solve(&self, lines: Vec<String>) -> Option<String>;
}