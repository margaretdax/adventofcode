use utils::*;

pub struct Solver;

impl Solution for Solver {
    fn get_number(self: &Self) -> i32 { 9 }

	fn solve(self: &Self, _lines: Vec<String>) -> Option<String> {
		let mut solution = "".to_owned();
		let mut answer = "".to_owned();
		{ // Do Part I
		
		}
		solution.push_str(format!("Part I: {}\n", answer).as_str());
		answer.clear();
		{ // Do Part II
			
		}
		solution.push_str(format!("Part II: {}\n", answer).as_str());
		
		//Some(solution)
		None
	}
}