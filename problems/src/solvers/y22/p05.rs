use std::fs::read;

use utils::*;

pub struct Solver;

impl Solution for Solver {
    fn get_number(self: &Self) -> i32 { 5 }

	fn solve(self: &Self, lines: Vec<String>) -> Option<String> {
		let mut solution = "".to_owned();
		let mut stack_tops = "".to_owned();
		{
			let (mut stacks, begin) = parse_stacks(&lines);
			let mut it = lines.iter();
			for _ in 0..begin { it.next(); }

			for line in it {
				let (a, b, c) = parse_move(line);
				//println!("{} {} {} {}", line, a, b, c);
				make_move(&mut stacks, a, b-1, c-1);
			}
			stack_tops = String::from_utf8(read_stacks(&stacks)).unwrap();
		}
		solution.push_str(format!("Part I: {}\n", stack_tops).as_str());

		stack_tops.clear();
		for line in lines.iter() {

		}
		solution.push_str(format!("Part II: {}\n", stack_tops).as_str());
		
		Some(solution)
	}
}

fn parse_move(line: &String) -> (usize, usize, usize) {
	let mut parts = line.split(' ');
	parts.next();
	let a = parts.next().unwrap().parse::<usize>().unwrap();
	parts.next();
	let b = parts.next().unwrap().parse::<usize>().unwrap();
	parts.next();
	let c = parts.next().unwrap().parse::<usize>().unwrap();
	(a, b, c)
}

fn make_move(stacks: &mut Vec<Vec<u8>>, n: usize, from: usize, to: usize) {
	for i in 0..n {
		let top = stacks[from].pop().unwrap();
		stacks[to].push(top);
	}
}

fn read_stacks(stacks: &Vec<Vec<u8>>) -> Vec<u8> {
	stacks.iter().cloned().map(|s| if s.is_empty() { b' ' } else { s.last().unwrap().clone() }).collect()
}

fn parse_stacks(lines: &Vec<String>) -> (Vec<Vec<u8>>, usize) {
	let num = (lines.first().unwrap().len() / 4) + 1;
	let mut result: Vec<Vec<u8>> = vec![];
	for i in 0..num {
		result.push(vec![]);
	}
	let mut n = 0;
	for line in lines.iter() {
		let li = line.as_bytes();
		for i in 0..num {
			let b = li[i * 4 + 1];
			if b >= b'0' && b <= b'9' {
				return (result, n + 2);
			}
			if b != b' ' {
				result[i].insert(0, b);
			}
		}
		n += 1;
	}
	panic!("didn't find the bottom")
}