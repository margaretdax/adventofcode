use utils::*;

pub struct Solver;

impl Solution for Solver {
    fn get_number(self: &Self) -> i32 { 5 }

	fn solve(self: &Self, lines: Vec<String>) -> Option<String> {
		let mut solution = "".to_owned();
		for _line in lines.iter() {
			
		}
		solution.push_str(format!("Part I: {}\n", 0).as_str());

		for _line in lines.iter() {
			
		}
		solution.push_str(format!("Part II: {}\n", 0).as_str());
		
		None
	}
}
