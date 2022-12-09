use utils::*;
use std::{collections::HashSet};

pub struct Solver;

impl Solution for Solver {
    fn get_number(self: &Self) -> i32 { 9 }

	fn solve(self: &Self, lines: Vec<String>) -> Option<String> {
		let mut solution = "".to_owned();
		let mut answer;
		{ // Do Part I
			let mut visited: HashSet<(i32, i32)> = HashSet::new();
			let mut head = (0, 0);
			let mut tail = (0, 0);
			visited.insert(tail);
			for line in lines.iter() {
				let (instr, steps) = parse_instruction(line);
				for _ in 0..steps {
					(head, tail) = process(head, tail, instr);
					visited.insert(tail);
					//print_state(head, tail, &visited);
				}
			}
			answer = format!("{}", visited.len());
		}
		solution.push_str(format!("Part I: {}\n", answer).as_str());
		answer.clear();
		{ // Do Part II
			let mut rope = vec![];
			for _ in 0..10 {
				rope.push((0, 0));
			}
			let mut visited: HashSet<(i32, i32)> = HashSet::new();
			visited.insert((0, 0));
			for line in lines.iter() {
				let (instr, steps) = parse_instruction(line);
				for _ in 0..steps {
					let mut step_instr = instr;
					for i in 1..rope.len() {
						let l = rope[i - 1];
						let f = rope[i];
						//println!("moving knot {} at ({},{}) by ({},{})", i - 1, l.0, l.1, step_instr.0, step_instr.1);
						let (ll, ff) = process(l, f, step_instr);
						step_instr = (ff.0 - f.0, ff.1 - f.1);
						if step_instr.0 < 0 {
							step_instr.0 += 1;
						} else if step_instr.0 > 0 {
							step_instr.0 -= 1;
						}
						if step_instr.1 < 0 {
							step_instr.1 += 1;
						} else if step_instr.1 > 0 {
							step_instr.1 -= 1;
						}
						rope[i - 1] = ll;
						rope[i] = ff;
					}
					let last = rope.last().unwrap();
					visited.insert(*last);
					//print_vec_state(&rope, &visited);
				}
			}
			answer = format!("{}", visited.len());
		}
		solution.push_str(format!("Part II: {}\n", answer).as_str());
		
		Some(solution)
	}
}

#[allow(dead_code)]
fn print_state(head: (i32, i32), tail: (i32, i32), visited: &HashSet<(i32, i32)>) {
	let width = 5;
	let height = 5;
	for y in 0..height {
		for x in 0..width {
			let p = (x, y);
			if head == p && tail == p {
				print!("B"); // both
			} else if head == p {
				print!("H");
			} else if tail == p {
				print!("T");
			} else if visited.contains(&p) {
				print!("#");
			} else {
				print!(".");
			}
		}
		print!("\n");
	}
	print!("\n");
}

#[allow(dead_code)]
fn print_vec_state(rope: &Vec<(i32, i32)>, visited: &HashSet<(i32, i32)>) {
	let width = 30;
	let height = 30;
	for y in 0..height {
		for x in 0..width {
			let p = (x, height - y - 1);
			let mut did_print = false;
			for i in 0..rope.len() {
				if rope[i] == p {
					if i == 0 {
						print!("H");
					} else {
						print!("{}", i);
					}
					did_print = true;
					break;
				}
			}

			if !did_print {
				if visited.contains(&p) {
					print!("#");
				} else {
					print!(".");
				}
			}
		}
		print!("\n");
	}
	print!("\n");
}

fn process(head: (i32, i32), tail: (i32, i32), instr: (i32, i32)) -> ((i32, i32), (i32, i32)) {
	let h = (head.0 + instr.0, head.1 + instr.1);
	let dx = h.0 - tail.0;
	let dy = h.1 - tail.1;

	match (dx, dy) {
		(2, 2) => (h, (tail.0 + 1, tail.1 + 1)),
		(-2, 2) => (h, (tail.0 - 1, tail.1 + 1)),
		(2, -2) => (h, (tail.0 + 1, tail.1 - 1)),
		(-2, -2) => (h, (tail.0 - 1, tail.1 - 1)),
		(2, _) => (h, (tail.0 + 1, h.1)),
		(-2, _) => (h, (tail.0 - 1, h.1)),
		(_, 2) => (h, (h.0, tail.1 + 1)),
		(_, -2) => (h, (h.0, tail.1 - 1)),
		_ => (h, tail)
	}
}

fn parse_instruction(line: &String) -> ((i32, i32), usize) {
	let mut iter = line.split_whitespace();
	let dir = match iter.next().unwrap() {
		"R" => (1, 0),
		"L" => (-1, 0),
		"U" => (0, 1),
		"D" => (0, -1),
		_ => panic!("panik")
	};
	let steps = iter.next().unwrap().parse::<usize>().unwrap();
	(dir, steps)
}