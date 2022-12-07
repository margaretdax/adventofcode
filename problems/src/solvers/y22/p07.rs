use utils::*;

pub struct Solver;

impl Solution for Solver {
    fn get_number(self: &Self) -> i32 { 7 }

	fn solve(self: &Self, lines: Vec<String>) -> Option<String> {
		let mut solution = "".to_owned();
		let mut answer = "".to_owned();
		for _line in lines.iter() {

		}
		solution.push_str(format!("Part I: {}\n", answer).as_str());

		answer.clear();
		for _line in lines.iter() {

		}
		solution.push_str(format!("Part II: {}\n", answer).as_str());
		
		//Some(solution)
		None
	}
}