use utils::*;

pub struct Solver;

enum Instruction {
	Noop,
	Add(i32)
}

struct Program {
	data: Vec<Instruction>,
	x: i32,
	cycle: usize
}

impl Iterator for Program {
	type Item = (i32, usize);
	fn next(&mut self) -> Option<Self::Item> {
		if self.cycle >= self.data.len() {
			return None
		}
		let result = (self.x, self.cycle + 1);
		if let Instruction::Add(dx) = self.data[self.cycle] {
			self.x += dx;
		}
		self.cycle += 1;
		Some(result)
	}
}

impl Solution for Solver {
    fn get_number(self: &Self) -> i32 { 10 }

	fn solve(self: &Self, lines: Vec<String>) -> Option<String> {
		let mut solution = "".to_owned();
		let mut answer;
		{ // Do Part I
			let mut sum: i32 = 0;
			let program = load_program(&lines);
			for eval in program.into_iter() {
				if (eval.1 + 20) % 40 == 0 {
					let score: i32 = eval.0 * (eval.1 as i32);
					sum += score;
				}
			}
			answer = format!("{}", sum);
		}
		solution.push_str(format!("Part I: {}\n", answer).as_str());
		answer.clear();
		{ // Do Part II
			let program = load_program(&lines);
			let frames = program.into_iter();
			let mut build = "".to_string();
			answer.push('\n');
			for (x, _) in frames {
				let mask = sprite_mask(x);
				let i = build.len();
				let c = &mask[i..i+1];
				build.push_str(c);
				//println!("Cycle: {}, X: {} Mask:\t{} {}", cycle, x, mask, build);

				if build.len() == 40 {
					answer.push_str(build.as_str());
					answer.push('\n');
					build.clear();
				}
			}
			// let pixels = frames.map(|(x, cycle)| {
			// 	let i = (cycle % 40) as i32;
			// 	let c = if (x - i).abs() < 2 { "#" } else { "." };
			// 	if i == 1 { format!("\n{}", c) } else { c.to_owned() } });
			// for pixel in pixels {
			// 	answer.push_str(pixel.as_str());
			// }
		}
		solution.push_str(format!("Part II: {}\n", answer).as_str());
		
		Some(solution)
	}
}

fn sprite_mask(x: i32) -> String {
	let mut result = "".to_owned();
	for i in 0..40 {
		result.push_str(if (x - i).abs() < 2 { "#" } else { "." });
	}
	result
}

fn load_program(lines: &Vec<String>) -> Program {
	let mut data = vec![];

	for line in lines.iter() {
		if line == "noop" {
			data.push(Instruction::Noop);
		} else {
			let x = line.split_whitespace().last().unwrap().parse::<i32>().unwrap();
			data.push(Instruction::Noop);
			data.push(Instruction::Add(x));
		}
	}

	Program { data: data, x: 1, cycle: 0 }
}