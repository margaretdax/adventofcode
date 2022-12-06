use utils::*;

pub struct Solver;

impl Solution for Solver {
    fn get_number(self: &Self) -> i32 { 6 }

	fn solve(self: &Self, lines: Vec<String>) -> Option<String> {
		let mut solution = "".to_owned();
		let mut answer = "".to_owned();
		for line in lines.iter() {
			let marker = find_marker(line);
			answer.push_str(format!("{}, ", marker).as_str());
		}
		solution.push_str(format!("Part I: {}\n", answer).as_str());

		for _line in lines.iter() {
			
		}
		solution.push_str(format!("Part II: {}\n", 0).as_str());
		
		Some(solution)
	}
}

fn find_marker(s: &String) -> usize {
	let end = s.len() - 14;
	for i in 0..=end {
		let sub = &s[i..i+14];
		if has_dup(sub) {
			return i+14
		}		
	}
	
	0
}

fn has_dup(s: &str) -> bool {
	let bytes = s.as_bytes();
	for i in 0..s.len() {
		for j in i+1..s.len() {
			if bytes[i] == bytes[j] {
				return false
			}
		}
	}
	return true
}