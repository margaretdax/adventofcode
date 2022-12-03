pub trait Solution {
    fn solve(self: Box<Self>, lines: Vec<String>) -> String;
}